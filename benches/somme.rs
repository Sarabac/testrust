use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testrust::fibo::{fibo2, fibonacci};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

pub fn crit_fibo2(c: &mut Criterion) {
    c.bench_function("fibo encore", |b| b.iter(|| fibo2(black_box(20))));
}

criterion_group!(benches, criterion_benchmark, crit_fibo2);
criterion_main!(benches);
