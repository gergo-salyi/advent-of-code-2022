use nom::{
    bytes::complete::{take, take_till1, take_while},
    character::complete::u8 as nom_u8,
    combinator::map,
    multi::many1,
    sequence::terminated,
    IResult,
};

const INPUT: &[u8] = include_bytes!("../res/input22");

#[allow(unused)]
pub fn part1() {
    let answer = run1(INPUT);
    assert_eq!(answer, 29408);
    println!("{answer}");
}

#[allow(unused)]
pub fn part2() {
    let answer = run2(INPUT);
    // assert_eq!(answer, todo!());
    println!("{answer}");
}

fn take_line(input: &[u8]) -> IResult<&[u8], (&[u8], usize)> {
    let (input, spaces) = take_while(|b| b == b' ')(input)?;
    let offset = spaces.len();
    let (input, tiles) = take_till1(|b| b == b'\n')(input)?;
    let (input, _) = take(1usize)(input)?;
    Ok((input, (tiles, offset)))
}

fn take_rows(input: &[u8]) -> IResult<&[u8], Vec<(&[u8], usize)>> {
    terminated(many1(take_line), take(1usize))(input)
}

fn take_move(input: &[u8]) -> IResult<&[u8], u8> {
    nom_u8(input)
}

fn take_turn(input: &[u8]) -> IResult<&[u8], u8> {
    map(take(1usize), |b: &[u8]| b[0])(input)
}

pub fn run1(input: &[u8]) -> u64 {
    let (mut input_instructions, board) = take_rows(input).unwrap();
    let mut col = board[0].1;
    let mut row = 0usize;
    let mut dir = 0usize;

    loop {
        let Ok((i, mov)) = take_move(input_instructions) else {break};
        input_instructions = i;

        match dir {
            0 => {
                let current_row = board[row];
                let offset = current_row.1;
                for _ in 0..mov {
                    match current_row.0.get(col.wrapping_sub(offset) + 1) {
                        Some(b'#') => break,
                        Some(b'.') => col += 1,
                        None => match current_row.0[0] {
                            b'#' => break,
                            b'.' => col = offset,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                }
            }
            1 => {
                for _ in 0..mov {
                    match board
                        .get(row + 1)
                        .and_then(|r| r.0.get(col.wrapping_sub(r.1)))
                    {
                        Some(b'#') => break,
                        Some(b'.') => row += 1,
                        None => {
                            let mut wrapped = row;
                            while board
                                .get(wrapped.wrapping_sub(1))
                                .and_then(|r| r.0.get(col.wrapping_sub(r.1)))
                                .is_some()
                            {
                                wrapped -= 1
                            }
                            let wrapped_row = board[wrapped];
                            match wrapped_row.0[col - wrapped_row.1] {
                                b'#' => break,
                                b'.' => row = wrapped,
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            2 => {
                let current_row = board[row];
                let offset = current_row.1;
                for _ in 0..mov {
                    match current_row.0.get(col.wrapping_sub(offset + 1)) {
                        Some(b'#') => break,
                        Some(b'.') => col -= 1,
                        None => {
                            let wrapped = current_row.0.len() - 1;
                            match current_row.0[wrapped] {
                                b'#' => break,
                                b'.' => col = offset + wrapped,
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            3 => {
                for _ in 0..mov {
                    match board
                        .get(row.wrapping_sub(1))
                        .and_then(|r| r.0.get(col.wrapping_sub(r.1)))
                    {
                        Some(b'#') => break,
                        Some(b'.') => row -= 1,
                        None => {
                            let mut wrapped = row;
                            while board
                                .get(wrapped + 1)
                                .and_then(|r| r.0.get(col.wrapping_sub(r.1)))
                                .is_some()
                            {
                                wrapped += 1
                            }
                            let wrapped_row = board[wrapped];
                            match wrapped_row.0[col - wrapped_row.1] {
                                b'#' => break,
                                b'.' => row = wrapped,
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }

        let Ok((i, trn)) = take_turn(input_instructions) else {break};
        input_instructions = i;

        match trn {
            b'L' => {
                if dir == 0 {
                    dir = 3
                } else {
                    dir -= 1
                }
            }
            b'R' => {
                if dir == 3 {
                    dir = 0
                } else {
                    dir += 1
                }
            }
            _ => break,
        }
    }

    (1000 * (row + 1) + 4 * (col + 1) + dir) as u64
}

pub fn run2(input: &[u8]) -> u64 {
    let (mut input_instructions, board) = take_rows(input).unwrap();
    let mut col = board[0].1;
    let mut row = 0usize;
    let mut dir = 0usize;

    loop {
        let Ok((i, mov)) = take_move(input_instructions) else {break};
        input_instructions = i;

        match dir {
            0 => {
                let current_row = board[row];
                let offset = current_row.1;
                for _ in 0..mov {
                    match current_row.0.get(col.wrapping_sub(offset) + 1) {
                        Some(b'#') => break,
                        Some(b'.') => col += 1,
                        None => match current_row.0[0] {
                            b'#' => break,
                            b'.' => col = offset,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                }
            }
            1 => {
                for _ in 0..mov {
                    match board
                        .get(row + 1)
                        .and_then(|r| r.0.get(col.wrapping_sub(r.1)))
                    {
                        Some(b'#') => break,
                        Some(b'.') => row += 1,
                        None => {
                            let mut wrapped = row;
                            while board
                                .get(wrapped.wrapping_sub(1))
                                .and_then(|r| r.0.get(col.wrapping_sub(r.1)))
                                .is_some()
                            {
                                wrapped -= 1
                            }
                            let wrapped_row = board[wrapped];
                            match wrapped_row.0[col - wrapped_row.1] {
                                b'#' => break,
                                b'.' => row = wrapped,
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            2 => {
                let current_row = board[row];
                let offset = current_row.1;
                for _ in 0..mov {
                    match current_row.0.get(col.wrapping_sub(offset + 1)) {
                        Some(b'#') => break,
                        Some(b'.') => col -= 1,
                        None => {
                            let wrapped = current_row.0.len() - 1;
                            match current_row.0[wrapped] {
                                b'#' => break,
                                b'.' => col = offset + wrapped,
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            3 => {
                for _ in 0..mov {
                    match board
                        .get(row.wrapping_sub(1))
                        .and_then(|r| r.0.get(col.wrapping_sub(r.1)))
                    {
                        Some(b'#') => break,
                        Some(b'.') => row -= 1,
                        None => {
                            let mut wrapped = row;
                            while board
                                .get(wrapped + 1)
                                .and_then(|r| r.0.get(col.wrapping_sub(r.1)))
                                .is_some()
                            {
                                wrapped += 1
                            }
                            let wrapped_row = board[wrapped];
                            match wrapped_row.0[col - wrapped_row.1] {
                                b'#' => break,
                                b'.' => row = wrapped,
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }

        let Ok((i, trn)) = take_turn(input_instructions) else {break};
        input_instructions = i;

        match trn {
            b'L' => {
                if dir == 0 {
                    dir = 3
                } else {
                    dir -= 1
                }
            }
            b'R' => {
                if dir == 3 {
                    dir = 0
                } else {
                    dir += 1
                }
            }
            _ => break,
        }
    }

    (1000 * (row + 1) + 4 * (col + 1) + dir) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example22");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 6032)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 5031)
    }
}
