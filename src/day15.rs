use ahash::{HashSet, HashSetExt};
use nom::{
    bytes::complete::{take, take_till},
    character::complete::i32 as nom_i32,
    combinator::map,
    multi::many1,
    sequence::tuple,
    IResult,
};

const INPUT: &[u8] = include_bytes!("../res/input15");

#[cfg(test)]
const TARGET_ROW: i32 = 10;
#[cfg(not(test))]
const TARGET_ROW: i32 = 2000000;

#[cfg(test)]
const SEARCH_LIMIT: i32 = 20;
#[cfg(not(test))]
const SEARCH_LIMIT: i32 = 4000000;

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 4879972
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 12525726647448
}

fn next_decimal(input: &[u8]) -> IResult<&[u8], i32> {
    let (input, _) = take_till(|b| b == b'=')(input)?;
    let (input, _) = take(1usize)(input)?;
    nom_i32(input)
}

fn sensor(input: &[u8]) -> IResult<&[u8], (i32, i32, i32, i32)> {
    tuple((next_decimal, next_decimal, next_decimal, next_decimal))(input)
}

/// (sensor\_x, sensor\_y, sensor\_range+1)
fn parse(input: &[u8]) -> IResult<&[u8], Vec<(i32, i32, i32)>> {
    many1(map(sensor, |s| {
        (s.0, s.1, (s.0.abs_diff(s.2) + s.1.abs_diff(s.3)) as i32 + 1)
    }))(input)
}

pub fn run1(input: &[u8]) -> u64 {
    let mut input = input;

    let mut answer = 0i32;

    let mut coverages: Vec<(i32, bool)> = Vec::new();

    let mut beacons_in_target_row = Vec::new();

    while let Ok((next_input, (sx, sy, bx, by))) = sensor(input) {
        input = next_input;
        let range = (sx.abs_diff(bx) + sy.abs_diff(by)) as i32;

        if by == TARGET_ROW {
            beacons_in_target_row.push(bx);
        }

        let reach_in_target_row = range - sy.abs_diff(TARGET_ROW) as i32;
        if reach_in_target_row > 0 {
            coverages.push((sx - reach_in_target_row, true));
            coverages.push((sx + reach_in_target_row + 1, false));
        }
    }

    coverages.sort_unstable();

    let mut covering_sensors = 1;
    let mut coverages_iter = coverages.iter();
    let mut last_coverage_point = coverages_iter.next().unwrap();
    for coverage_point in coverages_iter {
        if covering_sensors > 0 {
            let range = coverage_point.0 - last_coverage_point.0;
            answer += range;
        }
        match coverage_point.1 {
            true => covering_sensors += 1,
            false => covering_sensors -= 1,
        };
        last_coverage_point = coverage_point;
    }

    beacons_in_target_row.sort_unstable();
    beacons_in_target_row.dedup();
    answer -= beacons_in_target_row.len() as i32;

    answer as u64
}

pub fn run2(input: &[u8]) -> u64 {
    let sensors = parse(input).unwrap().1;

    // Each sensor has the margin points on
    // y =  x + sensor_y - sensor_x - (range + 1) ; x: [sensor_x, sensor_x + range]
    // y =  x + sensor_y - sensor_x + (range + 1) ; x: [sensor_x - range, sensor_x]
    // y = -x + sensor_y + sensor_x +- (range + 1)
    //
    // x + p1 = -x + p2
    // 2x = p2 - p1
    // x = (p2 - p1) / 2
    // y = (p2 + p1) / 2

    // TODO: store lines with slope, range, parity

    // lines with slope = +1 and parity +
    let mut lines1a = HashSet::new();
    // lines with slope = +1 and parity -
    let mut lines1b = HashSet::new();
    // lines with slope = -1 and parity +
    let mut lines2a = HashSet::new();
    // lines with slope = -1 and parity -
    let mut lines2b = HashSet::new();

    for (sx, sy, sr) in &sensors {
        lines1a.insert(sy - sx + sr);
        lines1b.insert(sy - sx - sr);
        lines2a.insert(sy + sx + sr);
        lines2b.insert(sy + sx - sr);
    }

    // println!(
    //     "slope: +1: {} lines, slope: -1: {} lines",
    //     lines1a.intersection(&lines1b).count(),
    //     lines2a.intersection(&lines2b).count()
    // );

    let mut candidates = Vec::new();

    for p1 in lines1a.intersection(&lines1b) {
        for p2 in lines2a.intersection(&lines2b) {

            let x = (p2 - p1) / 2;
            let y = (p2 + p1) / 2;

            if (0..=SEARCH_LIMIT).contains(&x)
                && (0..=SEARCH_LIMIT).contains(&y)
            {
                candidates.push((x, y));
            }
        }
    }

    'points: for (x, y) in candidates {
        for (sx, sy, sr) in &sensors {
            if (x.abs_diff(*sx) + y.abs_diff(*sy)) < *sr as u32 {
                continue 'points;
            }
        }
        return 4000000 * x as u64 + y as u64;
    }

    0u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example15");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 26)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 56000011)
    }
}
