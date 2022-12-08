use core::arch::x86_64::{
    __m128i,
    _mm_broadcastb_epi8,
    _mm_cmpeq_epi8,
    _mm_loadu_si128,
    _mm_setr_epi8,
    _mm_shuffle_epi8,
    _mm_storeu_si128,
    _mm_testz_si128,
    _mm_and_si128,
    _mm_movemask_epi8,
    _mm_tzcnt_32,
    _mm_insert_epi8,
};
/*
use core::arch::x86_64::{
    __m256i,
    _mm256_broadcastb_epi8,
    _mm256_cmpeq_epi8,
    _mm256_loadu_si256,
    _mm256_loadu2_m128i,
    _mm256_setr_epi8,
    _mm256_shuffle_epi8,
    _mm256_storeu_si256,
    _mm256_testz_si256,
};
*/

const INPUT: &[u8] = include_bytes!("../res/input06");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 1544
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 2145
}

#[allow(unused)]
unsafe fn dbg_m128i(packed: __m128i) {
    let mut byte_array = [0u8; 16];
    _mm_storeu_si128(byte_array.as_mut_ptr() as _, packed);
    eprintln!("{:?}", byte_array);
}

#[allow(unused)]
unsafe fn dbg_m128i_chars(packed: __m128i) {
    let mut byte_array = [0u8; 16];
    _mm_storeu_si128(byte_array.as_mut_ptr() as _, packed);
    eprintln!("{:?}", byte_array.iter().map(
        |char_code| {
            char::from_u32(*char_code as u32).unwrap_or('.')
        }
    ).collect::<Vec<_>>());
}

pub fn run1(input: &[u8]) -> u64 {
    'loop_characters: for i in 3..input.len() {
        let last_characters = &input[i - 3..=i];
        for j in 1..4 {
            for k in 0..j {
                if last_characters[j] == last_characters[k] {
                    continue 'loop_characters;
                }
            }
        }
        return (i + 1) as u64;
    }
    0
}

/*
// https://www.reddit.com/r/adventofcode/comments/zdw0u6/comment/iz3v100/
// https://github.com/SvetlinZarev/advent-of-code/blob/main/2022/aoc-day-06/src/lib.rs
const ASCII_LEN: usize = (b'z' - b'a' + 1) as usize;
pub fn sliding_window(input: &[u8]) -> usize {
    let window = 14;
    if input.len() < window {
        panic!("Input is too short: {}", input.len());
    }

    let mut seen = [0u32; ASCII_LEN];
    let mut uniq = 0;

    // Seed the algorithm with the first `window` bytes
    for idx in 0..window {
        let ch = (input[idx] - b'a') as usize;

        seen[ch] += 1;
        if seen[ch] == 1 {
            uniq += 1;
        }
    }

    // Fast path: check if the first `window` bytes are the solution
    if uniq == window {
        return window;
    }

    input
        .windows(window + 1)
        .enumerate()
        .find(|&(_idx, w)| {
            let ch = (w[0] - b'a') as usize;
            seen[ch] -= 1;
            if seen[ch] == 0 {
                uniq -= 1;
            }

            let ch = (w[window] - b'a') as usize;
            seen[ch] += 1;
            if seen[ch] == 1 {
                uniq += 1;
            }

            uniq == window
        })
        // convert the window index to character index
        .map(|(pos, w)| pos + w.len())
        .expect("no answer")
}
*/

// #[no_mangle]
// #[inline(never)]
pub fn run2(input: &[u8]) -> u64 {
    unsafe {run2_inner(input) }
}

