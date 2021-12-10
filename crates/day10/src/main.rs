use crate::input::{Input, Problem};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::from_str(INPUT);
    let data: Problem = input.into();

    println!("problem1: {}", data.part1());
    println!("problem2: {}", data.part2());
}

mod input {
    use std::fmt::Display;

    #[derive(Debug, Clone)]
    pub struct Line {
        data: String,
    }

    impl Line {
        pub fn from_str(s: &str) -> Self {
            Self {
                data: s.trim().into(),
            }
        }

        pub fn completion(&self) -> Result<String, char> {
            let mut stack = Vec::new();

            for c in self.data.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    _ => {
                        if Some(c) != stack.pop() {
                            return Err(c);
                        }
                    }
                }
            }
            Ok(stack.into_iter().rev().collect())
        }
    }

    #[derive(Debug)]
    pub struct Input {
        pub lines: Vec<Line>,
    }

    impl Input {
        pub fn from_str(s: &str) -> Self {
            Self {
                lines: s.trim().lines().map(Line::from_str).collect(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Problem {
        lines: Vec<Line>,
    }

    impl Problem {
        pub fn part1(&self) -> usize {
            self.scores().filter_map(|comp| comp.err()).sum()
        }

        pub fn part2(&self) -> usize {
            let mut scores: Vec<_> = self.scores().filter_map(|comp| comp.ok()).collect();
            scores.sort_unstable();
            scores[(scores.len() - 1) / 2]
        }

        fn scores(&'_ self) -> impl Iterator<Item = Result<usize, usize>> + '_ {
            self.lines.iter().map(|line| {
                line.completion()
                    .map(autocomplete_score)
                    .map_err(error_score)
            })
        }
    }

    fn autocomplete_score(s: String) -> usize {
        s.chars().fold(0, |score, c| {
            score * 5
                + match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                }
        })
    }

    fn error_score(c: char) -> usize {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }

    impl From<Input> for Problem {
        fn from(input: Input) -> Self {
            Problem { lines: input.lines }
        }
    }

    impl Display for Problem {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
}
#[cfg(test)]
mod test {
    use crate::{input::Problem, Input};
    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn problem1() {
        let input = Input::from_str(EXAMPLE);
        let expected = 26397;
        let data: Problem = input.into();
        let actual = data.part1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let input = Input::from_str(EXAMPLE);
        let data: Problem = input.into();
        let actual = data.part2();
        let expected = 288957;
        assert_eq!(expected, actual);
    }
}
