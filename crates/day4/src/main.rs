use input::Input;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::from_str(INPUT);
    println!("input: {:?}", input);
    // println!("problem1: {}", solve_problem1(input));
    // println!("problem2: {}", solve_problem2(input));
}

#[allow(unused)]
mod input {
    #[derive(Debug)]
    pub struct Line {}

    impl Line {
        pub fn from_str(s: &str) -> Self {
            Self {}
        }
    }

    #[derive(Debug)]
    pub struct Input {
        inner: Vec<Line>,
    }

    impl Input {
        pub fn from_str(s: &str) -> Self {
            Self {
                inner: s.lines().map(Line::from_str).collect(),
            }
        }
    }
}

#[allow(unused)]
fn solve_problem1(input: &Input) -> i32 {
    todo!()
}

#[allow(unused)]
fn solve_problem2(input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{input::Input, solve_problem1, solve_problem2};

    #[test]
    #[ignore = "day not available"]
    fn problem1() {
        let input = "";
        let input = Input::from_str(input);
        let expected = 0;
        let actual = solve_problem1(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore = "day not available"]
    fn problem2() {
        let input = "";
        let input = Input::from_str(input);
        let expected = 0;
        let actual = solve_problem2(&input);
        assert_eq!(expected, actual);
    }
}