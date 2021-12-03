use input::{Input, Line};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::from_str(INPUT);
    // println!("input: {:?}", input);
    println!("problem1: {}", solve_problem1(&input.inner));
    println!("problem2: {}", solve_problem2(&input.inner));
}

mod input {
    #[derive(Debug, Clone)]
    pub struct Line {
        pub bits: Vec<u8>,
    }

    impl Line {
        pub fn from_str(s: &str) -> Self {
            let bits = s
                .trim()
                .chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!(),
                })
                .collect();
            Self { bits }
        }
    }

    #[derive(Debug)]
    pub struct Input {
        pub inner: Vec<Line>,
    }

    impl Input {
        pub fn from_str(s: &str) -> Self {
            Self {
                inner: s.trim().lines().map(Line::from_str).collect(),
            }
        }
    }
}

fn solve_problem1(input: &[Line]) -> usize {
    if let Some((first, rest)) = input.split_first() {
        let mut count = first
            .bits
            .iter()
            .copied()
            .map(|i| i as i16)
            .collect::<Vec<_>>();
        for line in rest {
            for (c, &bit) in count.iter_mut().zip(&line.bits) {
                if bit == 0 {
                    *c -= 1;
                } else {
                    *c += 1;
                }
            }
        }

        let mask: Vec<_> = count
            .iter()
            .map(|b| if b >= &0 { 1usize } else { 0 })
            .collect();

        let gamma: usize = mask
            .iter()
            .rev()
            .enumerate()
            .map(|(i, b)| 2usize.pow(i as u32) * b)
            .sum();
        let epsilon: usize = mask
            .iter()
            .map(|b| 1 - b)
            .rev()
            .enumerate()
            .map(|(i, b)| 2usize.pow(i as u32) * b)
            .sum();

        return gamma * epsilon;
    }
    todo!("{:?}", input)
}

fn solve_problem2(input: &[Line]) -> usize {
    let len = input[0].bits.len();

    let criteria = oxygen_criteria;

    let oxygen = fun_name(input, len, criteria);

    let criteria = co2_criteria;

    let mut filtered = input.to_vec();
    for bit in 0..len {
        if filtered.len() == 1 {
            break;
        }
        filtered = filter_numbers(&filtered, bit, criteria);
    }
    let co2: usize = filtered[0]
        .bits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| 2usize.pow(i as u32) * *b as usize)
        .sum();

    oxygen * co2
}

fn fun_name(
    input: &[Line],
    len: usize,
    criteria: fn(&[Line], usize, usize, &[isize]) -> bool,
) -> usize {
    let mut filtered = input.to_vec();
    for bit in 0..len {
        if filtered.len() == 1 {
            break;
        }
        filtered = filter_numbers(&filtered, bit, criteria);
    }
    let value = filtered[0]
        .bits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| 2usize.pow(i as u32) * *b as usize)
        .sum();
    let oxygen: usize = value;
    oxygen
}

fn gen_mask(input: &[Line]) -> Option<Vec<isize>> {
    input.split_first().map(|(first, rest)| {
        let mut count = first
            .bits
            .iter()
            .copied()
            .map(|i| i as i16 * 2 - 1)
            .collect::<Vec<_>>();
        for line in rest {
            for (c, &bit) in count.iter_mut().zip(&line.bits) {
                if bit == 0 {
                    *c -= 1;
                } else {
                    *c += 1;
                }
            }
        }
        count
            .iter()
            .map(|&b| match b {
                0 => 0,
                _ if b < 0 => -1,
                _ => 1,
            })
            .collect()
    })
}

fn filter_numbers<F: Fn(&[Line], usize, usize, &[isize]) -> bool>(
    input: &[Line],
    bit: usize,
    predicate: F,
) -> Vec<Line> {
    let mask = gen_mask(input).unwrap();

    if input.len() == 1 {
        return input.to_vec();
    }
    let mut result = input.to_vec();
    let mut i = 0;
    while i < result.len() {
        if predicate(&result, i, bit, &mask) {
            result.remove(i);
        } else {
            i += 1;
        }
    }
    result
}

fn oxygen_criteria(result: &[Line], i: usize, bit: usize, mask: &[isize]) -> bool {
    if mask[bit] == 1 || mask[bit] == 0 {
        result[i].bits[bit] == 0
    } else {
        result[i].bits[bit] == 1
    }
}

fn co2_criteria(result: &[Line], i: usize, bit: usize, mask: &[isize]) -> bool {
    let m = mask[bit];
    let check = if m == 1 {
        0
    } else if m == -1 {
        1
    } else {
        0
    };

    result[i].bits[bit] != check
}

#[cfg(test)]
mod test {
    use crate::{input, solve_problem1, solve_problem2};

    const TEST: &str = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    #[test]
    fn problem1() {
        let input = input::Input::from_str(TEST);
        let expected = 198;
        let actual = solve_problem1(&input.inner);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let input = input::Input::from_str(TEST);
        let expected = 220;
        let actual = solve_problem2(&input.inner);
        assert_eq!(expected, actual);
    }
}
