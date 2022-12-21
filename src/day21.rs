use ahash::{HashMap, HashMapExt};
use nom::{
    branch::alt, bytes::complete::take, character::complete::u16 as nom_u16,
    combinator::map, IResult,
};

use std::collections::hash_map::Entry;

const INPUT: &[u8] = include_bytes!("../res/input21");

#[allow(unused)]
pub fn part1() {
    let answer = run1(INPUT);
    assert_eq!(answer, 268597611536314);
    println!("{answer}");
}

#[allow(unused)]
pub fn part2() {
    let answer = run2(INPUT);
    assert_eq!(answer, 3451534022348);
    println!("{answer}");
}

#[derive(Clone, Debug)]
struct Op {
    lhs_idx: u16,
    op: u8,
    rhs_idx: u16,
}

#[derive(Clone, Debug)]
enum Monkey {
    Num(i64),
    Op(Op),
}

fn number(monkey: u16, monkeys: &Vec<Monkey>) -> i64 {
    match &monkeys[monkey as usize] {
        Monkey::Num(n) => *n,
        Monkey::Op(o) => match o.op {
            b'+' => number(o.lhs_idx, monkeys) + number(o.rhs_idx, monkeys),
            b'-' => number(o.lhs_idx, monkeys) - number(o.rhs_idx, monkeys),
            b'*' => number(o.lhs_idx, monkeys) * number(o.rhs_idx, monkeys),
            b'/' => number(o.lhs_idx, monkeys) / number(o.rhs_idx, monkeys),
            _ => unreachable!(),
        },
    }
}

fn idx_from_name(
    name: [u8; 4],
    name_idx_map: &mut HashMap<[u8; 4], u16>,
) -> u16 {
    let next_id = name_idx_map.len() as u16;
    match name_idx_map.entry(name) {
        Entry::Occupied(hit) => *hit.get(),
        Entry::Vacant(miss) => *miss.insert(next_id),
    }
}

#[allow(clippy::type_complexity)]
fn operation(input: &[u8]) -> IResult<&[u8], ([u8; 4], u8, [u8; 4])> {
    let (input, lhs) = take(4usize)(input)?;
    let (input, _) = take(1usize)(input)?;
    let (input, op) = take(1usize)(input)?;
    let (input, _) = take(1usize)(input)?;
    let (input, rhs) = take(4usize)(input)?;

    Ok((
        input,
        (
            lhs.try_into().unwrap(),
            u8::from_ne_bytes(op.try_into().unwrap()),
            rhs.try_into().unwrap(),
        ),
    ))
}

fn take_monkey<'a>(
    input: &'a [u8],
    name_idx_map: &mut HashMap<[u8; 4], u16>,
    root_idx: &mut u16,
) -> IResult<&'a [u8], (u16, Monkey)> {
    let (input, name) = take(4usize)(input)?;
    let idx = idx_from_name(name.try_into().unwrap(), name_idx_map);

    if name == b"root" {
        *root_idx = idx
    }

    let (input, _) = take(2usize)(input)?;

    let (input, job) = alt((
        map(nom_u16, |num| Monkey::Num(num as i64)),
        map(operation, |(lhs, op, rhs)| {
            let lhs_idx = idx_from_name(lhs, name_idx_map);
            let rhs_idx = idx_from_name(rhs, name_idx_map);

            Monkey::Op(Op {
                lhs_idx,
                op,
                rhs_idx,
            })
        }),
    ))(input)?;

    Ok((input, (idx, job)))
}

pub fn run1(input: &[u8]) -> i64 {
    let min_len = input.len() / 8;

    let mut name_idx_map: HashMap<[u8; 4], u16> =
        HashMap::with_capacity(min_len * 2);

    let mut monkeys = vec![Monkey::Num(0); min_len];
    let mut root_idx = 0u16;

    for line in input.trim_ascii_end().split(|&b| b == b'\n') {
        let monkey = take_monkey(line, &mut name_idx_map, &mut root_idx)
            .unwrap()
            .1;
        monkeys[monkey.0 as usize] = monkey.1;
    }
    monkeys.truncate(name_idx_map.len());

    number(root_idx, &monkeys)
}

