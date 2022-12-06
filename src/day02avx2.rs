#[cfg(not(all(target_arch = "x86_64", target_feature = "avx2",)))]
compile_error!("needs x86_64 and AVX2");

use core::arch::x86_64::{
    __m256i, _mm256_add_epi16, _mm256_add_epi32, _mm256_add_epi64,
    _mm256_add_epi8, _mm256_adds_epu16, _mm256_adds_epu8, _mm256_blendv_epi8,
    _mm256_cmpeq_epi8, _mm256_cvtepu16_epi32, _mm256_cvtepu32_epi64,
    _mm256_cvtepu8_epi16, _mm256_extracti128_si256, _mm256_loadu_si256,
    _mm256_set1_epi8, _mm256_setr_epi8, _mm256_setzero_si256,
    _mm256_shuffle_epi8, _mm256_storeu_si256, _mm256_sub_epi8,
    _mm256_unpackhi_epi8, _mm256_unpacklo_epi8, _mm_add_epi64,
    _mm_storeu_si128,
};

use std::{intrinsics::unlikely, mem::MaybeUninit, ptr::copy_nonoverlapping};

const INPUT_BYTES: &[u8] = include_bytes!("../res/input02");

// Rock-paper-scissors game which yields 0 point in part 1
const IDENTITY_GAME: &[u8; 4] = b"A W\n";
const IDENTITY_ARRAY: &[[u8; 4]; 32] = &[*IDENTITY_GAME; 32];

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT_BYTES)) // 17189
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT_BYTES)) // 13490
}

pub fn run1(input: &[u8]) -> u64 {
    unsafe { score_avx2(input) }
}

#[allow(unused)]
unsafe fn dbg_m256i(packed: __m256i) {
    let mut byte_array = [0u8; 32];
    _mm256_storeu_si256(byte_array.as_mut_ptr() as _, packed);
    eprintln!("{:?}", byte_array);
}

