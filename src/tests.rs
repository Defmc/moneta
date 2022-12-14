use moneta_fn::{count, get_cache, moneta, reset_count};

#[test]
fn multiple_call() {
    #[moneta]
    pub fn foo() {
        fns::baz();
    }

    mod fns {
        #[moneta_fn::moneta]
        pub const fn baz() {}
    }

    foo();
    fns::baz();
    assert_eq!(count!(foo), 1);
    assert_eq!(count!(fns::baz), 2);
}

#[test]
fn no_cache() {
    #[moneta(cache = "forbid")]
    pub fn foo(a: &mut u8) -> u8 {
        *a += 1;
        *a - 1
    }

    let mut a = 0;
    assert_eq!(foo(&mut a), 0);
    assert!(get_cache!(foo).read().unwrap().is_empty());
}

#[test]
fn cached() {
    #[moneta]
    pub const fn pow3(x: u128) -> u128 {
        x * x * x
    }

    assert_eq!((0..10).map(pow3).sum::<u128>(), 2025);
    assert_eq!(get_cache!(pow3).read().unwrap().len(), 10);
}

#[test]
fn reset_count() {
    #[moneta]
    pub const fn pow2(x: usize) -> usize {
        x * x
    }

    assert_eq!(pow2(2), 4);
    assert_eq!(count!(pow2), 1);
    reset_count!(pow2);
    assert_eq!(count!(pow2), 0);
}