#[derive(Clone, Debug)]
enum Monkey2 {
    Num(Option<i64>),
    Op(Op),
}

fn number2(
    monkey: u16,
    monkeys: &Vec<Monkey2>,
    opstack: &mut Vec<(u8, i64)>,
) -> Option<i64> {
    match &monkeys[monkey as usize] {
        Monkey2::Num(n) => *n,
        Monkey2::Op(o) => {
            let lhs_opt = number2(o.lhs_idx, monkeys, opstack);
            let rhs_opt = number2(o.rhs_idx, monkeys, opstack);
            if let Some(lhs) = lhs_opt {
                if let Some(rhs) = rhs_opt {
                    match o.op {
                        b'+' => Some(lhs + rhs),
                        b'-' => Some(lhs - rhs),
                        b'*' => Some(lhs * rhs),
                        b'/' => Some(lhs / rhs),
                        _ => unreachable!(),
                    }
                } else {
                    let opposite_op = match o.op {
                        b'+' => (b'-', lhs),
                        b'-' => (b'_', lhs),
                        b'*' => (b'/', lhs),
                        b'/' => (b'%', lhs),
                        _ => unreachable!(),
                    };
                    opstack.push(opposite_op);
                    None
                }
            } else {
                let rhs = rhs_opt.unwrap();
                let opposite_op = match o.op {
                    b'+' => (b'-', rhs),
                    b'-' => (b'+', rhs),
                    b'*' => (b'/', rhs),
                    b'/' => (b'*', rhs),
                    _ => unreachable!(),
                };
                opstack.push(opposite_op);
                None
            }
        }
    }
}

fn take_monkey2<'a>(
    input: &'a [u8],
    name_idx_map: &mut HashMap<[u8; 4], u16>,
    root_idx: &mut u16,
) -> IResult<&'a [u8], (u16, Monkey2)> {
    let (input, name) = take(4usize)(input)?;
    let idx = idx_from_name(name.try_into().unwrap(), name_idx_map);

    if name == b"root" {
        *root_idx = idx
    }

    if name == b"humn" {
        return Ok((input, (idx, Monkey2::Num(None))));
    }

    let (input, _) = take(2usize)(input)?;

    let (input, job) = alt((
        map(nom_u16, |num| Monkey2::Num(Some(num as i64))),
        map(operation, |(lhs, op, rhs)| {
            let lhs_idx = idx_from_name(lhs, name_idx_map);
            let rhs_idx = idx_from_name(rhs, name_idx_map);

            Monkey2::Op(Op {
                lhs_idx,
                op,
                rhs_idx,
            })
        }),
    ))(input)?;

    Ok((input, (idx, job)))
}
pub fn run2(input: &[u8]) -> i64 {
    let min_len = input.len() / 8;

    let mut name_idx_map: HashMap<[u8; 4], u16> =
        HashMap::with_capacity(min_len * 2);

    let mut monkeys = vec![Monkey2::Num(None); min_len];
    let mut root_idx = 0u16;

    for line in input.trim_ascii_end().split(|&b| b == b'\n') {
        let monkey = take_monkey2(line, &mut name_idx_map, &mut root_idx)
            .unwrap()
            .1;
        monkeys[monkey.0 as usize] = monkey.1;
    }
    monkeys.truncate(name_idx_map.len());

    let mut opstack: Vec<(u8, i64)> = Vec::with_capacity(monkeys.len());

    number2(root_idx, &monkeys, &mut opstack);

    let root = opstack.pop().unwrap();

    let mut answer = root.1;
    while let Some((op, num)) = opstack.pop() {
        match op {
            b'+' => answer += num,
            b'-' => answer -= num,
            b'_' => answer = num - answer,
            b'*' => answer *= num,
            b'/' => answer /= num,
            b'%' => answer = num / answer,
            _ => unreachable!(),
        };
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example21");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 152)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 301)
    }
}
