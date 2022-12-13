extern crate proc_macro;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, FnArg, Item, ItemFn, LitStr, Pat, PatType, Path, ReturnType,
    Stmt, Type,
};

#[derive(Debug, FromMeta)]
struct Options {
    #[darling(default)]
    pub trace: Opt,
    #[darling(default)]
    pub time: Opt,
    #[darling(default)]
    pub cache: Opt,
    #[darling(default)]
    pub count: Opt,
}

#[derive(Debug, Default, FromMeta, PartialEq, Eq)]
enum Opt {
    Force,
    #[default]
    Default,
    Forbid,
}

impl Opt {
    pub fn is_enabled(&self, cfg: bool) -> bool {
        self == &Opt::Force || (self == &Opt::Default && cfg)
    }
}

fn split_args(def: &mut ItemFn) -> Vec<(Pat, Ident, Type)> {
    let mut args = Vec::new();

    for (i, arg) in def.sig.inputs.iter_mut().enumerate() {
        let numbered = Ident::new(&format!("_arg{i}"), Span::call_site());
        match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                args.push((*pat.clone(), numbered.clone(), *ty.clone()));
                *pat = parse_quote!(mut #numbered);
            }
            FnArg::Receiver(_) => {
                todo!()
            }
        }
    }
    args
}

