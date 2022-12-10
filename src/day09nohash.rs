// use ahash::{HashSet, HashSetExt};
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

struct NoHashSet {
    len: usize,
    table: Vec<(i16, i16)>,
}
impl NoHashSet {
    fn new() -> Self {
        NoHashSet {
            // element (0, 0) is always present
            len: 1,
            table: vec![(0, 0); 65536],
        }
    }

    fn insert(&mut self, value: (i16, i16)) {
        // element (0, 0) is always present
        if value == (0, 0) {
            return;
        }

        let mut nohash = (value.0 << 8) | (value.1 & 0b11111111);

        let current = self.table[nohash as u16 as usize];

        // println!("{value:?}");
        // println!("{nohash:b}");

        if current == (0, 0) {
            // was empty
            self.table[nohash as u16 as usize] = value;
            self.len += 1;
        } else if current == value {
            // value is already inserted
            return;
        } else {
            // collision!
            #[cfg(debug_assertions)]
            println!("Collision for {value:?}");

            for _ in 0..7 {
                nohash = nohash.rotate_left(3);
                let current = self.table[nohash as u16 as usize];
                if current == (0, 0) {
                    // was empty
                    self.table[nohash as u16 as usize] = value;
                    self.len += 1;
                    return;
                } else if current == value {
                    // value is already inserted
                    return;
                } else {
                    // still collision!
                    #[cfg(debug_assertions)]
                    println!("Repeated collision");

                    continue;
                }
            }

            panic!("Could not resolve collision")
        }
    }

    fn insert2(&mut self, value: (i16, i16)) {
        // element (0, 0) is always present
        if value == (0, 0) {
            return;
        }

        let mut nohash = (value.0 << 8) | (value.1 & 0b11111111);

        let current_ref =
            unsafe { self.table.get_unchecked_mut(nohash as u16 as usize) };
        let current = *current_ref;

        // println!("{value:?}");
        // println!("{nohash:b}");

        if current == (0, 0) {
            // was empty
            *current_ref = value;
            self.len += 1;
        } else if current == value {
            // value is already inserted
            return;
        } else {
            // collision!
            #[cfg(debug_assertions)]
            println!("Collision for {value:?}");

            for _ in 0..7 {
                nohash = nohash.rotate_left(3);

                let current_ref = unsafe {
                    self.table.get_unchecked_mut(nohash as u16 as usize)
                };
                let current = *current_ref;

                if current == (0, 0) {
                    // was empty
                    *current_ref = value;
                    self.len += 1;
                    return;
                } else if current == value {
                    // value is already inserted
                    return;
                } else {
                    // still collision!
                    #[cfg(debug_assertions)]
                    println!("Repeated collision");

                    continue;
                }
            }

            panic!("Could not resolve collision")
        }
    }
}

fn parse(input: &[u8]) -> (Vec<(u8, u8)>, usize) {
    let mut parsed = Vec::with_capacity(input.len() * 5); // 'd nn\n'

    let mut total_moves = 0usize;

    for line in input.split(|&b| b == b'\n') {
        if line.is_empty() {
            break;
        };

        let direction = line[0];
        let count = u8::from_radix_10(&line[2..]).0;

        total_moves += count as usize;

        parsed.push((direction, count));
    }

    let reserve = total_moves * 2;

    (parsed, reserve)
}

fn move_head(head: (i16, i16), direction: u8) -> (i16, i16) {
    match direction {
        b'U' => (head.0 - 1, head.1),
        b'D' => (head.0 + 1, head.1),
        b'L' => (head.0, head.1 - 1),
        b'R' => (head.0, head.1 + 1),
        _ => unreachable!(),
    }
}

fn move_tail(head: (i16, i16), tail: (i16, i16)) -> ((i16, i16), bool) {
    let v_diff = tail.0 - head.0;
    let h_diff = tail.1 - head.1;

    if v_diff.abs() <= 1 && h_diff.abs() <= 1 {
        (tail, false)
    } else {
        ((tail.0 - v_diff.signum(), tail.1 - h_diff.signum()), true)
    }
}

pub fn run1(input: &[u8]) -> u64 {
    let (parsed, _reserve) = parse(input);

    // let mut visited: HashSet<(i16, i16)> = HashSet::with_capacity(reserve);
    let mut visited = NoHashSet::new();

    //             verti, horiz
    let mut head = (0i16, 0i16);
    let mut tail = (0i16, 0i16);

    visited.insert(tail);

    for (direction, count) in parsed.iter() {
        for _ in 0..*count {
            head = move_head(head, *direction);
            let is_moved;
            (tail, is_moved) = move_tail(head, tail);
            if is_moved {
                visited.insert2(tail);
            }
        }
    }

    visited.len as u64
}

pub fn run2(input: &[u8]) -> u64 {
    let (parsed, _reserve) = parse(input);

    // let mut visited: HashSet<(i16, i16)> = HashSet::with_capacity(reserve);
    let mut visited = NoHashSet::new();

    //              verti, horiz
    let mut rope = [(0i16, 0i16); 10];

    visited.insert(rope[9]);

    for (direction, count) in parsed.iter() {
        for _ in 0..*count {
            rope[0] = move_head(rope[0], *direction);
            for knot in 1..9 {
                (rope[knot], _) = move_tail(rope[knot - 1], rope[knot]);
            }
            let is_moved;
            (rope[9], is_moved) = move_tail(rope[8], rope[9]);
            if is_moved {
                visited.insert(rope[9]);
            }
        }
    }

    visited.len as u64
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
