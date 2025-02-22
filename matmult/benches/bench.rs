extern crate criterion;
use std::time::Instant;

use criterion::*;

use criterion::Criterion;
use matmult::{mat_mult, naive_mat_mult, naive_opening, Matrix};
use util::algebra::field::mersenne61_ext::Mersenne61Ext;

fn bench_naive_mat_mult(c: &mut Criterion) {
    let row_size = vec![150, 300, 600, 900, 1200];
    println!("naive mat mult");
    for r in row_size {
        let mat_a = Matrix::<Mersenne61Ext>::sample(r, 768);
        let mat_b = Matrix::<Mersenne61Ext>::sample(768, 2304);
        let mat_c = mat_a.clone() * mat_b.clone();

        let start = Instant::now();
        let mut v_time = 0;
        let mut size = 0;
        for _ in 0..10 {
            let res = naive_mat_mult(&mat_a, &mat_b, &mat_c);
            v_time += res.0;
            size += res.1;
        }
        let p_time = start.elapsed().as_micros() as usize;
        println!(
            "row size: {}, prover time: {} us, verifier time: {} us, size: {} bytes",
            r,
            p_time / 10,
            v_time / 10,
            size / 10
        );
    }
}

fn bench_mat_mult(c: &mut Criterion) {
    let row_size = vec![150, 300, 600, 900, 1200];
    println!("batch-deepfold mat mult");
    for r in row_size {
        let mat_a = Matrix::<Mersenne61Ext>::sample(r, 768);
        let mat_b = Matrix::<Mersenne61Ext>::sample(768, 2304);
        let mat_c = mat_a.clone() * mat_b.clone();

        let start = Instant::now();
        let mut v_time = 0;
        let mut size = 0;
        for _ in 0..10 {
            let res = mat_mult(&mat_a, &mat_b, &mat_c);
            v_time += res.0;
            size += res.1;
        }
        let p_time = start.elapsed().as_micros() as usize;
        println!(
            "row size: {}, prover time: {} us, verifier time: {} us, size: {} bytes",
            r,
            p_time / 10,
            v_time / 10,
            size / 10
        );
    }
}

fn bench_naive_opening(c: &mut Criterion) {
    let row_size = vec![150, 300, 600, 900, 1200];
    println!("naive opening mat mult");
    for r in row_size {
        let mat_a = Matrix::<Mersenne61Ext>::sample(r, 768);
        let mat_b = Matrix::<Mersenne61Ext>::sample(768, 2304);
        let mat_c = mat_a.clone() * mat_b.clone();

        let start = Instant::now();
        let mut v_time = 0;
        let mut size = 0;
        for _ in 0..10 {
            let res = naive_opening(&mat_a, &mat_b, &mat_c);
            v_time += res.0;
            size += res.1;
        }
        let p_time = start.elapsed().as_micros() as usize;
        println!(
            "row size: {}, prover time: {} us, verifier time: {} us, size: {} bytes",
            r,
            p_time / 10,
            v_time / 10,
            size / 10
        );
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_naive_mat_mult, bench_mat_mult, bench_naive_opening
}

criterion_main!(benches);
