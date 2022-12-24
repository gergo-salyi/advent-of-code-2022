use std::mem::swap;

const INPUT: &[u8] = include_bytes!("../res/input24");

#[allow(unused)]
pub fn part1() {
    let answer = run1(INPUT);
    assert_eq!(answer, 257);
    println!("{answer}");
}

#[allow(unused)]
pub fn part2() {
    let answer = run2(INPUT);
    assert_eq!(answer, 828);
    println!("{answer}");
}

const UP: u8    = 0b0001u8;
const DOWN: u8  = 0b0010u8;
const LEFT: u8  = 0b0100u8;
const RIGHT: u8 = 0b1000u8;

#[allow(unused)]
fn print_valley(valley: &Vec<Vec<u8>>) {
    for line in valley {
        let row = line.iter()
            .map(|&blizzard| match blizzard {
                0 => b'.',
                UP => b'^',
                DOWN => b'v',
                LEFT => b'<',
                RIGHT => b'>',
                operlap => operlap.count_ones() as u8 + b'0',
            }).collect::<Vec<_>>();
        println!("{}", String::from_utf8(row).unwrap());
    }
}
#[allow(unused)]
fn print_valley_dir(valley: &Vec<Vec<u8>>, dir: u8) {
    for line in valley {
        let row = line.iter()
            .map(|&blizzard| match blizzard & dir {
                0 => b'.',
                UP => b'^',
                DOWN => b'v',
                LEFT => b'<',
                RIGHT => b'>',
                operlap => operlap.count_ones() as u8 + b'0',
            }).collect::<Vec<_>>();
        println!("{}", String::from_utf8(row).unwrap());
    }
}

fn move_blizzards(valley: &[Vec<u8>], mut empty: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let y_max = empty.len() - 1;
    let x_max = empty[0].len() - 1;

    for (y, row) in valley.iter().enumerate() {

        let (above, current, below) = if y == 0 {

            let (top_row, body) = empty.split_first_mut().unwrap();
            let (bot_row, body) = body.split_last_mut().unwrap();
            (bot_row, top_row, &mut body[0])

        } else if y == y_max {

            let (top_row, body) = empty.split_first_mut().unwrap();
            let (bot_row, body) = body.split_last_mut().unwrap();
            (body.last_mut().unwrap() , bot_row, top_row)

        } else {
            let (aboves, body) = empty.split_at_mut(y);
            let (current, belows) = body.split_at_mut(1);
            (aboves.last_mut().unwrap(), &mut current[0], belows.first_mut().unwrap())
        };

        let iter = row.iter()
            .zip(above.iter_mut())
            .zip(below.iter_mut())
            .enumerate();

        for (x, ((blizzard, pos_above), pos_below)) in iter {
            let idx_left = if x > 0 {
                x - 1 
            } else {
                x_max
            };
            let idx_right = if x < x_max {
                x + 1
            } else {
                0
            };
            *pos_above |= blizzard & UP;
            *pos_below |= blizzard & DOWN;
            current[idx_left] |= blizzard & LEFT;
            current[idx_right] |= blizzard & RIGHT;
        }
    }

    empty
}

