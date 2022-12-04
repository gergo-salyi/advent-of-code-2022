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

// mod day03;
// mod day03bitfield;
// fn day03_part1(c: &mut Criterion) {
//     const INPUT: &[u8] = include_bytes!("../res/input03");
//     c.bench_function("day03-part1", |b| {
//         b.iter(|| day03::run1(black_box(INPUT)))
//     });
//     c.bench_function("day03-part1-bitfield", |b| {
//         b.iter(|| day03bitfield::run1(black_box(INPUT)))
//     });
//     // c.bench_function(
//     //     "day02-part1-avx512",
//     //     |b| b.iter(|| day02avx512::run1(black_box(INPUT_BYTES)))
//     // );
// }

mod day04;
fn day04(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../res/input04");
    // c.bench_function("day04-part1", |b| {
    //     b.iter(|| day04::run1old(black_box(INPUT)))
    // });
    c.bench_function("day04-part1-opt", |b| {
        b.iter(|| day04::run1(black_box(INPUT)))
    });
    // c.bench_function("day04-part2", |b| {
    //     b.iter(|| day04::run2old(black_box(INPUT)))
    // });
    c.bench_function("day04-part2-opt", |b| {
        b.iter(|| day04::run2(black_box(INPUT)))
    });
}

criterion_group!(benches, day04);
criterion_main!(benches);
