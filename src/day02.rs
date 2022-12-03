const INPUT: &str = include_str!("../res/input02");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 17189
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 13490
}

const ROCK: i8 = 1;
const PAPER: i8 = 2;
const SCISSORS: i8 = 3;

fn score(opponent_shape: i8, my_shape: i8) -> u8 {
    let outcome = match my_shape - opponent_shape {
        0 => 3u8,      // draw
        1 | -2 => 6u8, // win
        -1 | 2 => 0u8, // loss
        _ => unreachable!(),
    };
    outcome + my_shape as u8
}

pub fn run1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let opponent_shape = match line.as_bytes()[0] {
                b'A' => ROCK,
                b'B' => PAPER,
                b'C' => SCISSORS,
                _ => panic!(),
            };
            let my_shape = match line.as_bytes()[2] {
                b'X' => ROCK,
                b'Y' => PAPER,
                b'Z' => SCISSORS,
                _ => panic!(),
            };
            score(opponent_shape, my_shape) as u64
        })
        .sum()
}

fn run2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let opponent_shape = match line.as_bytes()[0] {
                b'A' => ROCK,
                b'B' => PAPER,
                b'C' => SCISSORS,
                _ => panic!(),
            };

            let my_shape = match line.as_bytes()[2] {
                // need to lose
                b'X' => match opponent_shape {
                    ROCK => SCISSORS,
                    PAPER => ROCK,
                    SCISSORS => PAPER,
                    _ => unreachable!(),
                },

                // need to draw
                b'Y' => opponent_shape,

                // need to win
                b'Z' => match opponent_shape {
                    ROCK => PAPER,
                    PAPER => SCISSORS,
                    SCISSORS => ROCK,
                    _ => unreachable!(),
                },
                _ => panic!(),
            };

            score(opponent_shape, my_shape) as u64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../res/example02");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 15)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 12)
    }
}
