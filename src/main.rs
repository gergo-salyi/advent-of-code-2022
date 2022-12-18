#![feature(core_intrinsics, stdsimd, byte_slice_trim_ascii)]

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

/*

mod decimal;
mod day01;
mod day02;
mod day02avx2;
mod day02avx2upscale;
mod day02avx512;
mod day03;
mod day03bitfield;
mod day04;
mod day05;
mod day06;
mod day06simd;
mod day07;
mod day08;


mod day07tree;

mod day08simd;

mod day09;
mod day09nohash;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day17;
mod day18;
*/

mod day16;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    day16::part1();
    day16::part2();
}
