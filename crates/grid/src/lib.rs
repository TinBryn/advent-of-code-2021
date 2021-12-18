use std::{ops::{Index, IndexMut}, fmt::Display};

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

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let index = x + y * self.width;
        &mut self.grid[index]
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(elem: T, width: usize, height: usize) -> Grid<T> {
        let grid = vec![elem; width * height];
        Grid { grid, width }
    }

    pub fn height(&self) -> usize {
        self.grid.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Grid<usize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width {
                write!(f, "{:<4}", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.into_iter()
    }
}
