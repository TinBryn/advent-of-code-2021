use std::{collections::VecDeque, fmt::Display};

use crate::input::Input;

#[derive(Debug, Clone)]
pub struct Problem {
    data: Vec<u8>,
    width: usize,
}

impl Problem {
    pub fn from_input(input: Input) -> Self {
        Self {
            data: input.data,
            width: input.width,
        }
    }

    pub fn part1(&self) -> usize {
        let mut current = self.clone();
        let mut flashes = 0;

        const STEPS: usize = 100;
        for _ in 0..STEPS {
            let u = current.update();
            flashes += u;
        }

        flashes
    }

    pub fn part2(&self) -> usize {
        let mut current = self.clone();
        let len = self.data.len();

        for i in 1.. {
            if current.update() == len {
                return i;
            }
        }

        unreachable!()
    }

    pub fn update(&mut self) -> usize {
        let mut flashing = VecDeque::new();
        for (i, squid) in self.data.iter_mut().enumerate() {
            *squid += 1;
            if *squid > 9 {
                *squid = 0;
                flashing.push_back(i);
            }
        }

        while let Some(flasher) = flashing.pop_front() {
            for viewer in neighbours(flasher, self.width) {
                if self.data[viewer] > 0 {
                    self.data[viewer] += 1;
                    if self.data[viewer] > 9 {
                        self.data[viewer] = 0;
                        flashing.push_back(viewer)
                    }
                }
            }
        }

        self.data.iter().filter(|&&i| i == 0).count()
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, q) in self.data.iter().enumerate() {
            write!(f, "{}", q)?;
            if (i + 1) % self.width == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

pub fn neighbours(flasher: usize, width: usize) -> impl Iterator<Item = usize> {
    let (x, y) = {
        let x = flasher % width;
        let y = flasher / width;
        (x, y)
    };

    let (minx, maxx) = edge_cases(x, width);
    let (miny, maxy) = edge_cases(y, width);

    (minx..=maxx)
        .flat_map(move |nx| (miny..=maxy).map(move |ny| (nx, ny)))
        .filter(move |&(nx, ny)| nx != x || ny != y)
        .map(move |(x, y)| x + y * width)
}

fn edge_cases(x: usize, width: usize) -> (usize, usize) {
    let min = if x > 0 { x - 1 } else { x };
    let max = if x < (width - 1) { x + 1 } else { x };
    (min, max)
}
