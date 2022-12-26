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
    assert_eq!(answer, 115311);
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

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;
const DIR_MOVS: [(usize, usize); 4] = [
    (0, 1),
    (1, 0),
    (0, 0usize.wrapping_sub(1)),
    (0usize.wrapping_sub(1), 0),
];

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
            // dir = (dir +- 1).rem_euclid(4)
            b'L' => dir = (dir.wrapping_sub(1)) & 3,
            b'R' => dir = (dir + 1) & 3,
            _ => break,
        }
    }

    (1000 * (row + 1) + 4 * (col + 1) + dir) as u64
}

struct Board<'a> {
    board: Vec<(&'a [u8], usize)>,
}
impl Board<'_> {
    fn get(&self, y: usize, x: usize) -> Option<&u8> {
        if let Some((row, offset)) = self.board.get(y) {
            row.get(x.wrapping_sub(*offset))
        } else {
            None
        }
    }
}

// Cube vertices:
//  E--F  
// A--B|
// |G |H
// C--D

const EDGES_1: [(u8, u8); 4] = [
    (b'A', b'B'),
    (b'C', b'D'),
    (b'G', b'H'),
    (b'E', b'F'),
];
const EDGES_2: [(u8, u8); 4] = [
    (b'A', b'C'),
    (b'B', b'D'),
    (b'F', b'H'),
    (b'E', b'G'),
];
const EDGES_3: [(u8, u8); 4] = [
    (b'A', b'E'),
    (b'B', b'F'),
    (b'D', b'H'),
    (b'C', b'G'),
];

fn edges_from_vertices(a: u8, b: u8) -> [(u8, u8); 4] {
    match a.abs_diff(b) {
        1 => EDGES_1,
        2 => EDGES_2,
        4 => EDGES_3,
        _ => unreachable!()
    }
}

#[cfg(test)]
const EDGE_LEN: usize = 4;
#[cfg(not(test))]
const EDGE_LEN: usize = 50;

const NIL: u8 = b'.';
const UNK: u8 = b'?';

struct NetGrid {
    grid: [[u8; 5]; 5]
}
impl NetGrid {

    fn form_chunk_coords(net_chunks: &[(usize, usize)]) -> Self {
        let mut net_grid = Self::init(net_chunks);
        
        for _ in 0..3 {
            net_grid.roll_down();
            net_grid.roll_right();
            net_grid.roll_left();
        }
        net_grid
    }

    fn get_vertex(&self, y: usize, x: usize) -> Option<&u8> {
        self.grid.get(y)
            .and_then(|row| row.get(x))
            .filter(|&&e| e >= b'A')
    }
    
    fn is_vertex(&self, y: usize, x: usize) -> bool {
        self.grid.get(y)
            .and_then(|row| row.get(x))
            .filter(|&&e| e >= b'A')
            .is_some()
    }

    fn is_unk(&self, y: usize, x: usize) -> bool {
        self.grid.get(y)
            .and_then(|row| row.get(x))
            .filter(|&&e| e == UNK)
            .is_some()
    }

