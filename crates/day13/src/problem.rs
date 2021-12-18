use grid::Grid;

use crate::input::{Fold, Input, Point};

fn fold(grid: &Grid<u8>, fold: Fold) -> Grid<u8> {
    match fold {
        Fold::X(pos) => fold_x(grid, pos),
        Fold::Y(pos) => fold_y(grid, pos),
    }
}

fn fold_x(grid: &Grid<u8>, pos: usize) -> Grid<u8> {
    let left = pos;
    let right = grid.width() - pos - 1;
    let width = left.max(right);
    let mut result = Grid::new(b'.', width, grid.height());

    for y in 0..grid.height() {
        for x in 0..width {
            let x2 = 2 * width - x;
            if grid[(x, y)] == b'#' || (x2 < grid.width() && grid[(x2, y)] == b'#') {
                result[(x, y)] = b'#';
            }
        }
    }

    result
}

fn fold_y(grid: &Grid<u8>, pos: usize) -> Grid<u8> {
    let top = pos;
    let bottom = grid.height() - pos - 1;
    let height = top.max(bottom);
    let mut result = Grid::new(b'.', grid.width(), height);

    for x in 0..grid.width() {
        for y in 0..height {
            let y2 = 2 * height - y;
            if grid[(x, y)] == b'#' || (y2 < grid.height() && grid[(x, y2)] == b'#') {
                result[(x, y)] = b'#';
            }
        }
    }

    result
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
                grid = crate::problem::fold(&grid, fold);
            }
            println!("{}", grid);
        }
        0
    }

    pub fn common(&self) -> usize {
        let grid = crate::problem::fold(&self.grid, self.folds[0]);
        count_points(grid)
    }
}

fn count_points(grid: Grid<u8>) -> usize {
    grid.into_iter().filter(|&p| p == b'#').count()
}
