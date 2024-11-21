extern crate criterion;
use criterion::*;

use application::mat_mult::{mat_mult, naive_mat_mult, Matrix};
use criterion::Criterion;
use util::algebra::field::mersenne61_ext::Mersenne61Ext;

fn bench_naive_mat_mult(c: &mut Criterion) {
    let mat_a = Matrix::<Mersenne61Ext>::sample(150, 768);
    let mat_b = Matrix::<Mersenne61Ext>::sample(768, 2304);
    let mat_c = mat_a.clone() * mat_b.clone();

    c.bench_function(&format!("naive mat mult"), move |b| {
        b.iter_batched(
            || (mat_a.clone(), mat_b.clone(), mat_c.clone()),
            |(a, b, c)| {
                naive_mat_mult(&a, &b, &c);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_mat_mult(c: &mut Criterion) {
    let mat_a = Matrix::<Mersenne61Ext>::sample(150, 768);
    let mat_b = Matrix::<Mersenne61Ext>::sample(768, 2304);
    let mat_c = mat_a.clone() * mat_b.clone();

    c.bench_function(&format!("improved mat mult"), move |b| {
        b.iter_batched(
            || (mat_a.clone(), mat_b.clone(), mat_c.clone()),
            |(a, b, c)| {
                mat_mult(&a, &b, &c);
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_naive_mat_mult, bench_mat_mult
}

criterion_main!(benches);
