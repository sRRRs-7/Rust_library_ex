
// cargo bench

use criterion::{criterion_group, criterion_main, Criterion};

fn bm1(c: &mut Criterion) {
    c.bench_function("bench1",
    |b| b.iter(|| {
        let mut cnt = 0;
        for i in 1..=100 {
            cnt += i
        }
        println!("{}", cnt);
    }),);
}

fn bm2(c: &mut Criterion) {
    c.bench_function("bench2",
    |b| b.iter(|| {
        let mut cnt = 0;
        for i in 1..=100 {
            cnt += i
        }
        println!("{}", cnt);
    }),);
}

criterion_group!(benches, bm1, bm2);
criterion_main!(benches);