    fn find_other(&self, vertex: u8, y0: usize, x0: usize)
        -> (usize, usize)
    {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &point) in row.iter().enumerate() {
                if point == vertex && y != y0 && x != x0 {
                    return (y, x);
                }
            }
        }
        (y0, x0)
    }
    
    fn find_other_edge(&self, 
        vertex_a: u8,
        y_a0: usize, 
        x_a0: usize,
        vertex_b: u8,
        y_b0: usize, 
        x_b0: usize,
    ) -> (usize, usize, usize, usize)
    {
        let mut a = Vec::with_capacity(3);
        let mut b = Vec::with_capacity(3);

        a.push((y_a0, x_a0));
        b.push((y_b0, x_b0));

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &point) in row.iter().enumerate() {
                if point == vertex_a && y != y_a0 && x != x_a0 {
                    a.push((y, x));
                }
                if point == vertex_b && y != y_b0 && x != x_b0 {
                    b.push((y, x));
                }
            }
        }

        for (i, &(ya, xa)) in a.iter().enumerate() {
            for (j, &(yb, xb)) in b.iter().enumerate() {
                if i == 0 && j == 0 {continue}
                if ya == yb && xa.abs_diff(xb) == 1 {
                    return (ya, xa, yb, xb);
                }
                if xa == xb && ya.abs_diff(yb) == 1 {
                    return (ya, xa, yb, xb);
                }
            }
        }

        panic!()
    }

    fn init(net_chunks: &[(usize, usize)]) -> Self {
        debug_assert_eq!(net_chunks.len(), 6);

        let mut grid = [[NIL; 5]; 5];

        let (chunk0_y, chunk0_x) = net_chunks[0];

        grid[chunk0_y][chunk0_x] = b'A';
        grid[chunk0_y][chunk0_x + 1] = b'B';
        grid[chunk0_y + 1][chunk0_x] = b'C';
        grid[chunk0_y + 1][chunk0_x + 1] = b'D';

        for &(y, x) in net_chunks[1..].iter() {
            let point = &mut grid[y][x];
            if *point == NIL {
                *point = UNK;
            }
            let point = &mut grid[y][x + 1];
            if *point == NIL {
                *point = UNK;
            }
            let point = &mut grid[y + 1][x];
            if *point == NIL {
                *point = UNK;
            }
            let point = &mut grid[y + 1][x + 1];
            if *point == NIL {
                *point = UNK;
            }
        }

        Self { grid }
    }

    fn print(&self) {
        println!();
        for line in self.grid {
            println!("{}", std::str::from_utf8(&line).unwrap())
        }
        println!();
    }

    fn roll_down(&mut self) {
        for y in 1..4 {
            for x in 0..5 {

                let point = self.grid[y][x];

                let Some(&paired) = self.get_vertex(y, x + 1) else {continue};

                let Some(&back_point) = self
                    .get_vertex(y - 1, x) else {continue};

                let Some(&back_paired) = self
                    .get_vertex(y - 1, x + 1) else {continue};

                let are_unknowns_front = 
                    self.is_unk(y + 1, x) && self.is_unk(y + 1, x + 1);

                if !are_unknowns_front {
                    continue;
                }

                let edges = edges_from_vertices(point, paired);

                let mut edges_iter_fwd = edges.iter().cycle();

                let edge_idx = edges_iter_fwd
                    .position(|e| e.0 == point.min(paired)).unwrap();

                let back_edge_idx = edges.iter()
                    .position(|e| e.0 == back_point.min(back_paired)).unwrap();

                let vertices = {

                    if edge_idx == (back_edge_idx + 1) % 4 {
                        // forward iter is good
                        edges_iter_fwd.next().unwrap()

                    } else {
                        // we need reverse iter
                        edges.iter().rev().cycle()
                            .skip_while(|e| e.0 != point.min(paired))
                            .skip(1).next().unwrap()
                    }
                };

                if paired > point {
                    self.grid[y + 1][x] = vertices.0;
                    self.grid[y + 1][x + 1] = vertices.1;

                } else {
                    self.grid[y + 1][x] = vertices.1;
                    self.grid[y + 1][x + 1] = vertices.0;
                }
            }
        }
    }
    
    fn roll_right(&mut self) {
        for x in 1..4 {
            for y in 0..5 {

                let point = self.grid[y][x];

                let Some(&paired) = self.get_vertex(y + 1, x) else {continue};

                let Some(&back_point) = self
                    .get_vertex(y, x - 1) else {continue};

                let Some(&back_paired) = self
                    .get_vertex(y + 1, x - 1) else {continue};

                let are_unknowns_front = 
                    self.is_unk(y, x + 1) && self.is_unk(y + 1, x + 1);

                if !are_unknowns_front {
                    continue;
                }

                let edges = edges_from_vertices(point, paired);

                let mut edges_iter_fwd = edges.iter().cycle();

                let edge_idx = edges_iter_fwd
                    .position(|e| e.0 == point.min(paired)).unwrap();

                let back_edge_idx = edges.iter()
                    .position(|e| e.0 == back_point.min(back_paired)).unwrap();

                let vertices = {

                    if edge_idx == (back_edge_idx + 1) % 4 {
                        // forward iter is good
                        edges_iter_fwd.next().unwrap()

                    } else {
                        // we need reverse iter
                        edges.iter().rev().cycle()
                            .skip_while(|e| e.0 != point.min(paired))
                            .skip(1).next().unwrap()
                    }
                };

                if paired > point {
                    self.grid[y][x + 1] = vertices.0;
                    self.grid[y + 1][x + 1] = vertices.1;

                } else {
                    self.grid[y][x + 1] = vertices.1;
                    self.grid[y + 1][x + 1] = vertices.0;
                }
            }
        }
    }
    
    fn roll_left(&mut self) {
        for x in (1..4).rev() {
            for y in 0..5 {

                let point = self.grid[y][x];

                let Some(&paired) = self.get_vertex(y + 1, x) else {continue};

                let Some(&back_point) = self
                    .get_vertex(y, x + 1) else {continue};

                let Some(&back_paired) = self
                    .get_vertex(y + 1, x + 1) else {continue};

                let are_unknowns_front = 
                    self.is_unk(y, x - 1) && self.is_unk(y + 1, x - 1);

                if !are_unknowns_front {
                    continue;
                }

                let edges = edges_from_vertices(point, paired);

                let mut edges_iter_fwd = edges.iter().cycle();

                let edge_idx = edges_iter_fwd
                    .position(|e| e.0 == point.min(paired)).unwrap();

                let back_edge_idx = edges.iter()
                    .position(|e| e.0 == back_point.min(back_paired)).unwrap();

                let vertices = {

                    if edge_idx == (back_edge_idx + 1) % 4 {
                        // forward iter is good
                        edges_iter_fwd.next().unwrap()

                    } else {
                        // we need reverse iter
                        edges.iter().rev().cycle()
                            .skip_while(|e| e.0 != point.min(paired))
                            .skip(1).next().unwrap()
                    }
                };

                if paired > point {
                    self.grid[y][x - 1] = vertices.0;
                    self.grid[y + 1][x - 1] = vertices.1;

                } else {
                    self.grid[y][x - 1] = vertices.1;
                    self.grid[y + 1][x - 1] = vertices.0;
                }
            }
        }
    }
}