unsafe fn run2_inner(input: &[u8]) -> u64 {
    test_slices4(input.as_ptr())
}
/*
unsafe fn test_slices(input: *const u8) -> u64 {
    
    // ***** Constants *****

    // Shuffle masks to rotate lower 14 i8 elements
    let shuffle_rotate_left_lo_14_mask = _mm_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8, 
        9, 10, 11, 12, 13, 0, 14, 15);
    
    let shuffle_rotate_right_lo_14_mask = _mm_setr_epi8(13, 0, 1, 2, 3, 4, 5, 6, 
        7, 8, 9, 10, 11, 12, 14, 15);

    // self|    care      | dont't care      
    // [ 00, ff, ff, ff, ... 00, 00 ]
    #[allow(overflowing_literals)]
    let mask_not_self_and_within_14 = _mm_setr_epi8(
        0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0
    );

    // ***** Current slice vars *****

    let mut offset_1 = 0isize;

    // [  A,  B,  C,  A, ...  b,  a ]
    let mut chars_a = _mm_loadu_si128(input as _);


    // ***** Current cycle (n) vars *****

    let mut cycle_1 = 0u8;

    // self|    care      | dont't care      
    // [ 00, ff, ff, ff, ... 00, 00 ]
    let mut mask_not_self_and_within_14_n = mask_not_self_and_within_14;
    
    let mut char_n_in_lowest_a = chars_a;
    
    // [  A,  A,  A,  A, ...  A,  A ]
    let mut char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);


    loop {

        let is_same_as_char_n_a = _mm_cmpeq_epi8(chars_a, char_n_a);

        let chars_all_different_for_now_a = _mm_testz_si128(
            is_same_as_char_n_a, 
            mask_not_self_and_within_14_n
        );

        if chars_all_different_for_now_a == 0 {

            // This ain't the slice slice you are looking for..
            // load the next slice and reset the state
            // We advande by cycle_1 + 1 elements
            offset_1 += (cycle_1 + 1) as isize;

            // TODO: check for buffer overread
            // if offset_1 >= max_offest {
            //     return 0
            // }
            chars_a = _mm_loadu_si128(input.offset(offset_1) as _);

            cycle_1 = 0u8;
            mask_not_self_and_within_14_n = mask_not_self_and_within_14;
            char_n_in_lowest_a = chars_a;
            char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);

        } else {
            // Cycle goes 0..=13 but the last element was already
            // tested if a duplicates of any other
            if cycle_1 == (13 - 1) {
                // this is the one!
                return (offset_1 + 14) as u64;
            }
            // We are not done eith this slice yet
            // prepare testing the next character
            cycle_1 += 1;
            mask_not_self_and_within_14_n = _mm_shuffle_epi8(
                mask_not_self_and_within_14_n, 
                shuffle_rotate_right_lo_14_mask
            );
            char_n_in_lowest_a = _mm_shuffle_epi8(
                char_n_in_lowest_a, 
                shuffle_rotate_left_lo_14_mask
            );
            char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);
        }
    }
}
*/

unsafe fn test_slices4(input: *const u8) -> u64 {
    
    // ***** Constants *****

    // Shuffle masks to rotate lower 14 i8 elements
    let shuffle_rotate_left_lo_14_mask = _mm_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8, 
        9, 10, 11, 12, 13, 0, 14, 15);
    
    let shuffle_rotate_right_lo_14_mask = _mm_setr_epi8(13, 0, 1, 2, 3, 4, 5, 6, 
        7, 8, 9, 10, 11, 12, 14, 15);

    // self|    care      | dont't care      
    // [ 00, ff, ff, ff, ... 00, 00 ]
    #[allow(overflowing_literals)]
    let mask_not_self_and_within_14 = _mm_setr_epi8(
        0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0
    );

    // ***** Current slice vars *****

    let mut offset_1 = 0isize;

    // [  A,  B,  C,  A, ...  b,  a ]
    let mut chars_a = _mm_loadu_si128(input as _);


    // ***** Current cycle (n) vars *****

    let mut cycle_1 = 0u8;

    let mut char_n_in_lowest_a = chars_a;
    
    // [  A,  A,  A,  A, ...  A,  A ]
    let mut char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);


    loop {

        let is_same_as_char_n_a = _mm_cmpeq_epi8(char_n_in_lowest_a, char_n_a);

        let chars_all_different_for_now_a = _mm_testz_si128(
            is_same_as_char_n_a, 
            mask_not_self_and_within_14
        );

        if chars_all_different_for_now_a == 0 {

            // This ain't the slice slice you are looking for..
            // load the next slice and reset the state
            // We advande by cycle_1 + 1 elements
            offset_1 += (cycle_1 + 1) as isize;

            // TODO: check for buffer overread
            // if offset_1 >= max_offest {
            //     return 0
            // }
            chars_a = _mm_loadu_si128(input.offset(offset_1) as _);

            cycle_1 = 0u8;
            char_n_in_lowest_a = chars_a;
            char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);

        } else {
            // Cycle goes 0..=13 but the last element was already
            // tested if a duplicates of any other
            if cycle_1 == (13 - 1) {
                // this is the one!
                return (offset_1 + 14) as u64;
            }
            // We are not done eith this slice yet
            // prepare testing the next character
            cycle_1 += 1;
            char_n_in_lowest_a = _mm_shuffle_epi8(
                char_n_in_lowest_a, 
                shuffle_rotate_left_lo_14_mask
            );
            char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);
        }
    }
}

