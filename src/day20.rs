use atoi::FromRadix10Signed;

use std::cmp::Ordering;

const INPUT: &[u8] = include_bytes!("../res/input20");

#[allow(unused)]
pub fn part1() {
    let answer = run1(INPUT);
    assert_eq!(answer, 5904);
    println!("{answer}");
}

#[allow(unused)]
pub fn part2() {
    let answer = run2(INPUT);
    assert_eq!(answer, 8332585833851);
    println!("{answer}");
}

pub fn run1(input: &[u8]) -> i64 {
    let mut numbers = Vec::with_capacity(input.len() / 5);

    for line in input.trim_ascii_end().split(|&b| b == b'\n') {
        let number = i16::from_radix_10_signed(line).0;
        numbers.push(number);
    }

    let numbers = numbers;
    let len = numbers.len() as i16;

    let mut seqs: Vec<i16> = (0..len).collect();
    let mut seq0 = 0i16;

    for seq in 0..len {
        let n = numbers[seq as usize];

        if n == 0 {
            seq0 = seq;
            continue;
        }

        let idx = seqs.iter().position(|&e| e == seq).unwrap();

        let idx_new = (idx as i16 + n).rem_euclid(len - 1) as usize;

        match idx_new.cmp(&idx) {
            Ordering::Greater => {
                seqs.copy_within((idx + 1)..=idx_new, idx);
                seqs[idx_new] = seq;
            }
            Ordering::Less => {
                seqs.copy_within(idx_new..=(idx - 1), idx_new + 1);
                seqs[idx_new] = seq;
            }
            Ordering::Equal => {
                continue;
            }
        }
    }

    let len = len as usize;

    let i = seqs.iter().position(|&e| e == seq0).unwrap();
    let n1 = numbers[seqs[(i + 1000) % len] as usize];
    let n2 = numbers[seqs[(i + 2000) % len] as usize];
    let n3 = numbers[seqs[(i + 3000) % len] as usize];

    (n1 + n2 + n3) as i64
}

pub fn run2(input: &[u8]) -> i64 {
    let mut numbers = Vec::with_capacity(input.len() / 5);

    for line in input.trim_ascii_end().split(|&b| b == b'\n') {
        let number = i64::from_radix_10_signed(line).0 * 811589153;
        numbers.push(number);
    }

    let numbers = numbers;
    let len = numbers.len() as i16;

    let mut seqs: Vec<i16> = (0..len).collect();
    let mut seq0 = 0i16;

    for _ in 0..10 {
        for seq in 0..len {
            let n = numbers[seq as usize];

            if n == 0 {
                seq0 = seq;
                continue;
            }

            let idx = seqs.iter().position(|&e| e == seq).unwrap();

            let idx_new = (idx as i64 + n).rem_euclid(len as i64 - 1) as usize;

            match idx_new.cmp(&idx) {
                Ordering::Greater => {
                    seqs.copy_within((idx + 1)..=idx_new, idx);
                    seqs[idx_new] = seq;
                }
                Ordering::Less => {
                    seqs.copy_within(idx_new..=(idx - 1), idx_new + 1);
                    seqs[idx_new] = seq;
                }
                Ordering::Equal => {
                    continue;
                }
            }
        }
    }

    let len = len as usize;

    let i = seqs.iter().position(|&e| e == seq0).unwrap();
    let n1 = numbers[seqs[(i + 1000) % len] as usize];
    let n2 = numbers[seqs[(i + 2000) % len] as usize];
    let n3 = numbers[seqs[(i + 3000) % len] as usize];

    n1 + n2 + n3
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example20");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 3)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 1623178306)
    }
}
