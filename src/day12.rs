use std::collections::BinaryHeap;

const INPUT: &[u8] = include_bytes!("../res/input12");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 520
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 508
}

pub fn run1(input: &[u8]) -> u64 {
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (x, line) in input.split(|&b| b == b'\n').enumerate() {
        if line.is_empty() {
            break;
        };
        map.push(Vec::new());
        for (y, character) in line.iter().enumerate() {
            let character = {
                match character {
                    b'S' => {
                        start = (x, y);
                        b'a'
                    }
                    b'E' => {
                        end = (x, y);
                        b'z'
                    }
                    other => *other,
                }
            };
            map[x].push(character)
        }
    }

    //                    distance  current x y
    let mut bh: BinaryHeap<(usize, (usize, usize))> = BinaryHeap::new();

    let mut min_steps_to_reach =
        vec![vec![usize::MAX; map[0].len()]; map.len()];

    let (end_x, end_y) = end;

    let (x, y) = start;
    let distance = end_x.abs_diff(x) + end_y.abs_diff(y);

    bh.push((usize::MAX - distance, (x, y)));

    min_steps_to_reach[x][y] = 0;

    let mut answer = u64::MAX;

    while let Some((_, (x, y))) = bh.pop() {
        let steps = min_steps_to_reach[x][y];

        if x == end_x && y == end_y {
            answer = answer.min(steps as u64);
            continue;
        }

        let height = map[x][y];
        let new_steps = steps + 1;

        let new_x = x + 1;
        let new_y = y;
        if let Some(row) = min_steps_to_reach.get(new_x) {
            if let Some(&old_steps) = row.get(new_y) {
                if map[new_x][new_y] <= height + 1 && new_steps < old_steps {
                    min_steps_to_reach[new_x][new_y] = new_steps;
                    let distance = end_x.abs_diff(new_x) + end_y.abs_diff(new_y);
                    bh.push((usize::MAX - distance, (new_x, new_y)));
                }
            }
        };

        let new_x = x;
        let new_y = y + 1;
        if let Some(row) = min_steps_to_reach.get(new_x) {
            if let Some(&old_steps) = row.get(new_y) {
                if map[new_x][new_y] <= height + 1 && new_steps < old_steps {
                    min_steps_to_reach[new_x][new_y] = new_steps;
                    let distance = end_x.abs_diff(new_x) + end_y.abs_diff(new_y);
                    bh.push((usize::MAX - distance, (new_x, new_y)));
                }
            }
        };

        let new_x = x.wrapping_sub(1);
        let new_y = y;
        if let Some(row) = min_steps_to_reach.get(new_x) {
            if let Some(&old_steps) = row.get(new_y) {
                if map[new_x][new_y] <= height + 1 && new_steps < old_steps {
                    min_steps_to_reach[new_x][new_y] = new_steps;
                    let distance = end_x.abs_diff(new_x) + end_y.abs_diff(new_y);
                    bh.push((usize::MAX - distance, (new_x, new_y)));
                }
            }
        };

        let new_x = x;
        let new_y = y.wrapping_sub(1);
        if let Some(row) = min_steps_to_reach.get(new_x) {
            if let Some(&old_steps) = row.get(new_y) {
                if map[new_x][new_y] <= height + 1 && new_steps < old_steps {
                    min_steps_to_reach[new_x][new_y] = new_steps;
                    let distance = end_x.abs_diff(new_x) + end_y.abs_diff(new_y);
                    bh.push((usize::MAX - distance, (new_x, new_y)));
                }
            }
        };
    }

    answer
}

pub fn run2(input: &[u8]) -> u64 {
    let mut map = Vec::new();
    // let mut start = (0,0);
    let mut end = (0, 0);

    for (x, line) in input.split(|&b| b == b'\n').enumerate() {
        if line.is_empty() {
            break;
        };
        map.push(Vec::new());
        for (y, character) in line.iter().enumerate() {
            let character = {
                match character {
                    b'S' => {
                        // start = (x, y);
                        b'a'
                    }
                    b'E' => {
                        end = (x, y);
                        b'z'
                    }
                    other => *other,
                }
            };
            map[x].push(character)
        }
    }

    //                      height  current x y
    let mut bh: BinaryHeap<(usize, (usize, usize))> = BinaryHeap::new();

    let mut min_steps_to_reach =
        vec![vec![usize::MAX; map[0].len()]; map.len()];

    let (x, y) = end;

    bh.push((usize::MAX - map[x][y] as usize, (x, y)));

    min_steps_to_reach[x][y] = 0;

    let mut answer = u64::MAX;

    while let Some((_, (x, y))) = bh.pop() {
        let steps = min_steps_to_reach[x][y];

        if map[x][y] == b'a' {
            answer = answer.min(steps as u64);
            continue;
        }

        let height = map[x][y];
        let new_steps = steps + 1;

        let new_x = x + 1;
        let new_y = y;
        if let Some(row) = min_steps_to_reach.get(new_x) {
            if let Some(&old_steps) = row.get(new_y) {
                if map[new_x][new_y] >= height - 1 && new_steps < old_steps {
                    min_steps_to_reach[new_x][new_y] = new_steps;
                    bh.push((
                        usize::MAX - map[new_x][new_y] as usize,
                        (new_x, new_y),
                    ));
                }
            }
        };

        let new_x = x;
        let new_y = y + 1;
        if let Some(row) = min_steps_to_reach.get(new_x) {
            if let Some(&old_steps) = row.get(new_y) {
                if map[new_x][new_y] >= height - 1 && new_steps < old_steps {
                    min_steps_to_reach[new_x][new_y] = new_steps;
                    bh.push((
                        usize::MAX - map[new_x][new_y] as usize,
                        (new_x, new_y),
                    ));
                }
            }
        };

        let new_x = x.wrapping_sub(1);
        let new_y = y;
        if let Some(row) = min_steps_to_reach.get(new_x) {
            if let Some(&old_steps) = row.get(new_y) {
                if map[new_x][new_y] >= height - 1 && new_steps < old_steps {
                    min_steps_to_reach[new_x][new_y] = new_steps;
                    bh.push((
                        usize::MAX - map[new_x][new_y] as usize,
                        (new_x, new_y),
                    ));
                }
            }
        };

        let new_x = x;
        let new_y = y.wrapping_sub(1);
        if let Some(row) = min_steps_to_reach.get(new_x) {
            if let Some(&old_steps) = row.get(new_y) {
                if map[new_x][new_y] >= height - 1 && new_steps < old_steps {
                    min_steps_to_reach[new_x][new_y] = new_steps;
                    bh.push((
                        usize::MAX - map[new_x][new_y] as usize,
                        (new_x, new_y),
                    ));
                }
            }
        };
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example12");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 31)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 29)
    }
}
