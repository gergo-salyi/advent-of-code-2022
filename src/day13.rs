use std::cmp::Ordering;

use atoi::FromRadix10;
use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0},
    combinator::map,
    multi::{many1, separated_list0},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};
const INPUT: &[u8] = include_bytes!("../res/input13");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 5555
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 22852
}

#[derive(Debug, Eq, PartialEq)]
enum Item {
    Integer(usize),
    List(Vec<Item>),
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Integer(int_left), Item::Integer(int_right)) => {
                int_left.cmp(int_right)
            }

            (Item::List(list_left), Item::List(list_right)) => {
                for (item_left, item_right) in list_left.iter().zip(list_right)
                {
                    match item_left.cmp(item_right) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }
                }

                list_left.len().cmp(&list_right.len())
            }

            (Item::Integer(int_left), Item::List(_list_right)) => {
                Item::List(vec![Item::Integer(*int_left)]).cmp(other)
            }

            (Item::List(_list_left), Item::Integer(int_right)) => {
                self.cmp(&Item::List(vec![Item::Integer(*int_right)]))
            }
        }
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn decimal(input: &[u8]) -> IResult<&[u8], usize> {
    map(digit1, |b: &[u8]| usize::from_radix_10(b).0)(input)
}

fn integer(input: &[u8]) -> IResult<&[u8], Item> {
    map(decimal, Item::Integer)(input)
}

fn list(input: &[u8]) -> IResult<&[u8], Item> {
    map(
        delimited(char('['), separated_list0(char(','), item), char(']')),
        Item::List,
    )(input)
}

fn item(input: &[u8]) -> IResult<&[u8], Item> {
    alt((integer, list))(input)
}

fn packet(input: &[u8]) -> IResult<&[u8], Item> {
    terminated(item, multispace0)(input)
}

fn pair(input: &[u8]) -> IResult<&[u8], (Item, Item)> {
    terminated(separated_pair(list, char('\n'), list), multispace0)(input)
}

fn parse1(input: &[u8]) -> IResult<&[u8], Vec<(Item, Item)>> {
    many1(pair)(input)
}

fn parse2(input: &[u8]) -> IResult<&[u8], Vec<Item>> {
    many1(packet)(input)
}

pub fn run1(input: &[u8]) -> u64 {
    let mut answer = 0u64;

    let packets = parse1(input).unwrap().1;

    for (i, packet) in packets.iter().enumerate() {
        if packet.0 < packet.1 {
            answer += i as u64 + 1;
        }
    }

    answer
}

pub fn run2(input: &[u8]) -> u64 {
    let mut packets = parse2(input).unwrap().1;

    packets.sort_unstable();

    let divider1 = packets
        .binary_search(&packet(b"[[2]]").unwrap().1)
        .unwrap_err();

    let divider2 = packets
        .binary_search(&packet(b"[[6]]").unwrap().1)
        .unwrap_err();

    ((divider1 + 1) * (divider2 + 2)) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example13");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 13)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 140)
    }
}
