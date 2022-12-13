use bencher::{benchmark_group, benchmark_main, black_box, Bencher};

fn fibonacci_50(b: &mut Bencher) {
    b.iter(|| assert_eq!(moneta_bin::fibonacci(black_box(10)), 89))
}

fn tribonacci_10(b: &mut Bencher) {
    b.iter(|| assert_eq!(moneta_bin::tribonacci(black_box(10)), 149))
}

benchmark_group!(num_seqs, fibonacci_50, tribonacci_10);
benchmark_main!(num_seqs);