unsafe fn score_avx2(input_slice: &[u8]) -> u64 {
    let len = input_slice.len();

    debug_assert!(len % 4 == 0);

    // slice will need to be extended to N x 128 characters
    let clean_chunks = len / 128;
    let extra_count = len % 128;
    let needs_extra_chunk = extra_count > 0;

    // Do the memcpy business before we start filling the SIMD registers
    // The rest of the input after N x 128 characters is copied first to
    // the "extra_chunk" then the blank left is filled with 0 score yielding
    // "identity rock-paper-scissors games": "A <space> W \n"
    let mut extra_chunk: [MaybeUninit<u8>; 128] =
        MaybeUninit::uninit().assume_init();

    if needs_extra_chunk {
        copy_nonoverlapping(
            input_slice[(len - extra_count)..len].as_ptr(),
            extra_chunk.as_mut_ptr() as _,
            extra_count,
        );
        copy_nonoverlapping(
            IDENTITY_ARRAY.as_ptr() as _,
            extra_chunk[extra_count..].as_mut_ptr(),
            128 - extra_count,
        );
    }

    // Set SIMD constants and variables
    let shape_mask1 = _mm256_setr_epi8(
        0, 4, 8, 12, -1, -1, -1, -1, 2, 6, 10, 14, -1, -1, -1, -1, 0, 4, 8, 12,
        -1, -1, -1, -1, 2, 6, 10, 14, -1, -1, -1, -1,
    );
    let shape_mask2 = _mm256_setr_epi8(
        -1, -1, -1, -1, 0, 4, 8, 12, -1, -1, -1, -1, 2, 6, 10, 14, -1, -1, -1,
        -1, 0, 4, 8, 12, -1, -1, -1, -1, 2, 6, 10, 14,
    );
    let shape_score_offset = _mm256_set1_epi8(87);
    let outcome_score_offset = _mm256_set1_epi8(22);
    let bad_win_value = _mm256_set1_epi8(-1);
    let good_win_value = _mm256_set1_epi8(2);
    let bad_loss_value = _mm256_set1_epi8(3);
    let good_loss_value = _mm256_set1_epi8(0);

    let mut acc_u8x32 = _mm256_setzero_si256();
    let mut acc_u16x16 = _mm256_setzero_si256();

    // Process the input
    for i in 0..clean_chunks {
        let offset = i * 128;
        let chunk_score = score_u8x128(
            &*(input_slice[offset..(offset + 128)].as_ptr()
                as *const [u8; 128]),
            shape_mask1,
            shape_mask2,
            shape_score_offset,
            outcome_score_offset,
            bad_win_value,
            good_win_value,
            bad_loss_value,
            good_loss_value,
        );
        acc_u8x32 = _mm256_adds_epu8(acc_u8x32, chunk_score);

        // Cycle var i couts chunks. 1 chunk = 128 byte = 32 game
        // Acc u8 units get max 9 points / cycle
        // Acc u16 units get max 18 points / cycle

        // The u8x32 accumulator fills up every 28 cycles (255/9)
        if unlikely(i % 28 == 27) {
            // Flush acc_u8x32 into acc_u16x16 and reset acc_u8x32 to zero
            acc_u16x16 = _mm256_adds_epu16(
                acc_u16x16,
                hadd_get_u16x16_from_u8x32(acc_u8x32),
            );
            acc_u8x32 = _mm256_setzero_si256();
        }

        // The u16x16 accumulator would fill up every 3640 cycles (65535/18)
        // This is not reached with the current input (2500 lines)
    }

    if needs_extra_chunk {
        let extra_chunk_score = score_u8x128(
            // We only reach here through the same needs_extra_chunk
            // condition, which ensured that the extra_chunk is initialized
            // if it will be needed here -> assume init
            &*(extra_chunk.as_ptr() as *const [u8; 128]),
            shape_mask1,
            shape_mask2,
            shape_score_offset,
            outcome_score_offset,
            bad_win_value,
            good_win_value,
            bad_loss_value,
            good_loss_value,
        );
        acc_u8x32 = _mm256_adds_epu8(acc_u8x32, extra_chunk_score);
    }

    // Flush the rest of acc_u8x32 into acc_u16x16
    acc_u16x16 =
        _mm256_adds_epu16(acc_u16x16, hadd_get_u16x16_from_u8x32(acc_u8x32));

    // Horizontal sum acc_u16x16 to get the answer
    horizontal_sum_u16x16(acc_u16x16)
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
unsafe fn score_u8x128(
    input_array: &[u8; 128],
    shape_mask1: __m256i,
    shape_mask2: __m256i,
    shape_score_offset: __m256i,
    outcome_score_offset: __m256i,
    bad_win_value: __m256i,
    good_win_value: __m256i,
    bad_loss_value: __m256i,
    good_loss_value: __m256i,
) -> __m256i {
    // op : char code of the opponent's shape
    // my : char code of my shape

    // Load input
    // get four [ [ op, \s, my, \n ] x 8 ]
    let input_simd_1 = _mm256_loadu_si256(input_array[0..32].as_ptr() as _);
    let input_simd_2 = _mm256_loadu_si256(input_array[32..64].as_ptr() as _);
    let input_simd_3 = _mm256_loadu_si256(input_array[64..96].as_ptr() as _);
    let input_simd_4 = _mm256_loadu_si256(input_array[96..128].as_ptr() as _);

    // get two [ op x4, 00 x4, my x4, 00 x4, op x4, 00 x4, my x4, 00 x4 ]
    let shapes_1_half_1 = _mm256_shuffle_epi8(input_simd_1, shape_mask1);
    let shapes_2_half_1 = _mm256_shuffle_epi8(input_simd_3, shape_mask1);

    // get two [ 00 x4, op x4, 00 x4, my x4, 00 x4, op x4, 00 x4, my x4 ]
    let shapes_1_half_2 = _mm256_shuffle_epi8(input_simd_2, shape_mask2);
    let shapes_2_half_2 = _mm256_shuffle_epi8(input_simd_4, shape_mask2);

    // get two [ op x 8, my x 8, op x 8, my x 8 ]
    let shapes_1 = _mm256_add_epi8(shapes_1_half_1, shapes_1_half_2);
    let shapes_2 = _mm256_add_epi8(shapes_2_half_1, shapes_2_half_2);

    // get [ op x 32 ] and [ my x 32 ]
    let op_shapes = _mm256_unpacklo_epi8(shapes_1, shapes_2);
    let my_shapes = _mm256_unpackhi_epi8(shapes_1, shapes_2);

    // let shape_score_offset = _mm256_set1_epi8(87);
    let shape_score = _mm256_sub_epi8(my_shapes, shape_score_offset);

    let outcome = _mm256_sub_epi8(my_shapes, op_shapes);
    // Draw:  23
    // Win:   24 | 21
    // Loss:  22 | 25

    // let outcome_score_offset = _mm256_set1_epi8(22);
    let outcome = _mm256_sub_epi8(outcome, outcome_score_offset);

    // These needs to be mapped:
    // Draw:  1       =>   3
    // Win:   2 | -1  =>   6
    // Loss:  0 | 3   =>   0

    // Conditionally move 2 in places of -1
    // let bad_win_value = _mm256_set1_epi8(-1);
    // let good_win_value = _mm256_set1_epi8(2);
    let bad_win_mask = _mm256_cmpeq_epi8(outcome, bad_win_value);
    let outcome = _mm256_blendv_epi8(outcome, good_win_value, bad_win_mask);

    // Conditionally move 0 in places of 3
    // let bad_loss_value = _mm256_set1_epi8(3);
    // let good_loss_value = _mm256_set1_epi8(0);
    let bad_loss_mask = _mm256_cmpeq_epi8(outcome, bad_loss_value);
    let outcome = _mm256_blendv_epi8(outcome, good_loss_value, bad_loss_mask);

    // outcome_score = outcome * 3 = outcome + outcome + outcome
    let outcome2 = _mm256_add_epi8(outcome, outcome);
    let outcome_score = _mm256_add_epi8(outcome, outcome2);

    // get [ score x 32 ]
    _mm256_add_epi8(outcome_score, shape_score)
}

#[inline(always)]
unsafe fn horizontal_sum_u16x16(score: __m256i) -> u64 {
    let a = _mm256_cvtepu16_epi32(_mm256_extracti128_si256::<0>(score));
    let b = _mm256_cvtepu16_epi32(_mm256_extracti128_si256::<1>(score));
    let score = _mm256_add_epi32(a, b);
    let a = _mm256_cvtepu32_epi64(_mm256_extracti128_si256::<0>(score));
    let b = _mm256_cvtepu32_epi64(_mm256_extracti128_si256::<1>(score));
    let score = _mm256_add_epi64(a, b);
    let a = _mm256_extracti128_si256::<0>(score);
    let b = _mm256_extracti128_si256::<1>(score);
    let score = _mm_add_epi64(a, b);
    let mut buffer = [0u64; 2];
    _mm_storeu_si128(buffer.as_mut_ptr() as _, score);
    buffer[0] + buffer[1]
}

#[inline(always)]
unsafe fn hadd_get_u16x16_from_u8x32(u8x32: __m256i) -> __m256i {
    let a = _mm256_cvtepu8_epi16(_mm256_extracti128_si256::<0>(u8x32));
    let b = _mm256_cvtepu8_epi16(_mm256_extracti128_si256::<1>(u8x32));
    _mm256_add_epi16(a, b)
}

fn run2(_input: &[u8]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(INPUT_BYTES), 17189)
    }

    #[test]
    #[ignore]
    fn test2() {
        assert_eq!(run2(INPUT_BYTES), 13490)
    }
}
