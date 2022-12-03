#![feature(core_intrinsics, stdsimd)]

mod day01;
mod day02;
mod day02avx2;
// mod day02avx512;
mod day03;

fn main() {
    day02avx2::part1()
}
