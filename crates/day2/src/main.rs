const INPUT: &str = include_str!("input.txt");

#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    // println!("input: {:?}", INPUT);
    let input = Input::parse_input(INPUT);
    println!("problem1: {}", solve_problem1(&input));
    println!("problem2: {}", solve_problem2(&input));
}

struct Input<'a> {
    input: Vec<(&'a str, i32)>,
}

impl<'a> Input<'a> {
    fn parse_input(input: &'a str) -> Self {
        Input {
            input: input
                .lines()
                .map(|line| line.split_ascii_whitespace())
                .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
                .map(|(dir, n)| (dir, n.parse().unwrap()))
                .collect(),
        }
    }
}

fn solve_problem1(input: &Input) -> i32 {
    let mut depth = 0;
    let mut position = 0;
    for &(dir, amount) in &input.input {
        match dir {
            "forward" => position += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => println!("unknown command {}", dir),
        }
    }
    depth * position
}

fn solve_problem2(input: &Input) -> i32 {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    for &(dir, amount) in &input.input {
        match dir {
            "forward" => {
                position += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => println!("unknown command {}", dir),
        }
    }
    depth * position
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2, Input};

    const INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn problem1() {
        let expected = 150;
        let actual = solve_problem1(&Input::parse_input(INPUT));
        assert_eq!(expected, actual);
    }

    #[test]
    // #[ignore = "day not available"]
    fn problem2() {
        let expected = 900;
        let actual = solve_problem2(&Input::parse_input(INPUT));
        assert_eq!(expected, actual);
    }
}