#[derive(Default)]
struct Cube<const SIZE: usize> {
    // y0, x0, d0 -> (y, x, d) mapping at chunk_y0, chunk_x0, dir0
    wrapping_maps: [[[
        Option<Box<dyn Fn(usize, usize) -> (usize, usize, usize)>>;
    4]; 4]; 4],
}
impl<const SIZE: usize> Cube<SIZE> {
    fn from_net(net_chunks: &[(usize, usize)]) -> Self {
        debug_assert_eq!(net_chunks.len(), 6);
        let mut cube = Cube::default();

        let net_grid = NetGrid::form_chunk_coords(net_chunks);
        net_grid.print();

        // map top edges
        for x in 0..4 {
            for y in 0..4 {
                let paired_y = y;
                let paired_x = x + 1;

                if cube.map_edge(&net_grid, UP, y, x, 
                    paired_y, paired_x, y, x) 
                {
                    break;
                }
            }
        } 

        // map bottom edges
        for x in 0..4 {
            for y in (1..5).rev() {
                let paired_y = y;
                let paired_x = x + 1;

                if cube.map_edge(&net_grid, DOWN, y, x, 
                    paired_y, paired_x, y - 1, x) 
                {
                    break;
                }
            }
        } 
        
        // map left edges
        for y in 0..4 {
            for x in 0..4 {
                let paired_y = y + 1;
                let paired_x = x;

                if cube.map_edge(&net_grid, LEFT, y, x, 
                    paired_y, paired_x, y, x) 
                {
                    break;
                }
            }
        } 
        
        // map right edges
        for y in 0..4 {
            for x in (1..5).rev() {
                let paired_y = y + 1;
                let paired_x = x;

                if cube.map_edge(&net_grid, RIGHT, y, x, 
                    paired_y, paired_x, y, x - 1) 
                {
                    break;
                }
            }
        } 

        /*
        // TODO: Do this without hardcoding
        #[cfg(test)]
        {
            let wrapping_maps = &mut cube.wrapping_maps;
            wrapping_maps[1][2][RIGHT as usize] = Some(Box::new(
                |y0, x0| (2*SIZE, 4*SIZE-1 - (y0 - SIZE), DOWN)
            ) as _);
            wrapping_maps[2][2][DOWN as usize] = Some(Box::new(
                |y0, x0| (2*SIZE-1, 3*SIZE-1 - x0, UP)
            ) as _);
            wrapping_maps[1][1][UP as usize] = Some(Box::new(
                |y0, x0| (x0 - SIZE, 2*SIZE, RIGHT)
            ) as _);
        }
        */

        cube
    }