/// # Panics
/// When no path is provided
#[proc_macro]
pub fn count(func: TokenStream) -> TokenStream {
    let (path, fn_id) = parse_path(func).unwrap();
    let id = Ident::new(&format!("__MONETA_FN_COUNT_{fn_id}"), Span::call_site());
    TokenStream::from(quote! { unsafe { #(#path::)* #id } })
}

/// # Panics
/// When no path is provided
#[proc_macro]
pub fn get_cache(func: TokenStream) -> TokenStream {
    let (path, fn_id) = parse_path(func).unwrap();
    let id = Ident::new(&format!("__MONETA_FN_CACHE_{fn_id}"), Span::call_site());
    TokenStream::from(quote! { #(#path::)* #id })
}

/// # Panics
/// When no path is provided
fn parse_path(input: TokenStream) -> syn::Result<(Vec<Ident>, Ident)> {
    let mut input: Vec<_> = syn::parse::<Path>(input)?
        .segments
        .into_iter()
        .map(|seg| seg.ident)
        .collect();
    let fn_id = input.pop().unwrap();
    Ok((input, fn_id))
}

#[proc_macro_attribute]
pub fn moneta(meta: TokenStream, input: TokenStream) -> TokenStream {
    let mut outter = parse_macro_input!(input as ItemFn);
    let mut def_fn = outter.clone();
    let args: Vec<_> = split_args(&mut outter);

    let options = parse_macro_input!(meta as syn::AttributeArgs);
    let options = match Options::from_list(&options) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let cache_id = Ident::new(
        &format!("__MONETA_FN_CACHE_{}", def_fn.sig.ident),
        Span::call_site(),
    );

    let counter_id = Ident::new(
        &format!("__MONETA_FN_COUNT_{}", def_fn.sig.ident),
        Span::call_site(),
    );

    let ret_ty = match def_fn.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ref ty) => {
            let ty = ty.clone();
            quote! { #ty }
        }
    };

    let args_lit_name: Vec<_> = args
        .iter()
        .map(|(name, _, _)| {
            LitStr::new(
                if let Pat::Ident(ident) = name {
                    let id = TokenStream::from(quote! { #ident });
                    syn::parse::<Ident>(id).ok().map(|id| id.to_string())
                } else {
                    None
                }
                .unwrap_or_else(|| "pat".to_string())
                .as_ref(),
                Span::call_site(),
            )
        })
        .collect();
    let name = outter.sig.ident.clone();
    let out_args: Vec<_> = args.iter().map(|(_, arg, _)| arg).collect();
    let def_args: Vec<_> = args.iter().map(|(name, _, _)| name).collect();
    let func_name = LitStr::new(&format!("{name}"), Span::call_site());
    let res_id = Ident::new("res", Span::call_site());
    let start_id = Ident::new("start", Span::call_site());

    let get_ret = if options.cache.is_enabled(cfg!(feature = "cache")) {
        quote! {
            let #res_id = #name (#(#out_args),*);
        }
    } else {
        quote! { let #res_id = #name (#(#out_args),*); }
    };

    let (trace_in, trace_out) = trace(
        &options.trace,
        &options.time,
        &func_name,
        &start_id,
        args_lit_name.iter(),
        def_args.iter().copied(),
    );
    let cache_def = cache_def(&cache_id, &ret_ty);
    let (cache_get, cache_set) = cache(&options.cache, &def_args, &out_args, &cache_id, &res_id);
    let (counter_def, counter_inc) = counter(&options.count, &counter_id);

    let pre_injection = quote! {{
        #counter_inc
        #trace_in
        #cache_get
    }}
    .into();

    let post_injection = quote! {{
        let #start_id = std::time::Instant::now();
        #get_ret
        #trace_out
        #cache_set
        return #res_id;
    }}
    .into();

    def_fn
        .block
        .stmts
        .insert(0, parse_macro_input!(pre_injection as Stmt));

    outter.block.stmts = vec![
        Stmt::Item(Item::Fn(def_fn)),
        parse_macro_input!(post_injection as Stmt),
    ];

    let code = quote! {
        #counter_def
        #cache_def
        #outter
    };
    TokenStream::from(code)
}

fn cache_def(cache_id: &Ident, cache_ret: &TokenStream2) -> TokenStream2 {
    quote! {
        #[allow(non_upper_snake_case)]
        lazy_static::lazy_static! {
            pub static ref #cache_id: std::sync::RwLock<hashbrown::HashMap<String, #cache_ret>> =
                std::sync::RwLock::new(hashbrown::HashMap::new());
        }
    }
}

fn trace<'a>(
    trace_enabled: &Opt,
    time_enabled: &Opt,
    name_str: &LitStr,
    start_id: &Ident,
    args_names: impl Iterator<Item = &'a LitStr>,
    def_args: impl Iterator<Item = &'a Pat>,
) -> (TokenStream2, TokenStream2) {
    let trace_enabled = trace_enabled.is_enabled(cfg!(feature = "trace"));

    let in_trace = if trace_enabled {
        quote! {{
            let args_fmt: String = [
                #(#args_names,)*
            ].into_iter()
                .zip([#(format!("{:?}", #def_args),)*].into_iter())
                .map(|(n, v): (&str, String)| format!("\n\t{}: {}", n, v))
                .collect();
            println!("in {}: {}", #name_str, args_fmt);
        }}
    } else {
        quote! { ; }
    };

    let out_trace = if time_enabled.is_enabled(cfg!(feature = "time")) {
        quote! {
            println!("out {}: {:?}", #name_str, #start_id.elapsed());
        }
    } else if trace_enabled {
        quote! {
            println!("out {}", #name_str);
        }
    } else {
        quote! { ; }
    };

    (in_trace, out_trace)
}

fn cache(
    enabled: &Opt,
    def_args: &Vec<&Pat>,
    out_args: &Vec<&Ident>,
    counter_id: &Ident,
    res_id: &Ident,
) -> (TokenStream2, TokenStream2) {
    let debug_fmt = LitStr::new(&"{:?}".repeat(def_args.len()), Span::call_site());
    let enabled = enabled.is_enabled(cfg!(feature = "cache"));

    let get_cache = if enabled {
        quote! {{
            let values_fmt = format!(#debug_fmt, #(#def_args),*);
            if let Ok(reader) = #counter_id.read() {
                if let Some(val) = reader.get(&values_fmt) {
                    return val.clone();
                }
            }
        }}
    } else {
        quote! { ; }
    };

    let set_cache = if enabled {
        quote! {{
            let values_fmt = format!(#debug_fmt, #(#out_args),*);
            if let Ok(mut writer) = #counter_id.write() {
                writer.entry(values_fmt).or_insert(#res_id.clone());
            }
        }}
    } else {
        quote! {{ ; }}
    };
    (get_cache, set_cache)
}

fn counter(enabled: &Opt, counter_id: &Ident) -> (TokenStream2, TokenStream2) {
    let def = quote! {
        #[allow(non_upper_snake_case)]
        pub static mut #counter_id: usize = 0;
    };

    let inc = if enabled.is_enabled(cfg!(feature = "count")) {
        quote! { unsafe { #counter_id += 1 }; }
    } else {
        quote! { ; }
    };

    (def, inc)
}
