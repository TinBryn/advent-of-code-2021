use crate::input::{Input, Snailfish};


#[derive(Debug, Clone)]
pub struct Problem {
    numbers: Vec<Snailfish>,
}

impl Problem {
    pub fn from_input(input: Input) -> Self {
        Self {
            numbers: input.numbers,
        }
    }

    pub fn part1(&self) -> usize {
        self.common()
    }

    pub fn part2(&self) -> usize {
        self.common()
    }

    pub fn common(&self) -> usize {
        self.numbers
            .iter()
            .cloned()
            .reduce(|a, ref b| a.add(b))
            .map(|n| n.magnitude())
            .unwrap_or_default()
    }
}
