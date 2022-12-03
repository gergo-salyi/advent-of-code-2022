#[cfg(not(all(
    target_arch = "x86_64",
    target_feature = "avx512bw",
    target_feature = "avx512f"
)))]
compile_error!();

use core::arch::x86_64::_rdtsc;

use core::arch::x86_64::{
    __m512i, _mm256_add_epi64, _mm256_extracti128_si256, _mm512_add_epi16,
    _mm512_add_epi32, _mm512_add_epi64, _mm512_add_epi8,
    _mm512_cmpeq_epi8_mask, _mm512_cvtepu16_epi32, _mm512_cvtepu32_epi64,
    _mm512_cvtepu8_epi16, _mm512_extracti64x4_epi64, _mm512_loadu_si512,
    _mm512_mask_blend_epi8, _mm512_set1_epi8, _mm512_setzero_si512,
    _mm512_shuffle_epi8, _mm512_storeu_si512, _mm512_sub_epi8,
    _mm512_unpackhi_epi8, _mm512_unpacklo_epi8, _mm_add_epi64,
    _mm_storeu_si128,
};

use std::ptr::copy_nonoverlapping;
use std::time::{Duration, Instant};

const INPUT_BYTES: &[u8] = include_bytes!("../res/input02");

const EXT: &[u8; 4] = b"A W\n";

pub fn part1() {
    let start = unsafe { _rdtsc() };
    let answer = run1(INPUT_BYTES);
    let bench = unsafe { _rdtsc() - start };
    println!("{answer}"); // 17189
    eprintln!("{bench}");
}

pub fn part2() {
    // println!("{}", run2(INPUT)) // 13490
}

// #[no_mangle]
// #[inline(never)]
// pub extern "C" fn run1(input: &[u8]) -> u64 {
pub fn run1(input: &[u8]) -> u64 {
    unsafe { score_avx512(input) }
}

unsafe fn dbg512(packed: __m512i) {
    let mut byte_array = [0u8; 64];
    _mm512_storeu_si512(byte_array.as_mut_ptr() as _, packed);
    eprintln!("{:?}", byte_array);
}

unsafe fn score_avx512(input_slice: &[u8]) -> u64 {
    let shape_mask1: [i8; 64] = [
        0, 4, 8, 12, -1, -1, -1, -1, 2, 6, 10, 14, -1, -1, -1, -1, 0, 4, 8, 12,
        -1, -1, -1, -1, 2, 6, 10, 14, -1, -1, -1, -1, 0, 4, 8, 12, -1, -1, -1,
        -1, 2, 6, 10, 14, -1, -1, -1, -1, 0, 4, 8, 12, -1, -1, -1, -1, 2, 6,
        10, 14, -1, -1, -1, -1,
    ];
    let shape_mask2: [i8; 64] = [
        -1, -1, -1, -1, 0, 4, 8, 12, -1, -1, -1, -1, 2, 6, 10, 14, -1, -1, -1,
        -1, 0, 4, 8, 12, -1, -1, -1, -1, 2, 6, 10, 14, -1, -1, -1, -1, 0, 4, 8,
        12, -1, -1, -1, -1, 2, 6, 10, 14, -1, -1, -1, -1, 0, 4, 8, 12, -1, -1,
        -1, -1, 2, 6, 10, 14,
    ];
    let shape_mask1 = _mm512_loadu_si512(shape_mask1.as_ptr() as _);
    let shape_mask2 = _mm512_loadu_si512(shape_mask2.as_ptr() as _);

    // truncate any trailing characters
    let len = input_slice.len();
    let truncated_len = len - (len % 4);
    let input_slice = &input_slice[0..truncated_len];

    // extend slice to N x 256 bytes
    let clean_chunks = truncated_len / 256;
    let extra_count = truncated_len % 256;
    let extension_chunks_needed = (256 - extra_count) / 4;

    let mut acc = _mm512_setzero_si512();
    for i in 0..clean_chunks {
        let offset = i * 256;
        let score_of_256 = score_512(
            input_slice[offset..(offset + 256)].try_into().unwrap(),
            shape_mask1,
            shape_mask2,
        );
        acc = _mm512_add_epi16(acc, score_of_256);
    }
    /*
    if extension_chunks_needed > 0 {
        let mut extended_slice = [0u8; 256];
        copy_nonoverlapping(
            input_slice[(truncated_len-extra_count)..truncated_len].as_ptr(),
            (&mut extended_slice).as_mut_ptr(),
            extra_count
        );
        for j in 0..extension_chunks_needed {
            copy_nonoverlapping(
                EXT.as_ptr(),
                (&mut extended_slice[(4*j+extra_count)..]).as_mut_ptr(),
                4
            );
        }
        // dbg!(extended_slice);
        let score_of_256 = score_512(
            &extended_slice.try_into().unwrap(),
            shape_mask1,
            shape_mask2
        );
        acc = _mm512_add_epi16(acc, score_of_256);
    }
    */

    // horizontal sum
    horizontal_sum(acc)
}

