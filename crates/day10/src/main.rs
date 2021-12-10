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
            Self { data: s.into() }
        }

        pub fn repaired(&self) -> Result<String, (usize, u8)> {
            let mut stack = Vec::new();

            for (i, &c) in self.data.as_bytes().iter().enumerate() {
                match c {
                    b'(' => stack.push(b')'),
                    b'[' => stack.push(b']'),
                    b'{' => stack.push(b'}'),
                    b'<' => stack.push(b'>'),
                    b')' | b'}' | b']' | b'>' => {
                        if let Some(p) = stack.pop() {
                            if p != c {
                                return Err((i, c));
                            }
                        }
                    }
                    _ => {}
                }
            }
            let mut data = String::new();
            data.extend(stack.into_iter().rev().map(|b| b as char));
            Ok(data)
        }
    }

    impl Display for Line {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
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
            self.lines
                .iter()
                .filter_map(|line| line.repaired().err())
                .map(|(_, i)| i)
                // .inspect(|&b| println!("{:?}", b as char))
                .map(error_score)
                .sum()
        }

        pub fn part2(&self) -> usize {
            let mut scores: Vec<_> = self.lines
                .iter()
                .filter_map(|line| line.repaired().ok())
                .map(|s| autocomplete_score(&s))
                .collect();

            scores.sort_unstable();

            scores[(scores.len()-1)/2]
        }
    }

    fn autocomplete_score(s: &str) -> usize {
        let mut score = 0;
        for c in s.chars() {
            score *= 5;
            score +=
            match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0
            };
        }
        score
    }

    fn error_score(c: u8) -> usize {
        match c {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
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
