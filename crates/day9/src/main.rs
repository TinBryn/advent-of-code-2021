use crate::input::{HeightMap, Input};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::from_str(INPUT);

    let heightmap = HeightMap::from_input(input);

    // println!("input: {:?}", heightmap);

    println!("problem1: {}", heightmap.problem1());
    println!("problem2: {}", heightmap.problem2());
}

#[allow(unused)]
mod input {
    use std::{
        collections::HashSet,
        ops::{Index, IndexMut},
    };

    #[derive(Debug)]
    pub struct HeightMap {
        pub data: Vec<u8>,
        pub width: usize,
    }

    impl HeightMap {
        pub fn from_input(input: Input) -> Self {
            let width = input.lines[0].heights.len();
            let data = input.lines.into_iter().flat_map(|l| l.heights).collect();
            Self { width, data }
        }

        fn get_xy(width: usize, index: usize) -> (usize, usize) {
            (index % width, index / width)
        }

        fn all_locations(&self) -> impl Iterator<Item = (usize, usize)> {
            let len = self.data.len();
            let width = self.width;
            (0..len).map(move |index| Self::get_xy(width, index))
        }

        pub fn problem1(&self) -> usize {
            self.common(|(x, y)| 1 + self[(x, y)] as usize)
        }

        pub fn problem2(&self) -> usize {
            let basins = self.get_all_basins();

            basins.into_iter().take(3).product()
        }

        pub fn get_all_basins(&self) -> Vec<usize> {
            let mut basins: Vec<_> = (0..self.data.len())
                .map(move |index| Self::get_xy(self.width, index))
                .filter(|&(x, y)| is_low_point((x, y), self))
                .map(|(x, y)| self.fill_basin((x, y)))
                .collect();
            basins.sort_by(|a, b| b.cmp(a));
            basins
        }

        fn fill_basin(&self, (x, y): (usize, usize)) -> usize {
            let mut basin = HashSet::new();
            self.fill_basin_rec(&mut basin, (x, y));

            basin.len()
        }

        fn fill_basin_rec(&self, basin: &mut HashSet<(usize, usize)>, (x, y): (usize, usize)) {
            basin.insert((x, y));

            for (x1, y1) in neighbours((x, y), self) {
                if self[(x, y)] < self[(x1, y1)] && self[(x1, y1)] < 9 && !basin.contains(&(x1, y1))
                {
                    self.fill_basin_rec(basin, (x1, y1));
                }
            }
        }

        fn common<F: Fn((usize, usize)) -> usize>(&self, f: F) -> usize {
            (0..self.data.len())
                .map(move |index| Self::get_xy(self.width, index))
                .filter(|&(x, y)| is_low_point((x, y), self))
                .map(f)
                .sum()
        }

        pub fn height(&self) -> usize {
            self.data.len() / self.width
        }
    }

    pub fn neighbours(
        (x, y): (usize, usize),
        map: &HeightMap,
    ) -> impl Iterator<Item = (usize, usize)> {
        (x > 0)
            .then(|| (x - 1, y))
            .into_iter()
            .chain((x < map.width - 1).then(|| (x + 1, y)))
            .chain((y > 0).then(|| (x, y - 1)))
            .chain((y < map.height() - 1).then(|| (x, y + 1)))
    }

    pub fn is_low_point((x, y): (usize, usize), map: &HeightMap) -> bool {
        neighbours((x, y), map)
            .into_iter()
            .all(|(x1, y1)| map[(x, y)] < map[(x1, y1)])
    }

    impl Index<(usize, usize)> for HeightMap {
        type Output = u8;

        fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
            let index = y * self.width + x;
            &self.data[index]
        }
    }

    impl IndexMut<(usize, usize)> for HeightMap {
        fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
            let index = y * self.width + x;
            &mut self.data[index]
        }
    }

    #[derive(Debug)]
    pub struct Line {
        pub heights: Vec<u8>,
    }

    impl Line {
        pub fn from_str(s: &str) -> Self {
            let heights = s.trim().chars().map(|c| c as u8 - b'0').collect();
            Self { heights }
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

        pub fn problem1(&self) -> usize {
            todo!()
        }

        pub fn problem2(&self) -> usize {
            todo!()
        }
    }
}
#[cfg(test)]
mod test {
    use crate::{input::HeightMap, Input};

    const INPUT: &str = "
    2199943210
    3987894921
    9856789892
    8767896789
    9899965678";

    #[test]
    fn problem1() {
        let input = Input::from_str(INPUT);
        for line in &input.lines {
            println!("{:?}", line.heights);
        }
        let heightmap = HeightMap::from_input(input);
        let expected = 15;
        let actual = heightmap.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let input = Input::from_str(INPUT);
        let heightmap = HeightMap::from_input(input);
        let expected = 1134;
        let actual = {
            let this = heightmap;
            let basins = this.get_all_basins();

            println!("{:?}", basins);

            basins.into_iter().take(3).product::<usize>()
        };
        assert_eq!(expected, actual);
    }
}
