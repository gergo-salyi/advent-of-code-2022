use smallvec::SmallVec;

const INPUT: &[u8] = include_bytes!("../res/input03");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 8243
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 2631
}

fn priority(char_code: u8) -> u8 {
    if char_code < 91 {
        char_code - 38
    } else {
        char_code - 96
    }
}

fn run1(input: &[u8]) -> u64 {
    let mut sum = 0u64;
    for line in input.split(|&c| c == b'\n') {
        let half_len = line.len() / 2;
        if half_len == 0 {
            break;
        }
        let first = &line[0..half_len];
        let second = &line[half_len..];
        let mut first: SmallVec<[u8; 24]> = SmallVec::from_slice(first);
        first.sort_unstable();
        debug_assert!(!first.spilled());
        for item in second {
            if first.binary_search(item).is_ok() {
                sum += priority(*item) as u64;
                break;
            }
        }
    }
    sum
}

fn run2(input: &[u8]) -> u64 {
    let mut sum = 0u64;
    let mut lines = input.split(|&c| c == b'\n');
    loop {
        let Some(line1) = lines.next() else { break };
        if line1.is_empty() {
            break;
        };
        let line2 = lines.next().unwrap();
        let line3 = lines.next().unwrap();
        let mut line1: SmallVec<[u8; 64]> = SmallVec::from_slice(line1);
        let mut line2: SmallVec<[u8; 64]> = SmallVec::from_slice(line2);
        line1.sort_unstable();
        line2.sort_unstable();
        for item in line3 {
            if line1.binary_search(item).is_ok()
                && line2.binary_search(item).is_ok()
            {
                sum += priority(*item) as u64;
                break;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example03");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 157)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 70)
    }
}
