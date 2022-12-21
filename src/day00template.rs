const INPUT: &[u8] = include_bytes!("../res/input00");

#[allow(unused)]
pub fn part1() {
    let answer = run1(INPUT);
    // assert_eq!(answer, todo!());
    println!("{answer}");
}

#[allow(unused)]
pub fn part2() {
    let answer = run2(INPUT);
    // assert_eq!(answer, todo!());
    println!("{answer}");
}

pub fn run1(input: &[u8]) -> u64 {
    let mut answer = 0u64;
    for line in input.trim_ascii_end().split(|&b| b == b'\n') {
    }
    answer
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

    const EXAMPLE: &[u8] = include_bytes!("../res/example00");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), todo!())
    }

    #[test]
    #[ignore]
    fn test2() {
        assert_eq!(run2(EXAMPLE), todo!())
    }
}
