const INPUT: &[u8] = include_bytes!("../res/input00");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 
}

pub fn run1(input: &[u8]) -> u64 {
    let mut answer = 0u64;
    for line in input.split(|&b| b == b'\n') {
        if line.is_empty() {
            break;
        };
    }
    answer
}

pub fn run2(input: &[u8]) -> u64 {
    let mut answer = 0u64;
    for line in input.split(|&b| b == b'\n') {
        if line.is_empty() {
            break;
        };
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
    fn test2() {
        assert_eq!(run2(EXAMPLE), todo!())
    }
}
