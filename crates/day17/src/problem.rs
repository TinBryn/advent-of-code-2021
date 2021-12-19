use crate::input::Input;

#[derive(Debug, Clone)]
pub struct Problem {
    input: Input,
}

impl Problem {
    pub fn from_input(input: Input) -> Self {
        Self { input }
    }

    pub fn part1(&self) -> usize {
        self.common()
    }

    pub fn part2(&self) -> usize {
        self.common()
    }

    pub fn common(&self) -> usize {
        let minx = self.fun_name();
        let maxy = (-self.input.y_range.start()) as usize - 1;

        println!("minx: {}, maxy: {}", minx, maxy);
        maxy * (maxy + 1) / 2
        
    }

    fn fun_name(&self) -> isize {
        let mut minx = 0;
        while minx * (minx + 1) / 2 < *self.input.x_range.start() {
            minx += 1;
        }
        minx
    }
}
