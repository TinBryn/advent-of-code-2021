use input::{Input, Line};

const INPUT: &str = include_str!("input.txt");


fn main() {
    let input = Input::from_str(INPUT);
    println!("problem1: {}", solve_problem1(&input));
    println!("problem2: {}", solve_problem2(&input));
}

mod input {
    #[derive(Debug)]
    pub struct Line(pub String, pub i32);

    impl Line {
        pub fn from_str(s: &str) -> Self {
            let mut parts = s.split_ascii_whitespace();
            let s = parts.next().unwrap().into();
            let n = parts.next().unwrap().parse().unwrap();

            Self(s, n)
        }
    }

    #[derive(Debug)]
    pub struct Input {
        pub inner: Vec<Line>,
    }

    impl Input {
        pub fn from_str(s: &str) -> Self {
            Self {
                inner: s.lines().map(Line::from_str).collect(),
            }
        }
    }
}

fn solve_problem1(input: &Input) -> i32 {
    let mut depth = 0;
    let mut position = 0;
    for Line(dir, amount) in &input.inner {
        match dir.as_str() {
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
    for Line(dir, amount) in &input.inner {
        match dir.as_str() {
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
    use crate::{input, solve_problem1, solve_problem2};

    const INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn problem1() {
        let expected = 150;
        let actual = solve_problem1(&input::Input::from_str(INPUT));
        assert_eq!(expected, actual);
    }

    #[test]
    // #[ignore = "day not available"]
    fn problem2() {
        let expected = 900;
        let actual = solve_problem2(&input::Input::from_str(INPUT));
        assert_eq!(expected, actual);
    }
}
