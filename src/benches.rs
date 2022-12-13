use bencher::{benchmark_group, benchmark_main, black_box, Bencher};

pub fn uncached_fibonacci(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => (1..=2).map(|o| uncached_fibonacci(n - o)).sum(),
    }
}

pub fn uncached_tribonacci(n: u128) -> u128 {
    match n {
        0 => 0,
        1 | 2 => 1,
        3 => 2,
        _ => (1..=3).map(|o| uncached_tribonacci(n - o)).sum(),
    }
}

fn fibonacci_10(b: &mut Bencher) {
    b.iter(|| assert_eq!(moneta_bin::fibonacci(black_box(10)), 89))
}

fn tribonacci_10(b: &mut Bencher) {
    b.iter(|| assert_eq!(moneta_bin::tribonacci(black_box(10)), 149))
}

fn uncached_fibonacci_10(b: &mut Bencher) {
    b.iter(|| assert_eq!(uncached_fibonacci(black_box(10)), 89))
}

fn uncached_tribonacci_10(b: &mut Bencher) {
    b.iter(|| assert_eq!(uncached_tribonacci(black_box(10)), 149))
}

benchmark_group!(cached_seqs, fibonacci_10, tribonacci_10);
benchmark_group!(uncached_seqs, uncached_fibonacci_10, uncached_tribonacci_10);
benchmark_main!(cached_seqs, uncached_seqs);
