use ahash::{HashMap, HashMapExt};

const INPUT: &[u8] = include_bytes!("../res/input17");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 3181
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 1570434782634
}

fn repr(id: u8) -> &'static str {
    match id {
        0 => ".",
        1 => "#",
        2 => "@",
        _ => unreachable!()
    }
}

#[allow(unused)]
fn print(chamber: &[[u8; 7]; 2022 * 4]) {
    println!();
    for i in 0..40 {
        let layer = chamber[39 - i];
        println!("{:2} |{}{}{}{}{}{}{}|", 
            39 - i,
            repr(layer[0]),
            repr(layer[1]),
            repr(layer[2]),
            repr(layer[3]),
            repr(layer[4]),
            repr(layer[5]),
            repr(layer[6]),
        )
    }
    println!("   +-------+");
    println!();
}

pub fn run1(input: &[u8]) -> u64 {

    let mut chamber = [[0u8; 7]; 2022 * 4];

    let mut jets = input.trim_ascii().iter().cycle();

    let rock0: &[&[u8]] = &[&[1,1,1,1]];
    let rock1: &[&[u8]] = &[&[0,1,0],&[1,1,1],&[0,1,0]];
    let rock2: &[&[u8]] = &[&[1,1,1],&[0,0,1],&[0,0,1]];
    let rock3: &[&[u8]] = &[&[1],&[1],&[1],&[1]];
    let rock4: &[&[u8]] = &[&[1,1],&[1,1]];

    let rocks = [rock0, rock1, rock2, rock3, rock4];

    let mut pile_height = 0u16;
    let mut x: u8;
    let mut y: u16;

    for rock_seq in 0..2022 {

        let rock = rocks[rock_seq % 5];
        let rock_height = rock.len() as u8;
        let rock_width = rock[0].len() as u8;

        x = 2;

        for _ in 0..3 {
            match jets.next() {
                Some(b'<') => {
                    if x > 0 {
                        x -= 1;
                    }
                },
                Some(b'>') => {
                    if x + rock_width < 7 {
                        x += 1;
                    }
                },
                _ => unreachable!(),
            }
        }
        
        y = pile_height;

        loop {

            // how many layers of the rock to check starting from the bottom
            let check_depth_h = rock_height.min((pile_height - y as u16) as u8);

            match jets.next() {
                Some(b'<') => {

                    if x > 0 {

                        let mut left_collision = false;
                        'left_coll_chk: for ry in 0..check_depth_h {
                            for rx in 0..rock_width {
                                if rock[ry as usize][rx as usize]
                                    + chamber[(y + ry as u16) as usize]
                                        [(x + rx - 1) as usize] > 1 
                                {
                                    left_collision = true;
                                    break 'left_coll_chk;
                                }
                            }
                        }

                        if !left_collision {
                            x -= 1;
                        }
                    }
                },

                Some(b'>') => {

                    if x + rock_width < 7 {

                        let mut right_collision = false;
                        'right_coll_chk: for ry in 0..check_depth_h {
                            for rx in 0..rock_width {
                                if rock[ry as usize][(rock_width - rx - 1) as usize]
                                    + chamber[(y + ry as u16) as usize]
                                        [(x + rock_width - rx) as usize] > 1 
                                {
                                    right_collision = true;
                                    break 'right_coll_chk;
                                }
                            }
                        }

                        if !right_collision {
                            x += 1;
                        }
                    }
                },
                _ => continue,
            };
        
            if y == 0 {
                break;
            }

            // how many layers of the rock to check starting from the bottom
            let check_depth_v = rock_height.min((pile_height - y as u16 + 1) as u8);

            let mut bottom_collision = false;

            'bottom_coll_chk: for layer in 0..check_depth_v {
            
                let chamber_layer_below = 
                    chamber[(y + layer as u16 - 1) as usize];

                for (rx, rock_part) in rock[layer as usize]
                    .iter().enumerate() 
                {
                    if chamber_layer_below[x as usize + rx] + rock_part > 1 {
                        bottom_collision = true;
                        break 'bottom_coll_chk;
                    }
                }

            }

            if bottom_collision {
                break;
            }
            
            y -= 1;
        }

        // the rock comes to rest

        for (ry, rock_layer) in rock.iter().enumerate() {
            for (rx, rock_part) in rock_layer.iter().enumerate() {
                chamber[y as usize + ry][x as usize + rx] += *rock_part;
            }
        }

        pile_height = pile_height.max(y + rock_height as u16);
        
    }

    pile_height as u64
}

const DEPTH: usize = 17usize;

#[derive(Eq, Hash, PartialEq)]
struct State {
    rock: u8,
    jet: u16,
    chamber_top: [[u8; 7]; DEPTH]
}

