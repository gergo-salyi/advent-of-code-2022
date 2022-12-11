use atoi::FromRadix10;
use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::{alphanumeric1, char, digit1, multispace0, one_of},
    combinator::map,
    multi::{many1, separated_list0},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

use std::ops::{Add, Mul};

const INPUT: &[u8] = include_bytes!("../res/input11");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 66124
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 19309892877
}

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    divisor: usize,
    target_true: usize,
    target_false: usize,
}

fn decimal(input: &[u8]) -> IResult<&[u8], usize> {
    map(digit1, |b: &[u8]| usize::from_radix_10(b).0)(input)
}

fn starting_items(input: &[u8]) -> IResult<&[u8], Vec<usize>> {
    delimited(
        delimited(multispace0, tag("Starting items:"), multispace0),
        separated_list0(terminated(char(','), multispace0), decimal),
        multispace0,
    )(input)
}

fn operation(input: &[u8]) -> IResult<&[u8], Box<dyn Fn(usize) -> usize>> {
    map(
        delimited(
            delimited(
                multispace0::<&[u8], _>,
                tag("Operation: new = old"),
                multispace0,
            ),
            separated_pair(one_of("+*"), multispace0, alphanumeric1),
            multispace0,
        ),
        |(operator, operand)| {
            let operation = match operator {
                '+' => usize::add,
                '*' => usize::mul,
                _ => unimplemented!(),
            };
            match operand {
                b"old" => Box::new(move |old| operation(old, old)) as _,
                digits => {
                    let numeric_operand = usize::from_radix_10(digits).0;
                    Box::new(move |old| operation(old, numeric_operand)) as _
                }
            }
        },
    )(input)
}

fn divisor(input: &[u8]) -> IResult<&[u8], usize> {
    delimited(
        delimited(multispace0, tag("Test: divisible by"), multispace0),
        decimal,
        multispace0,
    )(input)
}

fn target_true(input: &[u8]) -> IResult<&[u8], usize> {
    delimited(
        delimited(multispace0, tag("If true: throw to monkey"), multispace0),
        decimal,
        multispace0,
    )(input)
}

fn target_false(input: &[u8]) -> IResult<&[u8], usize> {
    delimited(
        delimited(multispace0, tag("If false: throw to monkey"), multispace0),
        decimal,
        multispace0,
    )(input)
}

fn monkey(input: &[u8]) -> IResult<&[u8], Monkey> {
    map(
        tuple((
            tuple((tag("Monkey"), take_until("\n"), take(1usize))),
            starting_items,
            operation,
            divisor,
            target_true,
            target_false,
        )),
        |(_, items, operation, divisor, target_true, target_false)| Monkey {
            items,
            operation,
            divisor,
            target_true,
            target_false,
        },
    )(input)
}

fn parse(input: &[u8]) -> IResult<&[u8], Vec<Monkey>> {
    many1(monkey)(input)
}

pub fn run1(input: &[u8]) -> u64 {
    let mut monkeys = parse(input).unwrap().1;
    let mut activities = vec![0u64; monkeys.len()];
    for _ in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            for item_idx in 0..monkeys[monkey_idx].items.len() {
                let monkey = &monkeys[monkey_idx];

                activities[monkey_idx] += 1;

                let worry_level = (*monkey.operation)(monkey.items[item_idx]);

                let worry_level = worry_level / 3;

                if worry_level % monkey.divisor == 0 {
                    let target = monkey.target_true;
                    monkeys[target].items.push(worry_level);
                } else {
                    let target = monkey.target_false;
                    monkeys[target].items.push(worry_level);
                }
            }

            monkeys[monkey_idx].items.clear();
        }
    }
    activities.sort_unstable();
    activities.pop().unwrap() * activities.pop().unwrap()
}

pub fn run2(input: &[u8]) -> u64 {
    let mut monkeys = parse(input).unwrap().1;
    let common_factor: usize = monkeys.iter().map(|m| m.divisor).product();
    let mut activities = vec![0u64; monkeys.len()];
    for _ in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            let item_count = monkeys[monkey_idx].items.len();
            activities[monkey_idx] += item_count as u64;

            for item_idx in 0..item_count {
                let monkey = &monkeys[monkey_idx];

                let worry_level = (*monkey.operation)(monkey.items[item_idx]);

                let worry_level = worry_level % common_factor;

                if worry_level % monkey.divisor == 0 {
                    let target = monkey.target_true;
                    monkeys[target].items.push(worry_level);
                } else {
                    let target = monkey.target_false;
                    monkeys[target].items.push(worry_level);
                }
            }
            monkeys[monkey_idx].items.clear();
        }
    }
    activities.sort_unstable();
    activities.pop().unwrap() * activities.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example11");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 10605)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 2713310158)
    }

    #[test]
    fn test_monkey() {
        assert_eq!(
            23,
            monkey(
                b"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
"
            )
            .unwrap()
            .1
            .divisor
        )
    }

    #[test]
    fn test_monkeys() {
        assert_eq!(
            19,
            parse(
                b"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
"
            )
            .unwrap()
            .1[1]
                .divisor
        )
    }
}
