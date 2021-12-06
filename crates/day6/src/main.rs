use crate::input::Input;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::from_str(INPUT);

    println!("problem1: {}", input.problem1());
    println!("problem2: {}", input.problem2());
}

mod input {

    #[derive(Debug, Clone)]
    pub struct Input {
        phases: Vec<usize>,
    }

    impl Input {
        pub fn from_str(s: &str) -> Self {
            Self {
                phases: s
                    .trim()
                    .split(',')
                    .map(|n| n.trim().parse().unwrap())
                    .collect(),
            }
        }

    pub fn problem1(&self) -> usize {
        let mut population = self.phases.clone();
        for _ in 0..80 {
            let births = population
                .iter()
                .filter_map(|a| (*a == 0).then(|| 8))
                .collect::<Vec<_>>();
            for phase in population.iter_mut() {
                if *phase == 0 {
                    *phase = 6
                } else {
                    *phase -= 1;
                }
            }
            population.extend(births);
        }
        population.len()
    }

    pub fn problem2(&self) -> usize {
        let mut phase_count = [0; 9];
        for &phase in &self.phases {
            phase_count[phase] += 1;
        }

        for _ in 0..256 {
            let zero = phase_count[0];
            for i in 0..8 {
                phase_count[i] = phase_count[i + 1];
            }
            phase_count[6] += zero;
            phase_count[8] = zero;
        }
        phase_count.iter().sum()
    }
    }
}
#[cfg(test)]
mod test {
    use crate::Input;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn problem1() {
        let input = Input::from_str(INPUT);
        let expected = 5934;
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let input = Input::from_str(INPUT);
        let expected = 26984457539;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
