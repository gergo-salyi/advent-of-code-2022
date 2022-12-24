#![feature(core_intrinsics, stdsimd, byte_slice_trim_ascii)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

/*

                Part 1  Part 2

Day  8           29 us  311 us

Day  9          193 us  229 us
Day  9 nohash    72 us  153 us

Day 21           66 us   72 us
Day 23          325 us   40 ms
Day 24          6.7 ms   22 ms

*/

/*
mod day01;
mod day01opt;
fn day01(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../res/input01");
    const INPUT_STR: &str = include_str!("../res/input01");
    c.bench_function("day01-part1", |b| {
        b.iter(|| day01::run1(black_box(INPUT_STR)))
    });
    c.bench_function("day01-part1-opt", |b| {
        b.iter(|| day01opt::run1(black_box(INPUT)))
    });
    c.bench_function("day01-part2", |b| {
        b.iter(|| day01::run2(black_box(INPUT_STR)))
    });
    c.bench_function("day01-part2-opt", |b| {
        b.iter(|| day01opt::run2(black_box(INPUT)))
    });
}
mod day02;
mod day02avx2;
mod day02avx2upscale;
mod day02avx512;
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
fn day02_upscale(c: &mut Criterion) {
    let input = day02avx2upscale::input_bytes();
    c.bench_function("day02-upscale", |b| {
        b.iter(|| day02avx2upscale::run1(black_box(&input)))
    });
}

mod day03;
mod day03bitfield;
fn day03_part1(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../res/input03");
    c.bench_function("day03-part1", |b| {
        b.iter(|| day03::run1(black_box(INPUT)))
    });
    c.bench_function("day03-part1-bitfield", |b| {
        b.iter(|| day03bitfield::run1(black_box(INPUT)))
    });
    // c.bench_function(
    //     "day02-part1-avx512",
    //     |b| b.iter(|| day02avx512::run1(black_box(INPUT_BYTES)))
    // );
}

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

mod day06;
mod day06simd;
fn day06(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../res/input06");
    // c.bench_function("day06-part1", |b| {
    //     b.iter(|| day06::run1(black_box(INPUT)))
    // });
    // c.bench_function("day06-part2", |b| {
    //     b.iter(|| day06::run2(black_box(INPUT)))
    // });
    c.bench_function("day06-part2-simd", |b| {
        b.iter(|| day06simd::run2(black_box(INPUT)))
    });
    // c.bench_function("day06-part2-sliding-window", |b| {
    //     b.iter(|| day06simd::sliding_window(black_box(INPUT)))
    // });
}

mod day07;
fn day07(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../res/input07");
    const INPUT_DEEP: &[u8] = include_bytes!("../res/input07deep");
    c.bench_function("day07-part1", |b| {
        b.iter(|| day07::run1(black_box(INPUT)))
    });
    c.bench_function("day07-part2", |b| {
        b.iter(|| day07::run2(black_box(INPUT)))
    });
    c.bench_function("day07-part1-deep", |b| {
        b.iter(|| day07::run1(black_box(INPUT_DEEP)))
    });
    c.bench_function("day07-part2-deep", |b| {
        b.iter(|| day07::run2(black_box(INPUT_DEEP)))
    });
}

mod day08;
mod day08simd;
fn day08(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../res/input08");
    // c.bench_function("day08-part1", |b| {
    //     b.iter(|| day08::run1(black_box(INPUT)))
    // });
    c.bench_function("day08-part2", |b| {
        b.iter(|| day08::run2(black_box(INPUT)))
    });
    // c.bench_function("day08-part1-simd", |b| {
    //     b.iter(|| day08simd::run1(black_box(INPUT)))
    // });
    c.bench_function("day08-part2-simd", |b| {
        b.iter(|| day08simd::run2(black_box(INPUT)))
    });
}

mod day09;
mod day09nohash;
fn day09(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../res/input09");
    // c.bench_function("day09-part1", |b| {
    //     b.iter(|| day09::run1(black_box(INPUT)))
    // });
    c.bench_function("day09-part1-nohash", |b| {
        b.iter(|| day09nohash::run1(black_box(INPUT)))
    });
    // c.bench_function("day09-part2", |b| {
    //     b.iter(|| day09::run2(black_box(INPUT)))
    // });
    c.bench_function("day09-part2-nohash", |b| {
        b.iter(|| day09nohash::run2(black_box(INPUT)))
    });
}

mod day11;
fn day11(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../res/input11");
    c.bench_function("day11-part1", |b| {
        b.iter(|| day11::run1(black_box(INPUT)))
    });
    c.bench_function("day11-part2", |b| {
        b.iter(|| day11::run2(black_box(INPUT)))
    });
}

mod day20;
fn day20(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../res/input20");
    c.bench_function("day20-part1", |b| {
        b.iter(|| day20::run1(black_box(INPUT)))
    });
    c.bench_function("day20-part2", |b| {
        b.iter(|| day20::run2(black_box(INPUT)))
    });
}

mod day21;
fn day21(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../res/input21");
    c.bench_function("day21-part1", |b| {
        b.iter(|| day21::run1(black_box(INPUT)))
    });
    c.bench_function("day21-part2", |b| {
        b.iter(|| day21::run2(black_box(INPUT)))
    });
}
*/

mod day24;
fn day24(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../res/input24");
    c.bench_function("day24-part1", |b| {
        b.iter(|| day24::run1(black_box(INPUT)))
    });
    c.bench_function("day24-part2", |b| {
        b.iter(|| day24::run2(black_box(INPUT)))
    });
}

criterion_group!(benches, day24);
criterion_main!(benches);
