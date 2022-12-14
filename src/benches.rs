use bencher::{benchmark_group, benchmark_main, black_box, Bencher};
use moneta_fn::moneta;

#[must_use]
fn uncached_fibonacci(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => (1..=2).map(|o| uncached_fibonacci(n - o)).sum(),
    }
}

#[must_use]
fn uncached_tribonacci(n: u128) -> u128 {
    match n {
        0 => 0,
        1 | 2 => 1,
        3 => 2,
        _ => (1..=3).map(|o| uncached_tribonacci(n - o)).sum(),
    }
}

#[moneta(trace = "forbid", count = "forbid", time = "forbid")]
#[must_use]
pub fn cached_fibonacci(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => (1..=2).map(|o| cached_fibonacci(n - o)).sum(),
    }
}

#[moneta(trace = "forbid", count = "forbid", time = "forbid")]
#[must_use]
pub fn cached_tribonacci(n: u128) -> u128 {
    match n {
        0 => 0,
        1 | 2 => 1,
        3 => 2,
        _ => (1..=3).map(|o| cached_tribonacci(n - o)).sum(),
    }
}

fn cached_fibonacci_10(b: &mut Bencher) {
    b.iter(|| assert_eq!(cached_fibonacci(black_box(10)), 89))
}

fn cached_tribonacci_10(b: &mut Bencher) {
    b.iter(|| assert_eq!(cached_tribonacci(black_box(10)), 149))
}

fn uncached_fibonacci_10(b: &mut Bencher) {
    b.iter(|| assert_eq!(uncached_fibonacci(black_box(10)), 89))
}

fn uncached_tribonacci_10(b: &mut Bencher) {
    b.iter(|| assert_eq!(uncached_tribonacci(black_box(10)), 149))
}

benchmark_group!(cached_seqs, cached_fibonacci_10, cached_tribonacci_10);
benchmark_group!(uncached_seqs, uncached_fibonacci_10, uncached_tribonacci_10);
benchmark_main!(cached_seqs, uncached_seqs);
