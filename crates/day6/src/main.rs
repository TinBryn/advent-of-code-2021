use crate::input::Input;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::from_str(INPUT);

    println!("problem1: {}", input.problem1());
    println!("problem2: {}", input.problem2());
}

#[allow(unused)]
mod input {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct Input {
        ages: Vec<usize>,
    }

    impl Input {
        pub fn from_str(s: &str) -> Self {
            Self {
                ages: s
                    .trim()
                    .split(',')
                    .map(|n| n.trim().parse().unwrap())
                    .collect(),
            }
        }

        pub fn problem1(&self) -> usize {
            self.population_after_n_days(80)
        }

        pub fn problem2(&self) -> usize {
            self.population_after_n_days(256)
        }

        fn population_after_n_days(&self, iterations: i32) -> usize {
            let mut tally = self.tally();
            for _ in 0..iterations {
                update_population(&mut tally, 2);
            }
            tally.iter().copied().sum()
        }

        fn tally(&self) -> [usize; 9] {
            let mut tally = [0; 9];
            for &age in &self.ages {
                tally[age] += 1;
            }
            tally
        }
    }

    fn update_population(tally: &mut [usize], new_spawn_time: usize) {
        let zero = tally[0];
        for i in 0..tally.len() - 1 {
            tally[i] = tally[i + 1];
        }
        tally[tally.len() - 1 - new_spawn_time] += zero;
        tally[tally.len() - 1] = zero;
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
