extern crate self as moneta_fn;

use std::cell::RefCell;

pub use ::hashbrown;
pub use ::once_cell;
pub use moneta_fn_macros::{count, get_cache, get_counter, moneta, reset_count};

thread_local! {
    pub static DEPTH: RefCell<usize> = RefCell::new(0);
}

#[cfg(test)]
mod tests;