/*

// This is actually SLOWER...
// Idea was to insert one new element on slice reload instead of _mm_loadu
unsafe fn test_slices3(input: *const u8) -> u64 {
    
    // ***** Constants *****

    // Shuffle masks to rotate lower 14 i8 elements
    let shuffle_rotate_left_lo_14_mask = _mm_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8, 
        9, 10, 11, 12, 13, 0, 14, 15);
    
    let shuffle_rotate_right_lo_14_mask = _mm_setr_epi8(13, 0, 1, 2, 3, 4, 5, 6, 
        7, 8, 9, 10, 11, 12, 14, 15);

    // self|    care      | dont't care      
    // [ 00, ff, ff, ff, ... 00, 00 ]
    #[allow(overflowing_literals)]
    let mask_not_self_and_within_14 = _mm_setr_epi8(
        0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0
    );

    // ***** Current slice vars *****

    let mut offset_1 = 14isize;

    // [  A,  B,  C,  A, ...  b,  a ]
    let mut chars_a = _mm_loadu_si128(input as _);


    // ***** Current cycle (n) vars *****

    let mut cycle_1 = 0u8;

    // self|    care      | dont't care      
    // [ 00, ff, ff, ff, ... 00, 00 ]
    let mut mask_not_self_and_within_14_n = mask_not_self_and_within_14;
    
    let mut char_n_in_lowest_a = chars_a;
    
    // [  A,  A,  A,  A, ...  A,  A ]
    let mut char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);


    loop {

        let is_same_as_char_n_a = _mm_cmpeq_epi8(chars_a, char_n_a);

        let chars_all_different_for_now_a = _mm_testz_si128(
            is_same_as_char_n_a, 
            mask_not_self_and_within_14_n
        );

        if chars_all_different_for_now_a == 0 {

            // This ain't the slice slice you are looking for..
            // load the next slice and reset the state
            // We advande by cycle_1 + 1 elements
            //- offset_1 += (cycle_1 + 1) as isize;

            // TODO: check for buffer overread
            // if offset_1 >= max_offest {
            //     return 0
            // }
            //- chars_a = _mm_loadu_si128(input.offset(offset_1) as _);
            // dbg_m128i_chars(chars_a);
            chars_a = _mm_insert_epi8(
                chars_a,
                *(input.offset(offset_1) as *const i32),
                0
            );
            // dbg_m128i_chars(chars_a);
            chars_a = _mm_shuffle_epi8(
                chars_a, 
                shuffle_rotate_left_lo_14_mask
            );
            // dbg_m128i_chars(chars_a);

            offset_1 += 1 as isize;

            cycle_1 = 0u8;
            mask_not_self_and_within_14_n = mask_not_self_and_within_14;
            char_n_in_lowest_a = chars_a;
            char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);

        } else {
            // Cycle goes 0..=13 but the last element was already
            // tested if a duplicates of any other
            if cycle_1 == (13 - 1) {
                // this is the one!
                return offset_1 as u64;
            }
            // We are not done eith this slice yet
            // prepare testing the next character
            cycle_1 += 1;
            mask_not_self_and_within_14_n = _mm_shuffle_epi8(
                mask_not_self_and_within_14_n, 
                shuffle_rotate_right_lo_14_mask
            );
            char_n_in_lowest_a = _mm_shuffle_epi8(
                char_n_in_lowest_a, 
                shuffle_rotate_left_lo_14_mask
            );
            char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);
        }
    }
}

// This is actually SLOWER...
// Modify algo to start from the high end of the slice
unsafe fn test_slices2(input: *const u8) -> u64 {
    
    // ***** Constants *****

    // Shuffle masks to rotate lower 14 i8 elements
    let shuffle_rotate_left_lo_14_mask = _mm_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8, 
        9, 10, 11, 12, 13, 0, 14, 15);
    
    let shuffle_rotate_right_lo_14_mask = _mm_setr_epi8(13, 0, 1, 2, 3, 4, 5, 6, 
        7, 8, 9, 10, 11, 12, 14, 15);

    // self|    care      | dont't care      
    // [ 00, ff, ff, ff, ... 00, 00 ]
    #[allow(overflowing_literals)]
    let mask_not_self_and_within_14 = _mm_setr_epi8(
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0
    );

    // ***** Current slice vars *****

    let mut offset_1 = 0isize;

    // [  A,  B,  C,  A, ...  b,  a ]
    let mut chars_a = _mm_loadu_si128(input as _);
    

    // ***** Current cycle (n) vars *****

    let mut cycle_1 = 0u8;

    // self|    care      | dont't care      
    // [ 00, ff, ff, ff, ... 00, 00 ]
    let mut mask_not_self_and_within_14_n = mask_not_self_and_within_14;
    
    let mut char_n_in_lowest_a = _mm_shuffle_epi8(
        chars_a, 
        shuffle_rotate_right_lo_14_mask
    );
    
    // [  A,  A,  A,  A, ...  A,  A ]
    let mut char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);

    loop {

        let is_same_as_char_n_a = _mm_cmpeq_epi8(chars_a, char_n_a);

        let matched_mask = _mm_and_si128(
            is_same_as_char_n_a, 
            mask_not_self_and_within_14_n
        );
        let compressed = _mm_movemask_epi8(matched_mask);

        if compressed != 0 {

            // This ain't the slice slice you are looking for..
            // load the next slice and reset the state

            // Advance the slice to cut off the leftmost element that was
            // part of the found same char pair
            let elem_matched_with = _mm_tzcnt_32(compressed as u32);
            offset_1 += (elem_matched_with + 1) as isize;

            // TODO: check for buffer overread
            // if offset_1 >= max_offest {
            //     return 0
            // }
            chars_a = _mm_loadu_si128(input.offset(offset_1) as _);

            cycle_1 = 0u8;
            mask_not_self_and_within_14_n = mask_not_self_and_within_14;
            //- char_n_in_lowest_a = chars_a;
            char_n_in_lowest_a = _mm_shuffle_epi8(
                chars_a, 
                shuffle_rotate_right_lo_14_mask
            );
            char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);

        } else {
            // Cycle goes 0..=13 but the last element was already
            // tested if a duplicates of any other
            if cycle_1 == (13 - 1) {
                // this is the one!
                return (offset_1 + 14) as u64;
            }
            // We are not done eith this slice yet
            // prepare testing the next character
            cycle_1 += 1;
            mask_not_self_and_within_14_n = _mm_shuffle_epi8(
                mask_not_self_and_within_14_n, 
                shuffle_rotate_left_lo_14_mask
            );
            char_n_in_lowest_a = _mm_shuffle_epi8(
                char_n_in_lowest_a, 
                shuffle_rotate_right_lo_14_mask
            );
            char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);
        }
    }
}
*/
/*
unsafe fn test_slices2(input: *const u8) -> u64 {
    
    // ***** Constants *****

    // Shuffle masks to rotate lower 14 i8 elements
    let shuffle_rotate_left_lo_14_mask = _mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8, 
        9, 10, 11, 12, 13, 0, 14, 15, 1, 2, 3, 4, 5, 6, 7, 8, 
        9, 10, 11, 12, 13, 0, 14, 15);
    
    let shuffle_rotate_right_lo_14_mask = _mm256_setr_epi8(13, 0, 1, 2, 3, 4, 5, 6, 
        7, 8, 9, 10, 11, 12, 14, 15, 1, 2, 3, 4, 5, 6, 7, 8, 
        9, 10, 11, 12, 13, 0, 14, 15);

    // self|    care      | dont't care      
    // [ 00, ff, ff, ff, ... 00, 00 ]
    #[allow(overflowing_literals)]
    let mask_not_self_and_within_14 = _mm256_setr_epi8(
        0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0,
        0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0
    );


    // ***** Current slice vars *****

    let mut offset_next = 2isize;
    let mut offset_a = 0isize;
    let mut offset_b = 1isize;
    // let mut offset_3 = 32isize;
    // let mut offset_4 = 48isize;
    // [  A,  B,  C,  A, ...  b,  a ]
    let mut chars = _mm256_loadu2_m128i(input as _, input.offset(1) as _);
    

    // ***** Current cycle (n) vars *****

    let mut cycle_a = 0u8;
    let mut cycle_b = 0u8;
    // let mut cycle_3 = 0u8;
    // let mut cycle_4 = 0u8;

    // self|    care      | dont't care      
    // [ 00, ff, ff, ff, ... 00, 00 ]
    #[allow(overflowing_literals)]
    let mut mask_not_self_and_within_14_n_a = mask_not_self_and_within_14;
    let mut mask_not_self_and_within_14_n_b = mask_not_self_and_within_14;
    
    let mut char_n_in_lowest_a = chars_a;
    let mut char_n_in_lowest_b = chars_b;
    
    // [  A,  A,  A,  A, ...  A,  A ]
    let mut char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);
    let mut char_n_b = _mm_broadcastb_epi8(char_n_in_lowest_b);


    loop {

        let is_same_as_char_n_a = _mm256_cmpeq_epi8(chars, char_n);

        let chars_all_different_for_now_a = _mm_testz_si128(
            is_same_as_char_n_a, 
            mask_not_self_and_within_14_n_a
        );
        let chars_all_different_for_now_b = _mm_testz_si128(
            is_same_as_char_n_b, 
            mask_not_self_and_within_14_n_b
        );

        if chars_all_different_for_now_a == 0 {
            // This ain't the slice slice you are looking for..
            // load the next slice and reset the state
            offset_a = offset_next;
            offset_next += 1;
            // if offset_1 >= max_offest {
            //     return 0
            // }
            chars_a = _mm_loadu_si128(input.offset(offset_a) as _);

            cycle_a = 0u8;
            mask_not_self_and_within_14_n_a = mask_not_self_and_within_14;
            char_n_in_lowest_a = chars_a;
            char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);

        } else {
            if cycle_a == 15 {
                // this is the one!
                return (offset_a + 14) as u64;
            }
            // prepare testing next character
            cycle_a += 1;
            mask_not_self_and_within_14_n_a = _mm_shuffle_epi8(
                mask_not_self_and_within_14_n_a, 
                shuffle_rotate_right_lo_14_mask
            );
            char_n_in_lowest_a = _mm_shuffle_epi8(
                char_n_in_lowest_a, 
                shuffle_rotate_left_lo_14_mask
            );
            char_n_a = _mm_broadcastb_epi8(char_n_in_lowest_a);
        }


        if chars_all_different_for_now_b == 0 {
            // This ain't the slice slice you are looking for..
            // load the next slice and reset the state
            offset_b = offset_next;
            offset_next += 1;
            // if offset_1 >= max_offest {
            //     return 0
            // }
            chars_b = _mm_loadu_si128(input.offset(offset_b) as _);

            cycle_b = 0u8;
            mask_not_self_and_within_14_n_b = mask_not_self_and_within_14;
            char_n_in_lowest_b = chars_b;
            char_n_b = _mm_broadcastb_epi8(char_n_in_lowest_b);

        } else {
            if cycle_b == 15 {
                // this is the one!
                return (offset_b + 14) as u64;
            }
            // prepare testing next character
            cycle_b += 1;
            mask_not_self_and_within_14_n_b = _mm_shuffle_epi8(
                mask_not_self_and_within_14_n_b, 
                shuffle_rotate_right_lo_14_mask
            );
            char_n_in_lowest_b = _mm_shuffle_epi8(
                char_n_in_lowest_b, 
                shuffle_rotate_left_lo_14_mask
            );
            char_n_b = _mm_broadcastb_epi8(char_n_in_lowest_b);
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example06");

    #[test]
    #[ignore]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 7)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 19)
    }
    /*
    #[test]
    fn test_sliding_window() {
        assert_eq!(sliding_window(EXAMPLE), 19)
    }
    */
}
