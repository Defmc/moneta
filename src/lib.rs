extern crate self as moneta_fn;

pub use ::hashbrown;
pub use ::lazy_static;
pub use moneta_fn_macros::{count, get_cache, moneta};

#[cfg(test)]
mod tests;
