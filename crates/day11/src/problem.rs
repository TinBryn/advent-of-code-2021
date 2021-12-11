use crate::input::Input;

#[derive(Debug, Clone)]
pub struct Problem {
    data: Vec<u8>,
}

impl Problem {
    pub fn from_input(input: Input) -> Self {
        Self { data: input.data }
    }

    pub fn part1(&self) -> usize {
        let mut current = self.data.clone();
        let mut flashes = 0;

        for _ in 0..100 {
            let u = update(&mut current);
            flashes += u;
        }

        flashes
    }

    pub fn part2(&self) -> usize {
        let mut current = self.data.clone();

        for i in 1.. {
            let u = update(&mut current);
            if u == 100 {
                return i;
            }
        }

        unreachable!()
    }
}

fn update(current: &mut [u8]) -> usize {
    let mut flashing = vec![];
    #[allow(clippy::needless_range_loop)]
    for i in 0..100 {
        let src = current[i];
        if flash(&mut current[i], src) {
            flashing.push(i)
        }
    }

    while let Some(flasher) = flashing.pop() {
        for viewer in neighbours(flasher) {
            if current[viewer] > 0 {
                let src = current[viewer];
                if flash(&mut current[viewer], src) {
                    flashing.push(viewer)
                }
            }
        }
    }

    current.iter().filter(|&&i| i == 0).count()
}

fn flash(dst: &mut u8, src: u8) -> bool {
    *dst = src + 1;
    if *dst > 9 {
        *dst = 0;
        true
    } else {
        false
    }
}

pub fn neighbours(flasher: usize) -> impl Iterator<Item = usize> {
    let (x, y) = {
        let x = flasher % 10;
        let y = flasher / 10;
        (x, y)
    };

    let (minx, maxx) = edge_cases(x);
    let (miny, maxy) = edge_cases(y);

    (minx..=maxx)
        .flat_map(move |nx| (miny..=maxy).map(move |ny| (nx, ny)))
        .filter(move |&(nx, ny)| nx != x || ny != y)
        .map(|(x, y)| x + y * 10)
}

fn edge_cases(x: usize) -> (usize, usize) {
    let min = if x > 0 { x - 1 } else { x };
    let max = if x < 9 { x + 1 } else { x };
    (min, max)
}
