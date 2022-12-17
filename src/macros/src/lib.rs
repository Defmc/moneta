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
    #[darling(default)]
    pub visible: Opt,
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
        self == &Self::Force || (self == &Self::Default && cfg)
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
pub fn get_cache(func: TokenStream) -> TokenStream {
    let (path, fn_id) = parse_path(func).unwrap();
    let id = Ident::new(&format!("__MONETA_FN_CACHE_{fn_id}"), Span::call_site());
    quote! { #(#path::)* #id }.into()
}

/// # Panics
/// When no path is provided
#[proc_macro]
pub fn count(func: TokenStream) -> TokenStream {
    let counter = get_counter(func);
    let counter = parse_macro_input!(counter as Path);
    quote! { #counter .load(std::sync::atomic::Ordering::SeqCst) }.into()
}

/// # Panics
/// When no path is provided
#[proc_macro]
pub fn reset_count(func: TokenStream) -> TokenStream {
    let counter = get_counter(func);
    let counter = parse_macro_input!(counter as Path);
    quote! { #counter .store(0, std::sync::atomic::Ordering::SeqCst) }.into()
}

/// # Panics
/// When no path is provided
#[proc_macro]
pub fn get_counter(func: TokenStream) -> TokenStream {
    let (path, fn_id) = parse_path(func).unwrap();
    let id = Ident::new(&format!("__MONETA_FN_COUNT_{fn_id}"), Span::call_site());
    quote! { #(#path::)* #id }.into()
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
    outter.sig.constness = None;
    let fn_name = def_fn.sig.ident;
    def_fn.sig.ident = Ident::new("__MONETA_FN_WRAPPER", Span::call_site());

    let args: Vec<_> = split_args(&mut outter);
    let options = parse_macro_input!(meta as syn::AttributeArgs);
    let options = match Options::from_list(&options) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let vis = &def_fn.vis;
    let vis = if options.visible.is_enabled(cfg!(feature = "visible")) {
        quote! { pub }
    } else {
        quote! { #vis }
    };

    let cache_id = Ident::new(&format!("__MONETA_FN_CACHE_{fn_name}"), Span::call_site());
    let counter_id = Ident::new(&format!("__MONETA_FN_COUNT_{fn_name}"), Span::call_site());

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

    let wrapper_name = def_fn.sig.ident.clone();
    let out_args: Vec<_> = args.iter().map(|(_, arg, _)| arg).collect();
    let func_name = LitStr::new(&format!("{}", outter.sig.ident), Span::call_site());
    let res_id = Ident::new("res", Span::call_site());
    let start_id = Ident::new("start", Span::call_site());
    let values_id = Ident::new("values_fmt", Span::call_site());
    let depth_id = Ident::new("depth", Span::call_site());
    let prefix_id = Ident::new("prefix", Span::call_site());

    let get_ret = if options.cache.is_enabled(cfg!(feature = "cache")) {
        quote! { let #res_id = #wrapper_name (#(#out_args),*); }
    } else {
        quote! { let #res_id = #wrapper_name (#(#out_args),*); }
    };

    let trace_in = trace_in(
        &options.trace,
        &func_name,
        &depth_id,
        &prefix_id,
        args_lit_name.iter(),
        out_args.iter().copied(),
    );
    let trace_out = trace_out(
        (&options.trace, &options.time),
        &func_name,
        &start_id,
        &prefix_id,
        &res_id,
    );
    let (depth_def, prefix_def) = trace_prefixes(&options.trace, &depth_id, &prefix_id);
    let let_values_fmt = let_values_fmt(&values_id, &out_args);
    let cache_def = cache_def(&cache_id, &vis, &ret_ty);
    let (cache_get, cache_set) = cache(&options.cache, &values_id, &cache_id, &res_id);
    let (counter_def, counter_inc) = counter(&options.count, &vis, &counter_id);

    let post_injection = quote! {{
        #counter_inc
        #let_values_fmt
        #depth_def
        #prefix_def
        #trace_in
        #cache_get
        let #start_id = std::time::Instant::now();
        #get_ret
        #trace_out
        #cache_set
        return #res_id;
    }}
    .into();

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

fn cache_def(cache_id: &Ident, vis: &TokenStream2, cache_ret: &TokenStream2) -> TokenStream2 {
    quote! {
        #[allow(non_upper_snake_case)]
        #vis static #cache_id: ::moneta_fn::once_cell::sync::Lazy<std::sync::RwLock<::moneta_fn::hashbrown::HashMap<String, #cache_ret>>> =
            ::moneta_fn::once_cell::sync::Lazy::new(|| std::sync::RwLock::new(::moneta_fn::hashbrown::HashMap::new()));
    }
}

fn trace_in<'a>(
    trace: &Opt,
    name_str: &LitStr,
    depth_id: &Ident,
    prefix_id: &Ident,
    args_names: impl Iterator<Item = &'a LitStr>,
    out_args: impl Iterator<Item = &'a Ident>,
) -> TokenStream2 {
    let trace_enabled = trace.is_enabled(cfg!(feature = "trace"));
    if trace_enabled {
        quote! {{
            ::moneta_fn::DEPTH.with(|d| *d.borrow_mut() += 1);
            let args_fmt: String = [
                #(#args_names,)*
            ].into_iter()
                .zip([#(format!("{:?}", #out_args),)*].into_iter())
                .map(|(n, v): (&str, String)| format!("\n{}    {}: {}", #depth_id, n, v))
                .collect();
            println!("{}in {}: {}",
                     #prefix_id,
                     #name_str,
                     args_fmt
            );
        }}
    } else {
        quote! { ; }
    }
}

fn trace_out(
    (trace, time): (&Opt, &Opt),
    name_str: &LitStr,
    start_id: &Ident,
    prefix_id: &Ident,
    res_id: &Ident,
) -> TokenStream2 {
    let trace_enabled = trace.is_enabled(cfg!(feature = "trace"));
    let sub_depth = if trace_enabled {
        quote! { ::moneta_fn::DEPTH.with(|d| { *d.borrow_mut() -= 1; }); }
    } else {
        quote! { ; }
    };

    if time.is_enabled(cfg!(feature = "time")) {
        quote! {{
            #sub_depth
            println!("{}out {} ({:?}): {:?}",
                     #prefix_id,
                     #name_str,
                     #start_id.elapsed(),
                     #res_id);
        }}
    } else if trace_enabled {
        quote! {{
            #sub_depth
            println!("{}out {}: {:?}",
                     #prefix_id,
                     #name_str,
                     #res_id);
        }}
    } else {
        quote! { ; }
    }
}

fn trace_prefixes(
    trace_enabled: &Opt,
    depth_id: &Ident,
    prefix_id: &Ident,
) -> (TokenStream2, TokenStream2) {
    if trace_enabled.is_enabled(cfg!(feature = "trace")) {
        let depth = quote! {
            let #depth_id = ::moneta_fn::DEPTH.with(|d| "    ".repeat(*d.borrow()));
        };
        let prefix = quote! {
            let #prefix_id = format!("{}[{:?}{}] ", depth,
                     std::thread::current().id(),
                     std::thread::current().name().map_or_else(|| String::new(), |n| format!(":{}", n)));
        };
        (depth, prefix)
    } else {
        (quote! { ; }, quote! { let #prefix_id = String::new(); })
    }
}

fn cache(
    enabled: &Opt,
    values_fmt: &Ident,
    counter_id: &Ident,
    res_id: &Ident,
) -> (TokenStream2, TokenStream2) {
    let enabled = enabled.is_enabled(cfg!(feature = "cache"));

    let get_cache = if enabled {
        quote! {{
            if let Ok(reader) = #counter_id.read() {
                if let Some(val) = reader.get(&#values_fmt) {
                    return val.clone();
                }
            }
        }}
    } else {
        quote! { ; }
    };

    let set_cache = if enabled {
        quote! {{
            if let Ok(mut writer) = #counter_id.write() {
                writer.entry(#values_fmt).or_insert(#res_id.clone());
            }
        }}
    } else {
        quote! {{ ; }}
    };
    (get_cache, set_cache)
}

fn counter(enabled: &Opt, vis: &TokenStream2, counter_id: &Ident) -> (TokenStream2, TokenStream2) {
    let def = quote! {
        #[allow(non_upper_snake_case)]
        #vis static #counter_id: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    };

    let inc = if enabled.is_enabled(cfg!(feature = "count")) {
        quote! { #counter_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst); }
    } else {
        quote! { ; }
    };

    (def, inc)
}

fn let_values_fmt(id: &Ident, args: &Vec<&Ident>) -> TokenStream2 {
    let debug_fmt = LitStr::new(&"{:?}".repeat(args.len()), Span::call_site());
    quote! {
        let #id = format!(#debug_fmt, #(#args),*);
    }
}
