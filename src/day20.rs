use atoi::FromRadix10Signed;

const INPUT: &[u8] = include_bytes!("../res/input20");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 5904
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 8332585833851
}

pub fn run1(input: &[u8]) -> i64 {
    let mut numbers = Vec::new();

    for line in input.trim_ascii_end().split(|&b| b == b'\n') {
        let number = i16::from_radix_10_signed(line).0;
        numbers.push(number);
    }

    let numbers = numbers;
    let len = numbers.len() as i16;

    let mut seqs: Vec<i16> = (0..len).collect();

    for seq in 0..len {
        let n = numbers[seq as usize];

        if n == 0 { continue; }

        let i = seqs.iter().position(|&e| e == seq).unwrap();

        seqs.remove(i);

        let k = (n % (len - 1)).unsigned_abs() as usize;
        
        if n > 0 {
            seqs.rotate_left(k);
        } else {
            seqs.rotate_right(k);
        }

        seqs.insert(i, seq);
        
    }

    let len = len as usize;
    let mut numbers_de = vec![i16::MIN; len];
    for (i, seq) in seqs.iter().enumerate() {
        numbers_de[i] = numbers[*seq as usize]
    }

    let i = numbers_de.iter().position(|&e| e == 0).unwrap();
    let n1 = numbers_de[(i + 1000) % len];
    let n2 = numbers_de[(i + 2000) % len];
    let n3 = numbers_de[(i + 3000) % len];

    (n1 + n2 + n3) as i64
}

pub fn run2(input: &[u8]) -> i64 {
    let mut numbers = Vec::new();

    for line in input.trim_ascii_end().split(|&b| b == b'\n') {
        let number = i16::from_radix_10_signed(line).0;
        numbers.push(number);
    }

    let numbers: Vec<i64> = numbers.iter().map(|&n| n as i64 * 811589153).collect();
    let len = numbers.len() as i16;

    let mut seqs: Vec<i16> = (0..len).collect();

    for _ in 0..10 {
        for seq in 0..len {
            let n = numbers[seq as usize];

            if n == 0 { continue; }

            let i = seqs.iter().position(|&e| e == seq).unwrap();

            seqs.remove(i);

            let k = (n % (len as i64 - 1)).unsigned_abs() as usize;

            if n > 0 {
                seqs.rotate_left(k);
            } else {
                seqs.rotate_right(k);
            }

            seqs.insert(i, seq);

        }
    }

    let len = len as usize;
    let mut numbers_de = vec![i64::MIN; len];
    for (i, seq) in seqs.iter().enumerate() {
        numbers_de[i] = numbers[*seq as usize]
    }

    let i = numbers_de.iter().position(|&e| e == 0).unwrap();
    let n1 = numbers_de[(i + 1000) % len];
    let n2 = numbers_de[(i + 2000) % len];
    let n3 = numbers_de[(i + 3000) % len];

    n1 + n2 + n3
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example20");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 3)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 1623178306)
    }
}
