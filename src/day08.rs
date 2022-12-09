const INPUT: &[u8] = include_bytes!("../res/input08");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 1533
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 345744
}

pub fn run1(input: &[u8]) -> u64 {
    let mut answer = 0u64;
    let len = input.split(|&b| b == b'\n').next().unwrap().len();

    let mut forest: Vec<Vec<i8>> = vec![vec![-1; len]; len];

    for (i, line) in input.split(|&b| b == b'\n').enumerate() {
        for (j, height) in line.iter().enumerate() {
            forest[i][j] = ((*height) - b'0') as i8;
        }
    }

    let mut visiblity: Vec<Vec<u8>> = vec![vec![0; len]; len];

    for i in 0..len {
        let mut heighest_seen = -1i8;
        for j in 0..len {
            let height = forest[i][j];
            if height > heighest_seen {
                visiblity[i][j] = 1;
                if height == 9 {
                    break;
                }
            }
            heighest_seen = heighest_seen.max(height);
        }
    }

    for i in 0..len {
        let mut heighest_seen = -1i8;
        for j in (0..len).rev() {
            let height = forest[i][j];
            if height > heighest_seen {
                visiblity[i][j] = 1;
                if height == 9 {
                    break;
                }
            }
            heighest_seen = heighest_seen.max(height);
        }
    }

    for i in 0..len {
        let mut heighest_seen = -1i8;
        for j in 0..len {
            let height = forest[j][i];
            if height > heighest_seen {
                visiblity[j][i] = 1;
                if height == 9 {
                    break;
                }
            }
            heighest_seen = heighest_seen.max(height);
        }
    }

    for i in 0..len {
        let mut heighest_seen = -1i8;
        for j in (0..len).rev() {
            let height = forest[j][i];
            if height > heighest_seen {
                visiblity[j][i] = 1;
                if height == 9 {
                    break;
                }
            }
            heighest_seen = heighest_seen.max(height);
        }
    }

    #[allow(clippy::needless_range_loop)]
    for i in 0..len {
        for j in 0..len {
            answer += visiblity[i][j] as u64;
        }
    }

    answer
}

pub fn run2(input: &[u8]) -> u64 {
    let mut answer = 0u64;

    let len = input.split(|&b| b == b'\n').next().unwrap().len();

    let mut forest: Vec<Vec<i8>> = vec![vec![-1; len]; len];

    for (i, line) in input.split(|&b| b == b'\n').enumerate() {
        for (j, h) in line.iter().enumerate() {
            forest[i][j] = ((*h) - b'0') as i8;
        }
    }

    for a in 1..(len - 1) {
        for b in 1..(len - 1) {
            let self_height = forest[a][b];

            let mut score_right = 0u64;
            let mut max_in_line = -1i8;
            for i in 1..len {
                let Some(&h) = forest[a].get(b+i) else { break };

                score_right += 1;

                if h >= self_height {
                    break;
                };

                max_in_line = max_in_line.max(h);
            }

            let mut score_left = 0u64;
            let mut max_in_line = -1i8;
            for i in 1..len {
                let Some(&h) = forest[a].get(b.wrapping_sub(i)) else { break };

                score_left += 1;

                if h >= self_height {
                    break;
                };

                max_in_line = max_in_line.max(h);
            }

            let mut score_down = 0u64;
            let mut max_in_line = -1i8;
            for i in 1..len {
                let Some(h) = forest.get(a+i).map(|e| e[b]) else { break };

                score_down += 1;

                if h >= self_height {
                    break;
                };

                max_in_line = max_in_line.max(h);
            }

            let mut score_up = 0u64;
            let mut max_in_line = -1i8;
            for i in 1..len {
                let Some(h) = forest.get(a.wrapping_sub(i))
                    .map(|e| e[b]) else { break };

                score_up += 1;

                if h >= self_height {
                    break;
                };

                max_in_line = max_in_line.max(h);
            }

            answer =
                answer.max(score_right * score_left * score_down * score_up);
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example08");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 21)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 8)
    }
}