pub fn run1(input: &[u8]) -> u64 {
    // o---->
    // |    x
    // |
    // v y

    let mut lines = input.trim_ascii_end().split(|&b| b == b'\n');

    let top_wall  = lines.next().unwrap();

    let len_x = top_wall.len() - 2; // #...#
    let len_y = (input.len() + 1) / (len_x + 3) - 2;
    
    debug_assert_eq!(b'.', top_wall[1]);
    debug_assert!(2 > (input.len() + 1) % (len_x + 3));

    let mut valley = Vec::with_capacity(len_y);

    for line in lines {
        let mut row = Vec::with_capacity(len_x);
        for c in line.iter().skip(1) {
            let blizzard = match c {
                b'.' => 0u8,
                b'^' => UP,
                b'v' => DOWN,
                b'<' => LEFT,
                b'>' => RIGHT,
                b'#' => break,
                _ => unreachable!(),
            };
            row.push(blizzard);
        }
        if !row.is_empty() {
            valley.push(row);
        }
    }

    let empty = vec![vec![0u8; len_x]; len_y];

    valley = move_blizzards(&valley, empty.clone());
    assert_eq!(valley[0][0], 0);

    let goal_y = len_y as u8 - 1;
    let goal_x = len_x as u8 - 1;

    let mut poss = vec![vec![0u16; len_x]; len_y];
    let mut poss_next = vec![vec![0u16; len_x]; len_y];

    poss[0][0] = 1;

    for minute in 2..u16::MAX {
        valley = move_blizzards(&valley, empty.clone());

        let prev_min = minute - 1;

        for y0 in 0..(len_y as u8) {
            for x0 in 0..(len_x as u8) {

                if valley[y0 as usize][x0 as usize] == 0 {
                    // no blizzard at this point, we may move here

                    let neighbors = [
                        (y0, x0),
                        (y0.wrapping_sub(1), x0),
                        (y0 + 1, x0),
                        (y0, x0.wrapping_sub(1)),
                        (y0, x0 + 1),
                    ];

                    for neighbor in neighbors {
                        if let Some(row) = poss.get(neighbor.0 as usize) {
                            if let Some(point) = row.get(neighbor.1 as usize) {

                                if *point == prev_min {
                                    if y0 == goal_y && x0 == goal_x {
                                        return minute as u64 + 1;
                                    }
                                    poss_next[y0 as usize][x0 as usize] = minute;
                                    break;
                                }

                            }
                        }
                    }

                }

            }
        }

        swap(&mut poss, &mut poss_next);

    }

    panic!("No solution in u16::MAX game-minutes")
}

