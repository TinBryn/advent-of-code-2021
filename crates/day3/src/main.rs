use input::{Input, Line};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::from_str(INPUT);
    // println!("input: {:?}", input);
    println!("problem1: {}", solve_problem1(&input.inner));
    // println!("problem2: {}", solve_problem2(&input.inner));
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
            .map(|b| if b > &0 { 1usize } else { 0 })
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

fn solve_problem2(input: &[Line]) -> i32 {
    todo!("{:?}", input)
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
        let expected = 0;
        let actual = solve_problem2(&input.inner);
        assert_eq!(expected, actual);
    }
}
