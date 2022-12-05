use atoi::FromRadix10;
use nom::{
    bytes::complete::{take, take_until1},
    character::complete::{char, digit1},
    multi::fold_many1,
    sequence::separated_pair,
    IResult,
};

const INPUT: &[u8] = include_bytes!("../res/input04");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 490
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 921
}

fn parse_decimal(decimal: &[u8]) -> u8 {
    u8::from_radix_10(decimal).0
}

fn range(input: &[u8]) -> IResult<&[u8], (u8, u8)> {
    let (input, (d0, d1)) = separated_pair(digit1, char('-'), digit1)(input)?;
    Ok((input, (parse_decimal(d0), parse_decimal(d1))))
}

#[allow(clippy::type_complexity)]
fn pair(input: &[u8]) -> IResult<&[u8], ((u8, u8), (u8, u8))> {
    separated_pair(range, char(','), range)(input)
}

/*
fn parse(input: &[u8]) -> Vec<((u8, u8), (u8, u8))> {
    let (_, parsed) = separated_list1(char('\n'), pair)(input).unwrap();
    parsed
}
*/

fn is_pair_fully_contains(pair: ((u8, u8), (u8, u8))) -> bool {
    let a_contains_b = (pair.0 .0 <= pair.1 .0) & (pair.0 .1 >= pair.1 .1);
    let b_contains_a = (pair.0 .0 >= pair.1 .0) & (pair.0 .1 <= pair.1 .1);
    a_contains_b | b_contains_a
}

fn take_line_test_pair_fully_contains(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, line) = take_until1("\n")(input)?;
    let (input, _) = take(1usize)(input)?;
    Ok((input, is_pair_fully_contains(pair(line)?.1)))
}

fn is_pair_overlaps(pair: ((u8, u8), (u8, u8))) -> bool {
    let a_ends_before_b = pair.0 .1 < pair.1 .0;
    let b_ends_before_a = pair.0 .0 > pair.1 .1;
    let no_overlap = a_ends_before_b | b_ends_before_a;
    !no_overlap
}
fn take_line_test_pair_overlaps(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, line) = take_until1("\n")(input)?;
    let (input, _) = take(1usize)(input)?;
    Ok((input, is_pair_overlaps(pair(line)?.1)))
}

/*
pub fn run1old(input: &[u8]) -> u64 {
    let list = parse(input);
    let mut answer = 0u64;
    for pair in list {
        let a_containes_b = (pair.0 .0 <= pair.1 .0) & (pair.0 .1 >= pair.1 .1);
        let b_containes_a = (pair.0 .0 >= pair.1 .0) & (pair.0 .1 <= pair.1 .1);
        let fully_contains = a_containes_b | b_containes_a;
        if fully_contains {
            answer += 1
        };
    }
    answer
}
*/

pub fn run1(input: &[u8]) -> u64 {
    fold_many1(
        take_line_test_pair_fully_contains,
        || 0u64,
        |mut acc: u64, pair_fully_contains: bool| {
            if pair_fully_contains {
                acc += 1
            };
            acc
        },
    )(input)
    .unwrap()
    .1
}

/*
pub fn run2old(input: &[u8]) -> u64 {
    let list = parse(input);
    let mut answer = 0u64;
    for pair in list {
        let a_ends_before_b = pair.0 .1 < pair.1 .0;
        let b_ends_before_a = pair.0 .0 > pair.1 .1;
        let no_overlap = a_ends_before_b | b_ends_before_a;
        if !no_overlap {
            answer += 1
        };
    }
    answer
}
*/

pub fn run2(input: &[u8]) -> u64 {
    fold_many1(
        take_line_test_pair_overlaps,
        || 0u64,
        |mut acc: u64, pair_overlaps: bool| {
            if pair_overlaps {
                acc += 1
            };
            acc
        },
    )(input)
    .unwrap()
    .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example04");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 2)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 4)
    }
}
