const INPUT: &[u8] = include_bytes!("../res/input23");

#[allow(unused)]
pub fn part1() {
    let answer = run1(INPUT);
    assert_eq!(answer, 3923);
    println!("{answer}");
}

#[allow(unused)]
pub fn part2() {
    let answer = run2(INPUT);
    assert_eq!(answer, 1019);
    println!("{answer}");
}


#[allow(unused)]
fn print_grove(grove: &Vec<Vec<u8>>) {
    for line in grove {
        println!("{}", std::str::from_utf8(line).unwrap());
    }
}

pub fn run1(input: &[u8]) -> u64 {
    const ROUNDS: usize = 10;
    const ROUNDS_TWICE: usize = ROUNDS * 2;

    let len = input.len();
    let len_x = input.iter().position(|&b| b == b'\n').unwrap();
    let len_y = len / len_x + 1;

    let adjacent_tiles = [
    //    y,  x               |
        (-1, -1), // -       -
        (-1,  0), //  |(N)
        (-1,  1), // -       -
        ( 0,  1), //          |(E)
        ( 1,  1), // -       -
        ( 1,  0), //  |(S)
        ( 1, -1), // -       -
        ( 0, -1), //          |(W)
                  //          v
    ];
    
    let check_mask_and_move_list = [
    //  check_mask:  move: y, x
        (0b00000111, (-1,  0)), // N
        (0b01110000, ( 1,  0)), // S
        (0b11000001, ( 0, -1)), // W
        (0b00011100, ( 0,  1)), // E
    ];

    let mut elves = Vec::with_capacity(input.len());
    let mut grove = 
        vec![vec![b'.'; len_x + ROUNDS_TWICE]; len_y + ROUNDS_TWICE];

    let mut proposed_moves = Vec::with_capacity(elves.len());

    // First elf that wants to move there sets it to = round
    // Other elves that want to move there set it to = -round
    let mut proposed_tiles = 
        vec![vec![0i8; len_x + ROUNDS_TWICE]; len_y + ROUNDS_TWICE];

    for (row, line) in input.trim_ascii_end().split(|&b| b == b'\n').enumerate() {
        for (col, &c) in line.iter().enumerate() {
            let y = row + ROUNDS;
            let x = col + ROUNDS;
            if c == b'#' {
                elves.push((y as i16, x as i16));
                grove[y][x] = b'#';
            }
        }
    }

    let mut list_offset = 0;
    for round in 1i8..=ROUNDS.try_into().unwrap() {
        for (elf_idx, elf) in elves.iter().enumerate() {

            let mut adjacents = 0u16;
            for (bit_idx, tile) in adjacent_tiles.iter().enumerate() {

                if grove[(elf.0 + tile.0) as usize]
                    [(elf.1 + tile.1) as usize] == b'#'
                {
                    adjacents |= 1u16 << bit_idx;
                }
            }
            if adjacents == 0 {continue}

            for proposed_seq in 0..4 {
                let mut direction = list_offset + proposed_seq;
                if direction > 3 {
                    direction -= 4
                }

                let (mask, elf_move) = check_mask_and_move_list[direction];

                if adjacents & mask == 0 {
                    let move_tile = (elf.0 + elf_move.0, elf.1 + elf_move.1);

                    let proposed_tile_ref = &mut proposed_tiles
                        [move_tile.0 as usize][move_tile.1 as usize];

                    let neg_round = round.wrapping_neg();

                    if *proposed_tile_ref == round {
                        // this is the second elf trying to move here
                        *proposed_tile_ref = neg_round;

                    } else if *proposed_tile_ref == neg_round {
                        // this is the third or further elf trying to move here

                    } else {
                        // this is the first elf trying to move here
                        *proposed_tile_ref = round;
                        proposed_moves.push((
                            move_tile,
                            elf_idx,
                            *elf
                        ));
                    }

                    break;
                }
            }
        }

        for (move_tile, elf_idx, elf) in proposed_moves.drain(..) {
            if proposed_tiles[move_tile.0 as usize][move_tile.1 as usize] == round {
                elves[elf_idx] = (move_tile.0, move_tile.1);
                grove[elf.0 as usize][elf.1 as usize] = b'.';
                grove[move_tile.0 as usize][move_tile.1 as usize] = b'#';
            }
        }

        list_offset += 1;
        if list_offset == 4 {
            list_offset = 0;
        }
    }

    let mut min_y = i16::MAX;
    let mut max_y = i16::MIN;
    let mut min_x = i16::MAX;
    let mut max_x = i16::MIN;

    for elf in elves.iter() {
        min_y = min_y.min(elf.0);
        max_y = max_y.max(elf.0);
        min_x = min_x.min(elf.1);
        max_x = max_x.max(elf.1);
    }

    (max_y - min_y + 1) as u64 * (max_x - min_x + 1) as u64 - elves.len() as u64
}

