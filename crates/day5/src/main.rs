use crate::input::Input;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::from_str(INPUT);
    // println!("input: {:?}", input);

    println!("problem1: {}", input.problem1());
    println!("problem2: {}", input.problem2());
}

#[allow(unused)]
mod input {
    use std::fmt::{Debug, Display};

    #[derive(Debug, Clone, Copy)]
    pub struct Line {
        pub x1: usize,
        pub y1: usize,
        pub x2: usize,
        pub y2: usize,
    }

    impl Display for Line {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{},{} -> {},{}", self.x1, self.y1, self.x2, self.y2)
        }
    }

    impl Line {
        pub fn from_str(s: &str) -> Self {
            let points: Vec<usize> = s
                .split("->")
                .flat_map(|s| s.trim().split(','))
                .map(|i| i.parse().unwrap())
                .collect();
            if let [x1, y1, x2, y2] = *points.as_slice() {
                Self { x1, y1, x2, y2 }
            } else {
                panic!()
            }
        }

        pub fn is_aligned(&self) -> bool {
            self.x1 == self.x2 || self.y1 == self.y2
        }
    }

    #[derive(Debug, Clone)]
    pub struct Input {
        pub lines: Vec<Line>,
    }

    #[derive(Clone)]
    pub struct Grid {
        points: Vec<usize>,
        width: usize,
    }

    impl Debug for Grid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let height = self.points.len() / self.width;
            for y in 0..height {
                for x in 0..self.width {
                    let point = self.points[y * self.width + x];
                    if point == 0 {
                        write!(f, ".")?;
                    } else {
                        write!(f, "{:?}", point)?;
                    }
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    impl Input {
        pub fn from_str(s: &str) -> Self {
            Self {
                lines: s.trim().lines().map(Line::from_str).collect(),
            }
        }

        fn get_max_values(&self) -> (usize, usize) {
            let mut maxx = 0;
            let mut maxy = 0;
            for line in &self.lines {
                if line.x1 > maxx {
                    maxx = line.x1
                }
                if line.x2 > maxx {
                    maxx = line.x2
                }
                if line.y1 > maxy {
                    maxy = line.y1
                }
                if line.y2 > maxy {
                    maxy = line.y2
                }
            }
            (maxx, maxy)
        }

        pub fn problem1(&self) -> usize {
            let (maxx, maxy) = self.get_max_values();
            let mut grid = vec![0usize; (maxx + 1) * (maxy + 1)];
            for line in self.lines.iter().filter(|line| line.is_aligned()) {
                // println!("{}", line);
                if line.x1 == line.x2 {
                    let x = line.x1;
                    let lower = line.y1.min(line.y2);
                    let upper = line.y1.max(line.y2);
                    for y in lower..=upper {
                        grid[y * (maxx + 1) + x] += 1;
                    }
                } else {
                    let y = line.y1;
                    let lower = line.x1.min(line.x2);
                    let upper = line.x1.max(line.x2);
                    for x in lower..=upper {
                        grid[y * (maxx + 1) + x] += 1;
                    }
                }
            }
            let grid = Grid {
                points: grid,
                width: maxx + 1,
            };
            // println!("{:?}", grid);
            grid.points.iter().filter(|i| **i > 1).count()
        }

        pub fn problem2(&self) -> usize {
            let (maxx, maxy) = self.get_max_values();
            let mut grid = vec![0usize; (maxx + 1) * (maxy + 1)];
            for line in self.lines.iter() {
                // println!("{}", line);
                if line.x1 == line.x2 {
                    horizontal(line, &mut grid, maxx);
                } else if line.y1 == line.y2 {
                    vertical(line, &mut grid, maxx);
                } else if line.x1 < line.x2 && line.y1 < line.y2 {
                    // println!("(1, 1) -> (2, 2) {}", line);
                    for (x, y) in (line.x1..=line.x2).zip(line.y1..=line.y2) {
                        grid[y * (maxx + 1) + x] += 1;
                    }
                } else if line.x2 < line.x1 && line.y1 < line.y2 {
                    // println!("(2, 1) -> (1, 2) {}", line);
                    for (x, y) in (line.x2..=line.x1).zip((line.y1..=line.y2).rev()) {
                        grid[y * (maxx + 1) + x] += 1;
                    }
                }else if line.x1 < line.x2 && line.y2 < line.y1 {
                    // println!("(1, 2) -> (2, 1) {}", line);
                    for (x, y) in (line.x1..=line.x2).zip((line.y2..=line.y1).rev()) {
                        grid[y * (maxx + 1) + x] += 1;
                    }
                } else if line.x2 < line.x1 && line.y2 < line.y1 {
                    // println!("(2, 2) -> (1, 1) {}", line);
                    for (x, y) in (line.x2..=line.x1).zip(line.y2..=line.y1) {
                        grid[y * (maxx + 1) + x] += 1;
                    }
                }
            }
            let grid = Grid {
                points: grid,
                width: maxx + 1,
            };
            // println!("{:?}", grid);
            grid.points.iter().filter(|i| **i > 1).count()
        }
    }

    fn vertical(line: &Line, grid: &mut Vec<usize>, maxx: usize) {
        let y = line.y1;
        let lower = line.x1.min(line.x2);
        let upper = line.x1.max(line.x2);
        for x in lower..=upper {
            grid[y * (maxx + 1) + x] += 1;
        }
    }

    fn horizontal(line: &Line, grid: &mut Vec<usize>, maxx: usize) {
        let x = line.x1;
        let lower = line.y1.min(line.y2);
        let upper = line.y1.max(line.y2);
        for y in lower..=upper {
            grid[y * (maxx + 1) + x] += 1;
        }
    }
}
#[cfg(test)]
mod test {
    use crate::Input;

    const INPUT: &str = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    const DIAGONALS: &str = "
    8,0 -> 0,8
    6,4 -> 2,0
    0,0 -> 8,8
    5,5 -> 8,2";

    #[test]
    fn diagonals_only() {
        let input = Input::from_str(DIAGONALS);
        let _ = 12;
        let _ = input.problem2();
        // assert_eq!(expected, actual);
    }

    #[test]
    fn problem1() {
        let input = Input::from_str(INPUT);
        let expected = 5;
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let input = Input::from_str(INPUT);
        let expected = 12;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