    fn map_edge(
        &mut self,
        net_grid: &NetGrid,
        dir0: usize,
        y: usize, 
        x: usize, 
        paired_y: usize, 
        paired_x: usize,
        chunk_y: usize, 
        chunk_x: usize, 
    ) -> bool 
    {
        let Some(&point) = net_grid.get_vertex(y, x) else {return false};

        let Some(&paired) = net_grid
            .get_vertex(paired_y, paired_x) else {return false};

        /*
        let (other_point_y, other_point_x) = 
        net_grid.find_other(point, y, x);

        let (other_paired_y, other_paired_x) = 
        net_grid.find_other(paired, paired_y, paired_x);
        */
        
        let (other_point_y, other_point_x, other_paired_y, other_paired_x) = 
        net_grid.find_other_edge(point, y, x, paired, paired_y, paired_x);

        let vertical;
        let new_dir;
        let y_correction;
        let x_correction;
        let same_parity;

        if other_point_x == other_paired_x {
            // other edge is vertical
            vertical = true;

            if net_grid.is_vertex(other_point_y, other_point_x + 1)
                && net_grid.is_vertex(other_paired_y, other_paired_x + 1)
            {
                // gonna go right
                new_dir = RIGHT;
                x_correction = 0;
            } else {
                // gonna go left
                new_dir = LEFT;
                x_correction = 1;
            }

            if other_paired_y > other_point_y {
                same_parity = true;
                y_correction = 0;
            } else {
                same_parity = false;
                y_correction = 1
            }

        } else {
            // other edge is horizontal
            vertical = false;

            if net_grid.is_vertex(other_point_y + 1, other_point_x) 
                && net_grid.is_vertex(other_paired_y + 1, other_paired_x)
            {
                // gonna go down
                new_dir = DOWN;
                y_correction = 0;
            } else {
                // gonna go up
                new_dir = UP;
                y_correction = 1;
            }

            if other_paired_x > other_point_x {
                same_parity = true;
                x_correction = 0;
            } else {
                same_parity = false;
                x_correction = 1
            }
        }

        self.wrapping_maps[chunk_y][chunk_x][dir0] = Some(Box::new(
            move |y0: usize, x0: usize| {
                let diff = if dir0 % 2 == 1 {
                    x0 - (x * SIZE)
                } else {
                    y0 - (y * SIZE) 
                };
                let y_base = other_point_y * SIZE - y_correction;
                let x_base = other_point_x * SIZE - x_correction;
                (
                    // wrapped y
                    if !vertical {
                        y_base
                    } else {
                        if same_parity {
                            y_base + diff
                        } else {
                            y_base - diff
                        }
                    },

                    // wrapped x
                    if vertical {
                        x_base
                    } else {
                        if same_parity {
                            x_base + diff
                        } else {
                            x_base - diff
                        }
                    },

                    // wrapped direction
                    new_dir
                )}
        ) as _);

        println!("Mapped edge {}{} from {} {},{}-{},{} to {} {},{}-{},{}",
            std::str::from_utf8(&[point]).unwrap(),
            std::str::from_utf8(&[paired]).unwrap(),
            match dir0 {
                0 => "RIGHT",
                1 => "DOWN",
                2 => "LEFT",
                3 => "UP",
                _ => unreachable!()
            },
            y,
            x,
            paired_y,
            paired_x,
            match new_dir {
                0 => "RIGHT",
                1 => "DOWN",
                2 => "LEFT",
                3 => "UP",
                _ => unreachable!()
            },
            other_point_y,
            other_point_x,
            other_paired_y,
            other_paired_x
        );

        true
    }

