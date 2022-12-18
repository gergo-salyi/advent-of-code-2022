use atoi::FromRadix10;

use std::collections::VecDeque;

const INPUT: &[u8] = include_bytes!("../res/input18");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 4370
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 2458
}

const GRID_SIZE: usize = 24;

pub fn run1(input: &[u8]) -> u64 {
    let mut answer = 0i64;

    let neighbors: [(i8, i8, i8); 6] = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    let mut grid = [[[0i8; GRID_SIZE]; GRID_SIZE]; GRID_SIZE];

    for line in input.trim_ascii().split(|&b| b == b'\n') {
        let mut coords = line.split(|&b| b == b',');

        let x = i8::from_radix_10(coords.next().unwrap()).0 + 1;
        let y = i8::from_radix_10(coords.next().unwrap()).0 + 1;
        let z = i8::from_radix_10(coords.next().unwrap()).0 + 1;

        let mut neighbor_count = 0i8;

        for nbr in neighbors {
            neighbor_count += grid[(x + nbr.0) as usize][(y + nbr.1) as usize]
                [(z + nbr.2) as usize];
        }
        answer += (6 - neighbor_count * 2) as i64;

        grid[x as usize][y as usize][z as usize] = 1;
    }

    answer as u64
}

pub fn run2(input: &[u8]) -> u64 {
    let mut answer = 0u64;

    let neighbors: [(i8, i8, i8); 6] = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    // -1 => empty, visited
    //  0 => empty, unvisited
    //  1 => filled
    let mut grid = [[[0i8; GRID_SIZE]; GRID_SIZE]; GRID_SIZE];

    for line in input.trim_ascii().split(|&b| b == b'\n') {
        let mut coords = line.split(|&b| b == b',');

        let x = i8::from_radix_10(coords.next().unwrap()).0 + 1;
        let y = i8::from_radix_10(coords.next().unwrap()).0 + 1;
        let z = i8::from_radix_10(coords.next().unwrap()).0 + 1;

        grid[x as usize][y as usize][z as usize] = 1;
    }

    let mut search_path =
        VecDeque::with_capacity(GRID_SIZE * GRID_SIZE * GRID_SIZE);

    search_path.push_back((0i8, 0i8, 0i8));

    while let Some(point) = search_path.pop_front() {
        for nbr in neighbors {
            let nbr_x = point.0 + nbr.0;
            let nbr_y = point.1 + nbr.1;
            let nbr_z = point.2 + nbr.2;

            let Some(plane) = grid.get_mut(nbr_x as usize) else {continue};
            let Some(line) = plane.get_mut(nbr_y as usize) else {continue};
            let Some(np) = line.get_mut(nbr_z as usize) else {continue};

            match *np {
                0 => {
                    search_path.push_back((nbr_x, nbr_y, nbr_z));
                    *np = -1;
                }
                1 => answer += 1,
                _ => {}
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example18");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 64)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 58)
    }
}