pub fn run2(input: &[u8]) -> u64 {
    // o---->
    // |    x
    // |
    // v y

    let mut lines = input.trim_ascii_end().split(|&b| b == b'\n');

    let top_wall  = lines.next().unwrap();

    let len_x = top_wall.len() - 2; // #...#
    let len_y = (input.len() + 1) / (len_x + 3) - 2;
    
    debug_assert_eq!(b'.', top_wall[1]);
    debug_assert!(2 > (input.len() + 1) % (len_x + 3));

    let mut valley = Vec::with_capacity(len_y);

    for line in lines {
        let mut row = Vec::with_capacity(len_x);
        for c in line.iter().skip(1) {
            let blizzard = match c {
                b'.' => 0u8,
                b'^' => UP,
                b'v' => DOWN,
                b'<' => LEFT,
                b'>' => RIGHT,
                b'#' => break,
                _ => unreachable!(),
            };
            row.push(blizzard);
        }
        if !row.is_empty() {
            valley.push(row);
        }
    }

    let empty = vec![vec![0u8; len_x]; len_y];

    valley = move_blizzards(&valley, empty.clone());
    assert_eq!(valley[0][0], 0);

    let goal_y = len_y as u8 - 1;
    let goal_x = len_x as u8 - 1;

    let mut poss = vec![vec![0u16; len_x]; len_y];
    let mut poss_next = vec![vec![0u16; len_x]; len_y];

    poss[0][0] = 1;

    let mut target_y = goal_y;
    let mut target_x = goal_x;

    let mut minute = 2u16;

    'forth1: loop {
        valley = move_blizzards(&valley, empty.clone());
        let prev_min = minute - 1;
        for y0 in 0..(len_y as u8) {
            for x0 in 0..(len_x as u8) {
                if valley[y0 as usize][x0 as usize] == 0 {
                    // no blizzard at this point, we may move here
                    let neighbors = [
                        (y0, x0),
                        (y0.wrapping_sub(1), x0),
                        (y0 + 1, x0),
                        (y0, x0.wrapping_sub(1)),
                        (y0, x0 + 1),
                    ];
                    for neighbor in neighbors {
                        if let Some(row) = poss.get(neighbor.0 as usize) {
                            if let Some(point) = row.get(neighbor.1 as usize) {
                                if *point == prev_min {
                                    if y0 == target_y && x0 == target_x {
                                        break 'forth1;
                                    }
                                    poss_next[y0 as usize][x0 as usize] = minute;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        swap(&mut poss, &mut poss_next);
        minute += 1;
    }

    // move out of the field
    minute += 1;
    valley = move_blizzards(&valley, empty.clone());

    target_y = 0;
    target_x = 0;
    poss = vec![vec![0u16; len_x]; len_y];
    poss_next = vec![vec![0u16; len_x]; len_y];

    // try moving back to the field
    loop {
        minute += 1;
        valley = move_blizzards(&valley, empty.clone());
        if valley[goal_y as usize][goal_x as usize] == 0 {
            poss[goal_y as usize][goal_x as usize] = minute;
            break;
        }
    }

    // start going backwards
    minute += 1;
    'back: loop {
        valley = move_blizzards(&valley, empty.clone());
        let prev_min = minute - 1;

        if valley[goal_y as usize][goal_x as usize] == 0 {
            poss_next[goal_y as usize][goal_x as usize] = minute;
        }

        for y0 in 0..(len_y as u8) {
            for x0 in 0..(len_x as u8) {
                if valley[y0 as usize][x0 as usize] == 0 {
                    // no blizzard at this point, we may move here
                    let neighbors = [
                        (y0, x0),
                        (y0.wrapping_sub(1), x0),
                        (y0 + 1, x0),
                        (y0, x0.wrapping_sub(1)),
                        (y0, x0 + 1),
                    ];
                    for neighbor in neighbors {
                        if let Some(row) = poss.get(neighbor.0 as usize) {
                            if let Some(point) = row.get(neighbor.1 as usize) {
                                if *point == prev_min {
                                    if y0 == target_y && x0 == target_x {
                                        break 'back;
                                    }
                                    poss_next[y0 as usize][x0 as usize] = minute;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        swap(&mut poss, &mut poss_next);
        minute += 1;
    }

    // move out of the field
    minute += 1;
    valley = move_blizzards(&valley, empty.clone());

    target_y = goal_y;
    target_x = goal_x;
    poss = vec![vec![0u16; len_x]; len_y];
    poss_next = vec![vec![0u16; len_x]; len_y];

    // try moving back to the field
    loop {
        minute += 1;
        valley = move_blizzards(&valley, empty.clone());
        if valley[0][0] == 0 {
            poss[0][0] = minute;
            break;
        }
    }

    // start going forward again
    minute += 1;
    'forth2: loop {
        valley = move_blizzards(&valley, empty.clone());
        let prev_min = minute - 1;

        if valley[0][0] == 0 {
            poss_next[0][0] = minute;
        }

        for y0 in 0..(len_y as u8) {
            for x0 in 0..(len_x as u8) {
                if valley[y0 as usize][x0 as usize] == 0 {
                    // no blizzard at this point, we may move here
                    let neighbors = [
                        (y0, x0),
                        (y0.wrapping_sub(1), x0),
                        (y0 + 1, x0),
                        (y0, x0.wrapping_sub(1)),
                        (y0, x0 + 1),
                    ];
                    for neighbor in neighbors {
                        if let Some(row) = poss.get(neighbor.0 as usize) {
                            if let Some(point) = row.get(neighbor.1 as usize) {
                                if *point == prev_min {
                                    if y0 == target_y && x0 == target_x {
                                        break 'forth2;
                                    }
                                    poss_next[y0 as usize][x0 as usize] = minute;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        swap(&mut poss, &mut poss_next);
        minute += 1;
    }

    // move out of the field
    minute += 1;

    minute as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example24");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 18)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 54)
    }
}