#[inline(always)]
unsafe fn score_512(
    input_slice: &[u8; 256],
    shape_mask1: __m512i,
    shape_mask2: __m512i,
) -> __m512i {
    // Load input
    let input_simd_1 = _mm512_loadu_si512(input_slice[0..64].as_ptr() as _);
    let input_simd_2 = _mm512_loadu_si512(input_slice[64..128].as_ptr() as _);
    let input_simd_3 = _mm512_loadu_si512(input_slice[128..192].as_ptr() as _);
    let input_simd_4 = _mm512_loadu_si512(input_slice[192..256].as_ptr() as _);

    // get two [ op x4, 00 x4, my x4, 00 x4, op x4, 00 x4, my x4, 00 x4 ]
    let shape_1_half_1 = _mm512_shuffle_epi8(input_simd_1, shape_mask1);
    let shape_2_half_1 = _mm512_shuffle_epi8(input_simd_3, shape_mask1);

    // get two [ 00 x4, op x4, 00 x4, my x4, 00 x4, op x4, 00 x4, my x4 ]
    let shape_1_half_2 = _mm512_shuffle_epi8(input_simd_2, shape_mask2);
    let shape_2_half_2 = _mm512_shuffle_epi8(input_simd_4, shape_mask2);

    // get two [ op x 8, my x 8, op x 8, my x 8 ]
    let shapes_1 = _mm512_add_epi8(shape_1_half_1, shape_1_half_2);
    let shapes_2 = _mm512_add_epi8(shape_2_half_1, shape_2_half_2);

    // get [ op x 64 ] and [ my x 64 ]
    let op_shapes = _mm512_unpacklo_epi8(shapes_1, shapes_2);
    let my_shapes = _mm512_unpackhi_epi8(shapes_1, shapes_2);

    let shape_score_offset = _mm512_set1_epi8(87);
    let shape_score = _mm512_sub_epi8(my_shapes, shape_score_offset);

    let outcome = _mm512_sub_epi8(my_shapes, op_shapes);
    // Draw:  23
    // Win:   24 | 21
    // Loss:  22 | 25

    let outcome_score_offset = _mm512_set1_epi8(22);
    let outcome = _mm512_sub_epi8(outcome, outcome_score_offset);
    // This needs to be mapped:
    // Draw:  1       =>   3
    // Win:   2 | -1  =>   6
    // Loss:  0 | 3   =>   0

    let bad_win_value = _mm512_set1_epi8(-1);
    let good_win_value = _mm512_set1_epi8(2);
    let bad_win_mask = _mm512_cmpeq_epi8_mask(outcome, bad_win_value);
    let outcome = _mm512_mask_blend_epi8(bad_win_mask, outcome, good_win_value);

    let bad_loss_value = _mm512_set1_epi8(3);
    let good_loss_value = _mm512_set1_epi8(0);
    let bad_loss_mask = _mm512_cmpeq_epi8_mask(outcome, bad_loss_value);
    let outcome =
        _mm512_mask_blend_epi8(bad_loss_mask, outcome, good_loss_value);

    // outcome = outcome * 3 = outcome + outcome + outcome
    let outcome2 = _mm512_add_epi8(outcome, outcome);
    let outcome_score = _mm512_add_epi8(outcome, outcome2);

    // get [ score x 64 ]
    let score = _mm512_add_epi8(outcome_score, shape_score);

    let a = _mm512_cvtepu8_epi16(_mm512_extracti64x4_epi64::<0>(score));
    let b = _mm512_cvtepu8_epi16(_mm512_extracti64x4_epi64::<1>(score));
    let score = _mm512_add_epi16(a, b);

    score
}

#[inline(always)]
unsafe fn horizontal_sum(score: __m512i) -> u64 {
    // let a = _mm256_cvtepu8_epi16( _mm256_extracti128_si256::<0>(u8x32) );
    // let b = _mm256_cvtepu8_epi16( _mm256_extracti128_si256::<1>(u8x32) );
    // let score = _mm256_add_epi16(a, b);
    let a = _mm512_cvtepu16_epi32(_mm512_extracti64x4_epi64::<0>(score));
    let b = _mm512_cvtepu16_epi32(_mm512_extracti64x4_epi64::<1>(score));
    let score = _mm512_add_epi32(a, b);
    let a = _mm512_cvtepu32_epi64(_mm512_extracti64x4_epi64::<0>(score));
    let b = _mm512_cvtepu32_epi64(_mm512_extracti64x4_epi64::<1>(score));
    let score = _mm512_add_epi64(a, b);
    let a = _mm512_extracti64x4_epi64::<0>(score);
    let b = _mm512_extracti64x4_epi64::<1>(score);
    let score = _mm256_add_epi64(a, b);
    let a = _mm256_extracti128_si256::<0>(score);
    let b = _mm256_extracti128_si256::<1>(score);
    let score = _mm_add_epi64(a, b);
    let mut buffer = [0u64; 2];
    _mm_storeu_si128(buffer.as_mut_ptr() as _, score);
    let score = buffer[0] + buffer[1];
    score
}

fn run2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../res/example02");

    #[test]
    fn test1() {
        // assert_eq!(run1(EXAMPLE), 15)
    }

    #[test]
    fn test2() {
        // assert_eq!(run2(EXAMPLE), 12)
    }
}
