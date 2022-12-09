use ahash::{HashSet, HashSetExt};
use atoi::FromRadix10;

const INPUT: &[u8] = include_bytes!("../res/input09");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 6090
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 2566
}

fn move_head(head: (i16, i16), direction: u8) -> (i16, i16) {
    match direction {
        b'U' => (head.0 - 1, head.1),
        b'D' => (head.0 + 1, head.1),
        b'L' => (head.0, head.1 - 1),
        b'R' => (head.0, head.1 + 1),
        _ => unreachable!()
    }
}

fn move_tail(head: (i16, i16), tail: (i16, i16)) -> (i16, i16) {
    let v_diff = tail.0 - head.0;
    let h_diff = tail.1 - head.1;

    if v_diff.abs() <= 1 && h_diff.abs() <= 1 {
        tail
    } else {
        (tail.0 - v_diff.signum(), tail.1 - h_diff.signum())
    }
}

pub fn run1(input: &[u8]) -> u64 {
    let mut visited: HashSet<(i16, i16)> = HashSet::new();

    //             verti, horiz
    let mut head = (0i16, 0i16);
    let mut tail = (0i16, 0i16);

    visited.insert(tail);

    for line in input.split(|&b| b == b'\n') {
        if line.is_empty() {
            break;
        };

        let direction = line[0];
        let count = u8::from_radix_10(&line[2..]).0;

        for _ in 0..count {
            head = move_head(head, direction);
            tail = move_tail(head, tail);
            visited.insert(tail);
        }
    }
    visited.len() as u64
}

pub fn run2(input: &[u8]) -> u64 {
    let mut visited: HashSet<(i16, i16)> = HashSet::new();

    //              verti, horiz
    let mut rope = [(0i16, 0i16); 10];

    visited.insert(rope[9]);

    for line in input.split(|&b| b == b'\n') {
        if line.is_empty() {
            break;
        };

        let direction = line[0];
        let count = u8::from_radix_10(&line[2..]).0;

        for _ in 0..count {
            rope[0] = move_head(rope[0], direction);
            for knot in 1..10 {
                rope[knot] = move_tail(rope[knot - 1], rope[knot]);
            }
            visited.insert(rope[9]);
        }
    }
    visited.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &[u8] = include_bytes!("../res/example09p1");
    const EXAMPLE_2: &[u8] = include_bytes!("../res/example09p2");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE_1), 13)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE_2), 36)
    }
}
