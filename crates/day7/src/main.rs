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
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Input {
        positions: Vec<usize>,
    }

    impl Input {
        pub fn from_str(s: &str) -> Self {
            Self {
                positions: s.split(',').map(|n| n.trim().parse().unwrap()).collect(),
            }
        }

        pub fn problem1(&self) -> usize {
            self.common(Self::fuel_cost_1)
        }

        fn fuel_cost_1(k: usize, p: usize) -> usize {
            (k as isize - p as isize).abs() as usize
        }

        pub fn problem2(&self) -> usize {
            self.common(Self::fuel_cost_2)
        }

        fn fuel_cost_2(k: usize, p: usize) -> usize {
            let delta = (k as isize - p as isize).abs() as usize;

            delta * (delta + 1) / 2
        }

        fn common<F: Fn(usize, usize) -> usize>(&self, fuel_cost_fn: F) -> usize {
            let tally = self.get_tally();
            let (min, max) = tally_min_max_key(&tally);

            let mut min_fuel = usize::MAX;
            for p in min..=max {
                let fuel = total_fuel_cost(&tally, &fuel_cost_fn, p);
                if fuel < min_fuel {
                    min_fuel = fuel;
                }
            }
            min_fuel
        }

        fn get_tally(&self) -> HashMap<usize, usize> {
            let mut tally: HashMap<usize, usize> = HashMap::new();
            for i in &self.positions {
                let count = tally.get(i).copied().unwrap_or_default() + 1;
                tally.insert(*i, count);
            }
            tally
        }
    }

    fn total_fuel_cost<F: Fn(usize, usize) -> usize>(
        tally: &HashMap<usize, usize>,
        fuel_cost_fn: &F,
        p: usize,
    ) -> usize {
        let mut fuel = 0;
        for (&k, &v) in tally {
            let fuel_cost = fuel_cost_fn(k, p) * v;
            fuel += fuel_cost;
        }
        fuel
    }

    fn tally_min_max_key(tally: &HashMap<usize, usize>) -> (usize, usize) {
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        for &k in tally.keys() {
            if k < min {
                min = k;
            }
            if k > max {
                max = k;
            }
        }
        (min, max)
    }
}
#[cfg(test)]
mod test {
    use crate::Input;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
    #[test]
    fn problem1() {
        let input = Input::from_str(INPUT);
        let expected = 37;
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let input = Input::from_str(INPUT);
        let expected = 168;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
