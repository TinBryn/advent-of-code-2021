use crate::input::{Input, Line};

#[derive(Debug, Clone)]
pub struct Problem {
    lines: Vec<Line>,
}

impl Problem {
    pub fn from_input(input: Input) -> Self {
        Self { lines: input.lines }
    }

    pub fn part1(&self) -> usize {
        self.common()
    }

    pub fn part2(&self) -> usize {
        self.common()
    }

    pub fn common(&self) -> usize {
        println!("{}", self.lines.len());
        todo!()
    }
}
