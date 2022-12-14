## A set of macros to function profiling
Contains some convenient macros to analyze function execution, like `count` to know how many times a function was called, `trace` to log when entering/exiting from a function and what are its arguments, and `cache` for pure functions cache.


## Features
| Feature | Description | Additional dependencies |
| ------- | ----------- | ----------------------- |
| `count` | Makes available a macro `count!` to get how many times a function was called | None |
| `trace` | Prints when entering/exiting in a tagged function and its arguments | None |
| `cache` | Implements memoization for a tagged function | `lazy_static` and `hashbrown` |
| `visible` | Changes the cache storage and counter visibility | None |

Each feature can be forbided, forced or setted to global config (setted by features) in the attribute declaration. E.g:
```rust 
#[moneta_fn::moneta(cache = "forbid", visible = "forbid", count = "force")]
fn foo(a: u8) -> u8 {
    unimplemented!()
}
```

## Type dependencies
When enabled in a function, `cache` requires that all parameters implement the `Debug` trait and `Clone` for the return type.

## Wait... Why `Debug` for parameters and not `Clone`?
Consider these scenarios:
```rust
#[moneta_fn::moneta]
fn foo<'a, T>(lhs: &'a T, rhs: &'a T) -> T {
    unimplemented!()
}
```
It's hard to create cache storage with generic keys and lifetimes, once it can't be parsed in an equivalent format for a global `RwLock`.

Furthermore, it allows some optimizations for liked types:
```rust
#[moneta_fn::moneta]
fn foo<T: AsRef<str> + Debug>(lhs: T, rhs: T) -> T {
    unimplemented!()
}
```

## I don't want `trace`/`count`/`cache` in my release builds. How do I disable it?
There isn't ~yet~ a good way to enable/disable specific features in different profiles. You'll need to set `default-features` to `false` and define which features you want. E.g:
```toml 
[dependencies]
moneta_fn = { version = "*", default-features = false, features = ["cache", "count", "time"] }
```

Another way to do it, is using a setting a default feature.
```toml 
[dependencies]
moneta_fn = { version = "*", default-features = false, features = ["cache", "count", "time"] }

[features]
default = ["debug_mode"]
debug_mode = ["moneta_fn/trace"]
```

And when compiling with a profile relase, use `--no-default-features`
```bash
cargo build --release --no-default-features
```

## How do I enable/disable `trace`/`count`/`cache` for just one function?
Set `trace`/`count`/`cache` as `force`/`forbid`
```rust
#[moneta_fn::moneta(cache = "forbid")] // Will not update cache storage
fn foo(a: u8) -> u8 {
    unimplemented!()
}
```

## Will `count!`/`get_cache!` calls break when the `count`/`cache` feature is disabled?
No. The counting or cache will just not be updated.
