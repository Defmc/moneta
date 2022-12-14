#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use hashbrown::HashMap;
pub use lazy_static::lazy_static;
pub use macros::{count, get_cache, moneta};
extern crate self as moneta_fn;
#[allow(non_upper_snake_case)]
pub static __MONETA_FN_COUNT_fibonacci: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(
    0,
);
#[must_use]
pub fn fibonacci(mut _arg0: u128) -> u128 {
    #[must_use]
    pub fn __MONETA_FN_WRAPPER(n: u128) -> u128 {
        match n {
            0 | 1 => 1,
            _ => (1..=2).map(|o| fibonacci(n - o)).sum(),
        }
    }
    {
        __MONETA_FN_COUNT_fibonacci.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let values_fmt = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_debug(&_arg0)],
                ),
            );
            res
        };
        {
            let args_fmt: String = ["n"]
                .into_iter()
                .zip(
                    [
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_debug(&_arg0)],
                                ),
                            );
                            res
                        },
                    ]
                        .into_iter(),
                )
                .map(|(n, v): (&str, String)| {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["\n\t", ": "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&n),
                                ::core::fmt::ArgumentV1::new_display(&v),
                            ],
                        ),
                    );
                    res
                })
                .collect();
            {
                ::std::io::_print(
                    ::core::fmt::Arguments::new_v1(
                        &["in ", ": ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"fibonacci"),
                            ::core::fmt::ArgumentV1::new_display(&args_fmt),
                        ],
                    ),
                );
            };
        }
        {
            if let Ok(reader) = __MONETA_FN_CACHE_fibonacci.read() {
                if let Some(val) = reader.get(&values_fmt) {
                    return val.clone();
                }
            }
        }
        let start = std::time::Instant::now();
        let res = __MONETA_FN_WRAPPER(_arg0);
        {
            ::std::io::_print(
                ::core::fmt::Arguments::new_v1(
                    &["out ", ": ", "\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&"fibonacci"),
                        ::core::fmt::ArgumentV1::new_debug(&start.elapsed()),
                    ],
                ),
            );
        };
        {
            if let Ok(mut writer) = __MONETA_FN_CACHE_fibonacci.write() {
                writer.entry(values_fmt).or_insert(res.clone());
            }
        }
        return res;
    }
}
#[allow(non_upper_snake_case)]
pub static __MONETA_FN_COUNT_tribonacci: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(
    0,
);
#[must_use]
pub fn tribonacci(mut _arg0: u128) -> u128 {
    #[must_use]
    pub fn __MONETA_FN_WRAPPER(n: u128) -> u128 {
        match n {
            0 => 0,
            1 | 2 => 1,
            3 => 2,
            _ => (1..=3).map(|o| tribonacci(n - o)).sum(),
        }
    }
    {
        __MONETA_FN_COUNT_tribonacci.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let values_fmt = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_debug(&_arg0)],
                ),
            );
            res
        };
        {
            let args_fmt: String = ["n"]
                .into_iter()
                .zip(
                    [
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_debug(&_arg0)],
                                ),
                            );
                            res
                        },
                    ]
                        .into_iter(),
                )
                .map(|(n, v): (&str, String)| {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["\n\t", ": "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&n),
                                ::core::fmt::ArgumentV1::new_display(&v),
                            ],
                        ),
                    );
                    res
                })
                .collect();
            {
                ::std::io::_print(
                    ::core::fmt::Arguments::new_v1(
                        &["in ", ": ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"tribonacci"),
                            ::core::fmt::ArgumentV1::new_display(&args_fmt),
                        ],
                    ),
                );
            };
        }
        {
            if let Ok(reader) = __MONETA_FN_CACHE_tribonacci.read() {
                if let Some(val) = reader.get(&values_fmt) {
                    return val.clone();
                }
            }
        }
        let start = std::time::Instant::now();
        let res = __MONETA_FN_WRAPPER(_arg0);
        {
            ::std::io::_print(
                ::core::fmt::Arguments::new_v1(
                    &["out ", ": ", "\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&"tribonacci"),
                        ::core::fmt::ArgumentV1::new_debug(&start.elapsed()),
                    ],
                ),
            );
        };
        {
            if let Ok(mut writer) = __MONETA_FN_CACHE_tribonacci.write() {
                writer.entry(values_fmt).or_insert(res.clone());
            }
        }
        return res;
    }
}
#[cfg(test)]
mod tests {
    use macros::{count, get_cache, moneta};
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::multiple_call"]
    pub const multiple_call: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::multiple_call"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(multiple_call())),
    };
    fn multiple_call() {
        #[allow(non_upper_snake_case)]
        pub static __MONETA_FN_COUNT_foo: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(
            0,
        );
        ();
        pub fn foo() {
            pub fn __MONETA_FN_WRAPPER() {
                fns::baz();
            }
            {
                __MONETA_FN_COUNT_foo.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let values_fmt = {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(&[], &[]),
                    );
                    res
                };
                {
                    let args_fmt: String = []
                        .into_iter()
                        .zip([].into_iter())
                        .map(|(n, v): (&str, String)| {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["\n\t", ": "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&n),
                                        ::core::fmt::ArgumentV1::new_display(&v),
                                    ],
                                ),
                            );
                            res
                        })
                        .collect();
                    {
                        ::std::io::_print(
                            ::core::fmt::Arguments::new_v1(
                                &["in ", ": ", "\n"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&"foo"),
                                    ::core::fmt::ArgumentV1::new_display(&args_fmt),
                                ],
                            ),
                        );
                    };
                }
                {
                    if let Ok(reader) = __MONETA_FN_CACHE_foo.read() {
                        if let Some(val) = reader.get(&values_fmt) {
                            return val.clone();
                        }
                    }
                }
                let start = std::time::Instant::now();
                let res = __MONETA_FN_WRAPPER();
                {
                    ::std::io::_print(
                        ::core::fmt::Arguments::new_v1(
                            &["out ", ": ", "\n"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"foo"),
                                ::core::fmt::ArgumentV1::new_debug(&start.elapsed()),
                            ],
                        ),
                    );
                };
                {
                    if let Ok(mut writer) = __MONETA_FN_CACHE_foo.write() {
                        writer.entry(values_fmt).or_insert(res.clone());
                    }
                }
                return res;
            }
        }
        mod fns {
            #[allow(non_upper_snake_case)]
            pub static __MONETA_FN_COUNT_baz: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(
                0,
            );
            pub fn baz() {
                pub const fn __MONETA_FN_WRAPPER() {}
                {
                    __MONETA_FN_COUNT_baz
                        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let values_fmt = {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(&[], &[]),
                        );
                        res
                    };
                    {
                        let args_fmt: String = []
                            .into_iter()
                            .zip([].into_iter())
                            .map(|(n, v): (&str, String)| {
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["\n\t", ": "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_display(&n),
                                            ::core::fmt::ArgumentV1::new_display(&v),
                                        ],
                                    ),
                                );
                                res
                            })
                            .collect();
                        {
                            ::std::io::_print(
                                ::core::fmt::Arguments::new_v1(
                                    &["in ", ": ", "\n"],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&"baz"),
                                        ::core::fmt::ArgumentV1::new_display(&args_fmt),
                                    ],
                                ),
                            );
                        };
                    }
                    {
                        if let Ok(reader) = __MONETA_FN_CACHE_baz.read() {
                            if let Some(val) = reader.get(&values_fmt) {
                                return val.clone();
                            }
                        }
                    }
                    let start = std::time::Instant::now();
                    let res = __MONETA_FN_WRAPPER();
                    {
                        ::std::io::_print(
                            ::core::fmt::Arguments::new_v1(
                                &["out ", ": ", "\n"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&"baz"),
                                    ::core::fmt::ArgumentV1::new_debug(&start.elapsed()),
                                ],
                            ),
                        );
                    };
                    {
                        if let Ok(mut writer) = __MONETA_FN_CACHE_baz.write() {
                            writer.entry(values_fmt).or_insert(res.clone());
                        }
                    }
                    return res;
                }
            }
        }
        foo();
        fns::baz();
        match (&__MONETA_FN_COUNT_foo.load(std::sync::atomic::Ordering::SeqCst), &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &fns::__MONETA_FN_COUNT_baz.load(std::sync::atomic::Ordering::SeqCst),
            &2,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::no_cache"]
    pub const no_cache: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::no_cache"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(no_cache())),
    };
    fn no_cache() {
        #[allow(non_upper_snake_case)]
        pub static __MONETA_FN_COUNT_foo: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(
            0,
        );
        ();
        pub fn foo(mut _arg0: &mut u8) -> u8 {
            pub fn __MONETA_FN_WRAPPER(a: &mut u8) -> u8 {
                *a += 1;
                *a - 1
            }
            {
                __MONETA_FN_COUNT_foo.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let values_fmt = {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_debug(&_arg0)],
                        ),
                    );
                    res
                };
                {
                    let args_fmt: String = ["a"]
                        .into_iter()
                        .zip(
                            [
                                {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &[""],
                                            &[::core::fmt::ArgumentV1::new_debug(&_arg0)],
                                        ),
                                    );
                                    res
                                },
                            ]
                                .into_iter(),
                        )
                        .map(|(n, v): (&str, String)| {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["\n\t", ": "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&n),
                                        ::core::fmt::ArgumentV1::new_display(&v),
                                    ],
                                ),
                            );
                            res
                        })
                        .collect();
                    {
                        ::std::io::_print(
                            ::core::fmt::Arguments::new_v1(
                                &["in ", ": ", "\n"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&"foo"),
                                    ::core::fmt::ArgumentV1::new_display(&args_fmt),
                                ],
                            ),
                        );
                    };
                };
                let start = std::time::Instant::now();
                let res = __MONETA_FN_WRAPPER(_arg0);
                {
                    ::std::io::_print(
                        ::core::fmt::Arguments::new_v1(
                            &["out ", ": ", "\n"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"foo"),
                                ::core::fmt::ArgumentV1::new_debug(&start.elapsed()),
                            ],
                        ),
                    );
                };
                { }
                return res;
            }
        }
        let mut a = 0;
        match (&foo(&mut a), &0) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        if !__MONETA_FN_CACHE_foo.read().unwrap().is_empty() {
            ::core::panicking::panic(
                "assertion failed: get_cache!(foo).read().unwrap().is_empty()",
            )
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::cached"]
    pub const cached: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::cached"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(cached())),
    };
    fn cached() {
        #[allow(non_upper_snake_case)]
        pub static __MONETA_FN_COUNT_pow3: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(
            0,
        );
        ();
        pub fn pow3(mut _arg0: u128) -> u128 {
            pub const fn __MONETA_FN_WRAPPER(x: u128) -> u128 {
                x * x * x
            }
            {
                __MONETA_FN_COUNT_pow3.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let values_fmt = {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_debug(&_arg0)],
                        ),
                    );
                    res
                };
                {
                    let args_fmt: String = ["x"]
                        .into_iter()
                        .zip(
                            [
                                {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &[""],
                                            &[::core::fmt::ArgumentV1::new_debug(&_arg0)],
                                        ),
                                    );
                                    res
                                },
                            ]
                                .into_iter(),
                        )
                        .map(|(n, v): (&str, String)| {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["\n\t", ": "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&n),
                                        ::core::fmt::ArgumentV1::new_display(&v),
                                    ],
                                ),
                            );
                            res
                        })
                        .collect();
                    {
                        ::std::io::_print(
                            ::core::fmt::Arguments::new_v1(
                                &["in ", ": ", "\n"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&"pow3"),
                                    ::core::fmt::ArgumentV1::new_display(&args_fmt),
                                ],
                            ),
                        );
                    };
                }
                {
                    if let Ok(reader) = __MONETA_FN_CACHE_pow3.read() {
                        if let Some(val) = reader.get(&values_fmt) {
                            return val.clone();
                        }
                    }
                }
                let start = std::time::Instant::now();
                let res = __MONETA_FN_WRAPPER(_arg0);
                {
                    ::std::io::_print(
                        ::core::fmt::Arguments::new_v1(
                            &["out ", ": ", "\n"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"pow3"),
                                ::core::fmt::ArgumentV1::new_debug(&start.elapsed()),
                            ],
                        ),
                    );
                };
                {
                    if let Ok(mut writer) = __MONETA_FN_CACHE_pow3.write() {
                        writer.entry(values_fmt).or_insert(res.clone());
                    }
                }
                return res;
            }
        }
        match (&(0..10).map(pow3).sum::<u128>(), &2025) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&__MONETA_FN_CACHE_pow3.read().unwrap().len(), &10) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&cached, &multiple_call, &no_cache])
}
