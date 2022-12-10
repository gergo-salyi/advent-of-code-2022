use atoi::FromRadix10Signed;
use std::io::Write;

const INPUT: &[u8] = include_bytes!("../res/input10");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 13180
}

#[allow(unused)]
pub fn part2() {
    // EZFCHJAB
    std::io::stdout().lock().write_all(&run2(INPUT)[0..245]).unwrap();
    println!();
}

pub fn run1(input: &[u8]) -> u64 {
    let mut lines = input.split(|&b| b == b'\n');

    let mut x = 1i64;
    let mut p = 0i64;
    let mut add_in_progress = false;

    let mut signal_strength = Vec::with_capacity(6);

    for cycle in 1.. {
        if (cycle - 20) % 40 == 0 {
            signal_strength.push(x * cycle);
        }

        if !add_in_progress {
            let line = lines.next().unwrap();
            if line.is_empty() {
                break;
            };

            if line[0] == b'a' {
                // start addition
                p = i64::from_radix_10_signed(&line[5..]).0;
                add_in_progress = true;
            } else {
                // noop
            }
        } else {
            // finish addition
            x += p;
            add_in_progress = false;
        }
    }

    signal_strength.iter().sum::<i64>() as u64
}

pub fn run2(input: &[u8]) -> Vec<u8> {
    let mut lines = input.split(|&b| b == b'\n');

    let mut x = 1i64;
    let mut p = 0i64;
    let mut add_in_progress = false;

    let mut output = Vec::with_capacity(256);

    for cycle in 1.. {
        let crt = (cycle - 1) % 40;

        if (crt - x).abs() < 2 {
            output.push(b'#')
        } else {
            output.push(b'.')
        }

        if crt == 39 {
            output.push(b'\n')
        }

        if !add_in_progress {
            let line = lines.next().unwrap();
            if line.is_empty() {
                break;
            };

            if line[0] == b'a' {
                // start addition
                p = i64::from_radix_10_signed(&line[5..]).0;
                add_in_progress = true;
            } else {
                // noop
            }
        } else {
            // finish addition
            x += p;
            add_in_progress = false;
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example10");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 13140)
    }

    #[test]
    fn test2() {
        assert_eq!(
            &run2(EXAMPLE)[0..245],
            b"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .as_slice()
        )
    }
}
