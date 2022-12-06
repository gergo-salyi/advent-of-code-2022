use atoi::FromRadix10;
use nom::{
    bytes::complete::{take, take_until1},
    character::complete::digit1,
    IResult,
};

const INPUT: &[u8] = include_bytes!("../res/input05");

/*
[N]         [C]     [Z]
[Q] [G]     [V]     [S]         [V]
[L] [C]     [M]     [T]     [W] [L]
[S] [H]     [L]     [C] [D] [H] [S]
[C] [V] [F] [D]     [D] [B] [Q] [F]
[Z] [T] [Z] [T] [C] [J] [G] [S] [Q]
[P] [P] [C] [W] [W] [F] [W] [J] [C]
[T] [L] [D] [G] [P] [P] [V] [N] [R]
 1   2   3   4   5   6   7   8   9
*/

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // SVFDLGLWV
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // DCVTCVPCL
}

fn decimal(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, decimal) = digit1(input)?;
    Ok((input, u8::from_radix_10(decimal).0))
}

fn step_from_line(input: &[u8]) -> IResult<&[u8], (u8, u8, u8)> {
    // "move "
    let (input, _) = take(5usize)(input)?;
    let (input, n) = decimal(input)?;
    // " from "
    let (input, _) = take(6usize)(input)?;
    let (input, s) = decimal(input)?;
    // " to "
    let (input, _) = take(4usize)(input)?;
    let (input, d) = decimal(input)?;

    Ok((input, (n, s - 1, d - 1)))
}

fn take_init_layout(input: &[u8]) -> IResult<&[u8], Vec<Vec<u8>>> {
    let (input, init_layout) = take_until1(b"\n\n".as_slice())(input)?;
    let (procedure, _) = take(2usize)(input)?;

    let item_number = init_layout
        .iter()
        // Count the capital letters
        .filter(|&b| (65..=90).contains(b))
        .count();

    let mut stacks: Vec<Vec<u8>> = vec![Vec::with_capacity(item_number); 9];

    let init_layout_lines = init_layout.split(|&b| b == b'\n').rev().skip(1);

    for line in init_layout_lines {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let Some(&byte) = line.get(4 * i + 1) else { break };
            if byte != b' ' {
                stack.push(byte)
            }
        }
    }

    Ok((procedure, stacks))
}

fn answer(stacks: &Vec<Vec<u8>>) -> String {
    let mut answer = Vec::with_capacity(stacks.len());
    for stack in stacks {
        if let Some(&byte) = stack.last() {
            answer.push(byte)
        }
    }
    String::from_utf8(answer).unwrap()
}

pub fn run1(input: &[u8]) -> String {
    let (procedure, mut stacks) = take_init_layout(input).unwrap();

    for line in procedure.split(|&byte| byte == b'\n') {
        if line.is_empty() {
            break;
        }
        let (n, s, d) = step_from_line(line).unwrap().1;
        for _ in 0..n {
            let item = stacks[s as usize].pop().unwrap();
            stacks[d as usize].push(item);
        }
    }

    answer(&stacks)
}

pub fn run2(input: &[u8]) -> String {
    let (procedure, mut stacks) = take_init_layout(input).unwrap();

    for line in procedure.split(|&byte| byte == b'\n') {
        if line.is_empty() {
            break;
        }
        let (n, s, d) = step_from_line(line).unwrap().1;
        let source_new_len = stacks[s as usize].len() - n as usize;
        for i in 0..n {
            let item = stacks[s as usize][source_new_len + i as usize];
            stacks[d as usize].push(item);
        }
        stacks[s as usize].truncate(source_new_len);
    }

    answer(&stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example05");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE).as_str(), "CMZ")
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE).as_str(), "MCD")
    }
}
