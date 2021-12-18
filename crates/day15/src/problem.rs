use std::collections::BinaryHeap;

use grid::Grid;

use crate::input::Input;

#[derive(Debug, Clone, Copy)]
pub struct Candidate {
    pub cost: usize,
    pub point: (usize, usize),
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Candidate {}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub grid: Grid<u8>,
}

impl Problem {
    pub fn from_input(input: Input) -> Self {
        let width = if let Some(first) = input.lines.first() {
            first.data.len()
        } else {
            let grid = Grid::new(0, 0, 0);
            return Self { grid };
        };

        let mut grid = Grid::new(0, width, input.lines.len());

        for (y, line) in input.lines.into_iter().enumerate() {
            for (x, cost) in line.data.into_iter().enumerate() {
                grid[(x, y)] = cost;
            }
        }

        Self { grid }
    }

    pub fn part1(&self) -> usize {
        self.common(1)
    }

    pub fn part2(&self) -> usize {
        self.common(5)
    }

    pub fn common(&self, scale: usize) -> usize {
        let width = self.grid.width() * scale;
        let height = self.grid.height() * scale;
        let mut closed_set = Grid::new(usize::MAX, width, height);
        let mut open_set = BinaryHeap::<Candidate>::new();

        closed_set[(0, 0)] = 0;
        // setup initial values in heap
        for point in [(0, 1), (1, 0)] {
            let item = Candidate {
                cost: self.grid[point] as usize,
                point,
            };
            closed_set[point] = item.cost;
            open_set.push(item)
        }

        while let Some(Candidate { cost, point }) = open_set.pop() {
            // right
            if point.0 < width - 1 {
                let point = (point.0 + 1, point.1);
                self.push_new_point(cost, point, &mut closed_set, &mut open_set);
            }

            //down
            if point.1 < height - 1 {
                let point = (point.0, point.1 + 1);
                self.push_new_point(cost, point, &mut closed_set, &mut open_set);
            }

            // left
            if point.0 > 0 {
                let point = (point.0 - 1, point.1);
                self.push_new_point(cost, point, &mut closed_set, &mut open_set);
            }

            // up
            if point.1 > 0 {
                let point = (point.0, point.1 - 1);
                self.push_new_point(cost, point, &mut closed_set, &mut open_set);
            }
        }
        // println!("\n{}", closed_set);

        closed_set[(width - 1, height - 1)]
    }

    fn push_new_point(
        &self,
        previous_cost: usize,
        point: (usize, usize),
        closed_set: &mut Grid<usize>,
        open_set: &mut BinaryHeap<Candidate>,
    ) {
        let width = self.grid.width();
        let height = self.grid.height();
        let mod_point = (point.0 % width, point.1 % height);
        let offset = (point.0 / width + point.1 / height) as u8;
        let grid_cost = (self.grid[mod_point] + offset - 1) % 9 + 1;
        let cost = previous_cost + grid_cost as usize;
        if cost < closed_set[point] {
            closed_set[point] = cost;
            open_set.push(Candidate { cost, point });
        }
    }
}
