use std::cmp::{max, min};

use atoi::FromRadix10;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const INPUT: &[u8] = include_bytes!("../res/input14");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 892
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 27155
}

fn decimal(input: &[u8]) -> IResult<&[u8], usize> {
    map(digit1, |b: &[u8]| usize::from_radix_10(b).0)(input)
}

fn parse(input: &[u8]) -> IResult<&[u8], Vec<Vec<(usize, usize)>>> {
    let pair = separated_pair(decimal, char(','), decimal);
    let rock = separated_list1(tag(" -> "), pair);
    separated_list1(char('\n'), rock)(input)
}

pub fn run1(input: &[u8]) -> u64 {

    let rocks = parse(input).unwrap().1;

    // x = 480..540, y = 0..175
    let mut cave = vec![vec![0u8; 175]; 60];

    for rock in rocks.iter() {
        for (&(x0, y0), &(x1, y1)) in rock.iter().zip(rock.iter().skip(1)) {
            if x0 == x1 {
                for i in min(y0, y1)..=max(y0, y1) {
                    cave[x0 - 480][i] = 1;
                }
            } else if y0 == y1 {
                for i in min(x0, x1)..=max(x0, x1) {
                    cave[i - 480][y0] = 1;
                }
            } else {
                panic!()
            }
        };
    }

    for i in 1.. {
        let (mut sand_x, mut sand_y) = (20, 0);
        loop {
            if cave[sand_x][sand_y + 1] == 0 {

                sand_y += 1;

            } else if cave[sand_x - 1][sand_y + 1] == 0 {

                sand_x -= 1;
                sand_y += 1;

            } else if cave[sand_x + 1][sand_y + 1] == 0 {

                sand_x += 1;
                sand_y += 1;
                    
            } else {

                cave[sand_x][sand_y] = 1;
                break;
            }

            if sand_y == 174 {
                return i as u64 - 1;
            }
        }
    }

    panic!();
}

pub fn run2(input: &[u8]) -> u64 {
    let mut rocks = parse(input).unwrap().1;

    // TODO: fix hardcoded floor
    rocks.push(vec![(0, 170), (999, 170)]);

    // x = 480..540, y = 0..175
    let mut cave = vec![vec![0u8; 175]; 1000];

    for rock in rocks.iter() {
        for (&(x0, y0), &(x1, y1)) in rock.iter().zip(rock.iter().skip(1)) {
            if x0 == x1 {
                for i in min(y0, y1)..=max(y0, y1) {
                    cave[x0][i] = 1;
                }
            } else if y0 == y1 {
                for i in min(x0, x1)..=max(x0, x1) {
                    cave[i][y0] = 1;
                }
            } else {
                panic!()
            }
        };
    }

    for i in 1.. {
        let (mut sand_x, mut sand_y) = (500, 0);
        loop {
            if cave[sand_x][sand_y + 1] == 0 {

                sand_y += 1;

            } else if cave[sand_x - 1][sand_y + 1] == 0 {

                sand_x -= 1;
                sand_y += 1;

            } else if cave[sand_x + 1][sand_y + 1] == 0 {

                sand_x += 1;
                sand_y += 1;

            } else {

                if sand_x == 500 && sand_y == 0 {

                    return i as u64;

                } else {

                    cave[sand_x][sand_y] = 1;
                    break;

                }
            }
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example14");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 24)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 93)
    }
}
