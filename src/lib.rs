extern crate self as moneta_fn;

pub use ::hashbrown;
pub use ::lazy_static;
pub use macros::{count, get_cache, moneta};

#[cfg(test)]
mod tests;
