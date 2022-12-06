#![feature(core_intrinsics, stdsimd)]

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// mod decimal;
// mod day01;
// mod day02;
// mod day02avx2;
// mod day02avx2upscale;
// mod day02avx512;
// mod day03;
// mod day03bitfield;
// mod day04;
mod day05;
mod day06;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    day06::part2()
}
