use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::input::{Fold, Input, Point};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    grid: Vec<T>,
    width: usize,
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let index = x + y * self.width;
        &self.grid[index]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let index = x + y * self.width;
        &mut self.grid[index]
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Clone> Grid<T> {
    fn new(elem: T, width: usize, height: usize) -> Grid<T> {
        let grid = vec![elem; width * height];
        Grid { grid, width }
    }

    pub fn height(&self) -> usize {
        self.grid.len() / self.width
    }
}

impl Grid<u8> {
    pub fn fold(&self, fold: Fold) -> Grid<u8> {
        match fold {
            Fold::X(pos) => self.fold_x(pos),
            Fold::Y(pos) => self.fold_y(pos),
        }
    }

    pub fn fold_y(&self, pos: usize) -> Grid<u8> {
        let top = pos;
        let bottom = self.height() - pos - 1;
        let height = top.max(bottom);
        let mut result = Grid::new(b'.', self.width, height);

        for x in 0..self.width {
            for y in 0..height {
                let d = height - y;
                let y2 = height + d;
                if self[(x, y)] == b'#' || (y2 < self.height() && self[(x, y2)] == b'#') {
                    result[(x, y)] = b'#';
                }
            }
        }

        result
    }

    pub fn fold_x(&self, pos: usize) -> Grid<u8> {
        let left = pos;
        let right = self.width - pos - 1;
        let width = left.max(right);
        let mut result = Grid::new(b'.', width, self.height());

        for y in 0..self.height() {
            for x in 0..width {
                let d = width - x;
                let x2 = width + d;
                if self[(x, y)] == b'#' || (x2 < self.width && self[(x2, y)] == b'#') {
                    result[(x, y)] = b'#';
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub grid: Grid<u8>,
    pub folds: Vec<Fold>,
}

impl Problem {
    pub fn from_input(input: Input) -> Self {
        let mut maxx = usize::MIN;
        let mut maxy = usize::MIN;

        for &Point { x, y } in &input.points {
            if x > maxx {
                maxx = x;
            }
            maxy = maxy.max(y);
        }

        let mut grid = Grid::new(b'.', maxx + 1, maxy + 1);

        for Point { x, y } in input.points {
            grid[(x, y)] = b'#';
        }

        Self {
            grid,
            folds: input.folds,
        }
    }

    pub fn part1(&self) -> usize {
        self.common()
    }

    pub fn part2(&self) -> usize {
        {
            let mut grid = self.grid.clone();

            for &fold in &self.folds {
                grid = grid.fold(fold);
            }
            println!("{}", grid);
        }
        0
    }

    pub fn common(&self) -> usize {
        let grid = self.grid.fold(self.folds[0]);
        count_points(grid)
    }
}

fn count_points(grid: Grid<u8>) -> usize {
    grid.grid.into_iter().filter(|&p| p == b'#').count()
}
