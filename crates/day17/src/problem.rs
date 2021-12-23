use crate::input::Input;

#[derive(Debug, Clone)]
pub struct Problem {
    top: isize,
    left: isize,
    right: isize,
    bottom: isize,
}

impl Problem {
    pub fn from_input(input: Input) -> Self {
        Self {
            top: *input.y_range.end(),
            left: *input.x_range.start(),
            right: *input.x_range.end(),
            bottom: *input.y_range.start(),
        }
    }

    pub fn part1(&self) -> usize {
        self.common()
    }

    pub fn part2(&self) -> usize {
        let mut count = 0;
        for x_vel in 0..=self.right {
            if x_vel * (x_vel + 1) < self.left * 2 {
                continue;
            }
            for y_vel in self.bottom..=(-self.bottom) {
                if self.hits(x_vel, y_vel) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn hits(&self, mut x_vel: isize, mut y_vel: isize) -> bool {
        let (mut x, mut y) = (0, 0);
        while y >= self.bottom && x <= self.right {
            if x >= self.left
            && x <= self.right
            && y >= self.bottom
            && y <= self.top
            {
                return true;
            }
            x += x_vel;
            if x_vel > 0 {
                x_vel -= 1;
            }
            y += y_vel;
            y_vel -= 1;
        }
        false
    }

    pub fn common(&self) -> usize {
        let minx = self.minx_vel();
        let maxy = (-self.bottom) as usize - 1;

        println!("minx: {}, maxy: {}", minx, maxy);
        maxy * (maxy + 1) / 2
    }

    fn minx_vel(&self) -> isize {
        let mut minx = 0;
        while minx * (minx + 1) / 2 < self.left {
            minx += 1;
        }
        minx
    }
}