pub fn run2(input: &[u8]) -> u64 {

    let input_trimmed = input.trim_ascii();

    let cycle = input_trimmed.len();

    let mut state_hist = HashMap::with_capacity(20_000_000);

    let mut chamber = vec![[0u8; 7]; 35_000_000];

    let mut jets = input_trimmed.iter().cycle();

    let rock0: &[&[u8]] = &[&[1,1,1,1]];
    let rock1: &[&[u8]] = &[&[0,1,0],&[1,1,1],&[0,1,0]];
    let rock2: &[&[u8]] = &[&[1,1,1],&[0,0,1],&[0,0,1]];
    let rock3: &[&[u8]] = &[&[1],&[1],&[1],&[1]];
    let rock4: &[&[u8]] = &[&[1,1],&[1,1]];

    let rocks = [rock0, rock1, rock2, rock3, rock4];

    let mut pile_height = 0u32;
    let mut x: u8;
    let mut y: u32;

    let mut init_height = 0u64;
    let mut cycled_height = 0u64;
    let mut fini_height_base = 0u64;
    let mut fini_height = 0u64;

    let mut fini_seq = usize::MAX;

    let mut is_repetition = false;

    for rock_seq in 0.. {

        if rock_seq >= DEPTH && !is_repetition {
            
            let state = State {
                rock: (rock_seq % 5) as u8,
                jet: (rock_seq % cycle) as u16,
                chamber_top: chamber[(pile_height as usize - DEPTH)..pile_height as usize].try_into().unwrap()
            };

            let last_repetition = state_hist.insert(state, (rock_seq, pile_height));

            if let Some((last_rock_seq, last_pile_height)) = last_repetition {

                println!("Repetition: last: rock={last_rock_seq} height={last_pile_height}, current: rock={rock_seq} height={pile_height}");

                is_repetition = true;

                let init_count = last_rock_seq;
                init_height = last_pile_height as u64;

                let cycle_count = rock_seq - last_rock_seq;
                let cycle_height = pile_height - last_pile_height;

                let after_init = 1000000000000 - init_count;

                let whole_cycles = after_init / cycle_count;
                let residual = after_init % cycle_count;

                cycled_height = whole_cycles as u64 * cycle_height as u64;

                fini_height_base = pile_height as u64;
                fini_seq = rock_seq + residual;
            }
        }

        if rock_seq == fini_seq {
            fini_height = pile_height as u64 - fini_height_base;
            return init_height + cycled_height + fini_height;
        }

        let rock = rocks[rock_seq % 5];
        let rock_height = rock.len() as u8;
        let rock_width = rock[0].len() as u8;

        x = 2;

        for _ in 0..3 {
            match jets.next() {
                Some(b'<') => {
                    if x > 0 {
                        x -= 1;
                    }
                },
                Some(b'>') => {
                    if x + rock_width < 7 {
                        x += 1;
                    }
                },
                _ => unreachable!(),
            }
        }
        
        y = pile_height;

        loop {

            // how many layers of the rock to check starting from the bottom
            let check_depth_h = rock_height.min((pile_height - y as u32) as u8);

            match jets.next() {
                Some(b'<') => {

                    if x > 0 {

                        let mut left_collision = false;
                        'left_coll_chk: for ry in 0..check_depth_h {
                            for rx in 0..rock_width {
                                if rock[ry as usize][rx as usize]
                                    + chamber[(y + ry as u32) as usize]
                                        [(x + rx - 1) as usize] > 1 
                                {
                                    left_collision = true;
                                    break 'left_coll_chk;
                                }
                            }
                        }

                        if !left_collision {
                            x -= 1;
                        }
                    }
                },

                Some(b'>') => {

                    if x + rock_width < 7 {

                        let mut right_collision = false;
                        'right_coll_chk: for ry in 0..check_depth_h {
                            for rx in 0..rock_width {
                                if rock[ry as usize][(rock_width - rx - 1) as usize]
                                    + chamber[(y + ry as u32) as usize]
                                        [(x + rock_width - rx) as usize] > 1 
                                {
                                    right_collision = true;
                                    break 'right_coll_chk;
                                }
                            }
                        }

                        if !right_collision {
                            x += 1;
                        }
                    }
                },
                _ => continue,
            };
        
            if y == 0 {
                break;
            }

            // how many layers of the rock to check starting from the bottom
            let check_depth_v = rock_height.min((pile_height - y as u32 + 1) as u8);

            let mut bottom_collision = false;

            'bottom_coll_chk: for layer in 0..check_depth_v {
            
                let chamber_layer_below = 
                    chamber[(y + layer as u32 - 1) as usize];

                for (rx, rock_part) in rock[layer as usize]
                    .iter().enumerate() 
                {
                    if chamber_layer_below[x as usize + rx] + rock_part > 1 {
                        bottom_collision = true;
                        break 'bottom_coll_chk;
                    }
                }

            }

            if bottom_collision {
                break;
            }
            
            y -= 1;
        }

        // the rock comes to rest

        for (ry, rock_layer) in rock.iter().enumerate() {
            for (rx, rock_part) in rock_layer.iter().enumerate() {
                chamber[y as usize + ry][x as usize + rx] += *rock_part;
            }
        }

        pile_height = pile_height.max(y + rock_height as u32);

    }

    0u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example17");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 3068)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 1514285714288)
    }
}