pub fn run2(input: &[u8]) -> u64 {

    let len = input.len();
    let len_x = input.iter().position(|&b| b == b'\n').unwrap();
    let len_y = len / len_x + 1;
    let offset_x = len_x * 2;
    let offset_y = len_y * 2;
    let size_x = len_x * 5;
    let size_y = len_y * 5;

    let adjacent_tiles = [
    //    y,  x               |
        (-1, -1), // -       -
        (-1,  0), //  |(N)
        (-1,  1), // -       -
        ( 0,  1), //          |(E)
        ( 1,  1), // -       -
        ( 1,  0), //  |(S)
        ( 1, -1), // -       -
        ( 0, -1), //          |(W)
                  //          v
    ];
    
    let check_mask_and_move_list = [
    //  check_mask:  move: y, x
        (0b00000111, (-1,  0)), // N
        (0b01110000, ( 1,  0)), // S
        (0b11000001, ( 0, -1)), // W
        (0b00011100, ( 0,  1)), // E
    ];

    let mut elves = Vec::with_capacity(input.len());
    let mut grove = 
        vec![vec![b'.'; size_x]; size_y];

    let mut proposed_moves = Vec::with_capacity(elves.len());

    // First elf that wants to move there sets it to = round
    // Other elves that want to move there set it to = -round
    let mut proposed_tiles = 
        vec![vec![0i16; size_x]; size_y];

    for (row, line) in input.trim_ascii_end().split(|&b| b == b'\n').enumerate() {
        for (col, &c) in line.iter().enumerate() {
            let y = row + offset_y;
            let x = col + offset_x;
            if c == b'#' {
                elves.push((y as i16, x as i16));
                grove[y][x] = b'#';
            }
        }
    }


    let mut list_offset = 0;
    for round in 1i16..=i16::MAX {
        for (elf_idx, elf) in elves.iter().enumerate() {

            let mut adjacents = 0u16;
            for (bit_idx, tile) in adjacent_tiles.iter().enumerate() {

                if grove[(elf.0 + tile.0) as usize]
                    [(elf.1 + tile.1) as usize] == b'#'
                {
                    adjacents |= 1u16 << bit_idx;
                }
            }
            if adjacents == 0 {continue}

            for proposed_seq in 0..4 {
                let mut direction = list_offset + proposed_seq;
                if direction > 3 {
                    direction -= 4
                }

                let (mask, elf_move) = check_mask_and_move_list[direction];

                if adjacents & mask == 0 {
                    let move_tile = (elf.0 + elf_move.0, elf.1 + elf_move.1);

                    let proposed_tile_ref = &mut proposed_tiles
                        [move_tile.0 as usize][move_tile.1 as usize];

                    let neg_round = round.wrapping_neg();

                    if *proposed_tile_ref == round {
                        // this is the second elf trying to move here
                        *proposed_tile_ref = neg_round;

                    } else if *proposed_tile_ref == neg_round {
                        // this is the third or further elf trying to move here

                    } else {
                        // this is the first elf trying to move here
                        *proposed_tile_ref = round;
                        proposed_moves.push((
                            move_tile,
                            elf_idx,
                            *elf
                        ));
                    }

                    break;
                }
            }
        }

        if proposed_moves.len() == 0 {
            return round as u64;
        }

        for (move_tile, elf_idx, elf) in proposed_moves.drain(..) {
            if proposed_tiles[move_tile.0 as usize][move_tile.1 as usize] == round {
                elves[elf_idx] = (move_tile.0, move_tile.1);
                grove[elf.0 as usize][elf.1 as usize] = b'.';
                grove[move_tile.0 as usize][move_tile.1 as usize] = b'#';
            }
        }

        list_offset += 1;
        if list_offset == 4 {
            list_offset = 0;
        }
    }

    panic!("More than i16::MAX rounds would be required");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example23");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 110)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 20)
    }
}
