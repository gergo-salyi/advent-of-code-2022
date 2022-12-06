const INPUT: &[u8] = include_bytes!("../res/input06");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 1544
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 2145
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

pub fn run2(input: &[u8]) -> u64 {
    'loop_characters: for i in 13..input.len() {
        let last_characters = &input[i - 13..=i];
        for j in 1..14 {
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example06");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 7)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 19)
    }
}
