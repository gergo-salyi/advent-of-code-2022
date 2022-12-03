const INPUT: &str = include_str!("../res/input01");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 69310
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 206104
}

fn run1(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|line_groups| {
            line_groups
                .lines()
                .fold(0u64, |acc, line| acc + line.parse::<u64>().unwrap())
        })
        .max()
        .unwrap()
}

struct LargestThree {
    values: [u64; 3],
    min_index: usize,
    min_value: u64,
}
impl LargestThree {
    fn new() -> Self {
        LargestThree {
            values: [0; 3],
            min_index: 0,
            min_value: 0,
        }
    }

    fn add_value(&mut self, candidate: u64) {
        if candidate > self.min_value {
            self.values[self.min_index] = candidate;

            let (new_min_index, new_min_value) = self
                .values
                .iter()
                .enumerate()
                .min_by_key(|&(_index, value)| value)
                .unwrap();
            self.min_index = new_min_index;
            self.min_value = *new_min_value;
        }
    }

    fn sum_values(&self) -> u64 {
        self.values.iter().sum()
    }
}

fn run2(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|line_groups| {
            line_groups
                .lines()
                .fold(0u64, |acc, line| acc + line.parse::<u64>().unwrap())
        })
        .fold(LargestThree::new(), |mut acc, value| {
            acc.add_value(value);
            acc
        })
        .sum_values()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../res/example01");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 24000)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 45000)
    }
}
