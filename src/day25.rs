const INPUT: &[u8] = include_bytes!("../res/input25");

#[allow(unused)]
pub fn part1() {
    let answer = run1(INPUT);
    assert_eq!(&answer, b"2=000=22-0-102=-1001");
    println!("{}", String::from_utf8(answer).unwrap());
}

#[allow(unused)]
pub fn part2() {
    let answer = run2(INPUT);
    // assert_eq!(answer, todo!());
    println!("{answer}");
}

const POW5: [i64; 28] = {
    let mut arr = [0i64; 28];
    let mut i = 0usize;
    while i < arr.len() {
        arr[i] = 5i64.pow(i as _);
        i += 1;
    }
    arr
};

fn snafu_from_int(mut num: u64) -> Vec<u8> {
    let mut snafu_decimals = Vec::with_capacity(29); // log_5( 2^64 ).ceil() + 1
    let mut carry = false;

    loop {
        let mut base5_decimal = (num % 5) as u8;
        if carry {
            base5_decimal += 1;
            carry = false;
        }
        let snafu_decimal = match base5_decimal {
            3 => {
                carry = true;
                b'='
            }
            4 => {
                carry = true;
                b'-'
            }
            5 => {
                carry = true;
                b'0'
            }
            d => d + b'0'
        };
        snafu_decimals.push(snafu_decimal);
        num /= 5;
        if num == 0 {
            break;
        }
    }

    if carry == true {
        snafu_decimals.push(b'1');
    }

    snafu_decimals.reverse();

    snafu_decimals
}

pub fn run1(input: &[u8]) -> Vec<u8> {
    let mut sum = 0i64;
    for line in input.trim_ascii_end().split(|&b| b == b'\n') {
        let mut decimal_number = 0i64;
        for (decimal_pos, &snafu_decimal) in line.iter().rev().enumerate() {
            decimal_number += POW5[decimal_pos] * match snafu_decimal {
                b'-' => -1i64,
                b'=' => -2i64,
                decimal => (decimal - b'0') as i64
            };
        }
        sum += decimal_number;
    }
    snafu_from_int(sum as u64)
}

pub fn run2(input: &[u8]) -> u64 {
    let mut answer = 0u64;
    for line in input.trim_ascii_end().split(|&b| b == b'\n') {
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example25");

    #[test]
    fn test_snafu_from_int() {
        assert_eq!(snafu_from_int(314159265), b"1121-1110-1=0")
    }

    #[test]
    fn test1() {
        assert_eq!(&run1(EXAMPLE), b"2=-1=0")
    }

    #[test]
    #[ignore]
    fn test2() {
        assert_eq!(run2(EXAMPLE), todo!())
    }
}