    fn wrap(&self, y0: usize, x0: usize, dir0: usize) -> (usize, usize, usize) {
        let (y_ch0, x_ch0) = Cube::<SIZE>::chunk_coords_from_coords((y0, x0));
        let wrap_map = self.wrapping_maps
            [y_ch0][x_ch0][dir0].as_ref().unwrap();
        wrap_map(y0, x0)
    }

    // eg. [0, SIZE[ -> 0 ; [SIZE, 2*SIZE[ -> 1 ; ...
    fn chunk_coords_from_coords(coords: (usize, usize)) -> (usize, usize) {
        (coords.0 / SIZE, coords.1 / SIZE)
    }
}

pub fn run2(input: &[u8]) -> u64 {
    let (mut input_instructions, board) = take_rows(input).unwrap();

    let mut x = board[0].1;
    let mut y = 0usize;
    let mut dir = 0usize;

    let board = Board{ board };
    
    // Testing
    // Cube net is described by the top left edges of the faces in the net
    // TODO: get the net from the parsing
    #[cfg(test)]
    let cube: Cube<EDGE_LEN> = Cube::from_net(&[
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 2),
        (2, 3),
    ]);
    
    #[cfg(not(test))]
    let cube: Cube<EDGE_LEN> = Cube::from_net(&[
        (0, 1),
        (0, 2),
        (1, 1),
        (2, 0),
        (2, 1),
        (3, 0),
    ]);

    println!("{y},{x}");

    loop {
        let Ok((input_rem, mov_count)) = 
            take_move(input_instructions) else {break};

        input_instructions = input_rem;

        let (mut mov_y, mut mov_x) = DIR_MOVS[dir as usize];

        for _ in 0..mov_count {
            let next_y = y.wrapping_add(mov_y);
            let next_x = x.wrapping_add(mov_x);
            match board.get(next_y, next_x) {
                Some(b'#') => break,
                Some(b'.') => {
                    y = next_y;
                    x = next_x;
                },
                None => { 
                    println!("Wrapping position: {y},{x}, direction: {}",
                        match dir {
                            0 => "RIGHT",
                            1 => "DOWN",
                            2 => "LEFT",
                            3 => "UP",
                            _ => unreachable!()
                        }
                    );
                    let (y_wrp, x_wrp, d_wrp) = cube.wrap(y, x, dir);
                    println!("Wrapped to {y_wrp},{x_wrp}, dir: {}", 
                        match d_wrp {
                            0 => "RIGHT",
                            1 => "DOWN",
                            2 => "LEFT",
                            3 => "UP",
                            _ => unreachable!()
                        }
                    );
                    match board.get(y_wrp, x_wrp).unwrap() {
                        b'#' => break,
                        b'.' => {
                            y = y_wrp;
                            x = x_wrp;
                            dir = d_wrp;
                            (mov_y, mov_x) = DIR_MOVS[dir as usize];
                        },
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),
            }
        }
        println!("{y},{x}");

        let Ok((i, trn)) = take_turn(input_instructions) else {break};
        input_instructions = i;

        match trn {
            b'L' => dir = (dir.wrapping_sub(1)) & 3,
            b'R' => dir = (dir + 1) & 3,
            _ => break,
        }
    }

    1000 * (y as u64 + 1) + 4 * (x as u64 + 1) + dir as u64
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
