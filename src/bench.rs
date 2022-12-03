#![feature(core_intrinsics, stdsimd)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod day02;
mod day02avx2;
// mod day02avx512;
fn day02_part1(c: &mut Criterion) {
    const INPUT_STR: &str = include_str!("../res/input02");
    const INPUT_BYTES: &[u8] = include_bytes!("../res/input02");
    c.bench_function("day02-part1", |b| {
        b.iter(|| day02::run1(black_box(INPUT_STR)))
    });
    c.bench_function("day02-part1-avx2", |b| {
        b.iter(|| day02avx2::run1(black_box(INPUT_BYTES)))
    });
    // c.bench_function(
    //     "day02-part1-avx512",
    //     |b| b.iter(|| day02avx512::run1(black_box(INPUT_BYTES)))
    // );
}

criterion_group!(benches, day02_part1);
criterion_main!(benches);
