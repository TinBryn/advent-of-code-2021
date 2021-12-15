use std::collections::HashMap;

use crate::input::{Input, Insertion};

#[derive(Debug, Clone)]
pub struct Problem {
    pub template: String,
    pub subs: HashMap<[u8; 2], u8>,
}

impl Problem {
    pub fn from_input(input: Input) -> Self {
        let mut subs = HashMap::new();
        for Insertion {
            pattern: [a, c],
            insert: b,
        } in input.lines
        {
            subs.insert([a, c], b);
        }
        Self {
            template: input.template,
            subs,
        }
    }

    pub fn part1(&self) -> usize {
        self.common(10)
    }

    pub fn part2(&self) -> usize {
        self.common(40)
    }

    pub fn common(&self, iter: usize) -> usize {
        let mut pairs = HashMap::<[u8; 2], usize>::new();
        for p in self.template.as_bytes().windows(2).map(|w| [w[0], w[1]]) {
            if let Some(t) = pairs.get_mut(&p) {
                *t += 1;
            } else {
                pairs.insert(p, 1);
            }
        }

        for _ in 0..iter {
            pairs = self.replace(pairs);
        }

        let &last = self.template.as_bytes().last().unwrap();
        min_max_tally(tallys(&pairs, last))
    }

    fn replace(&self, pairs: HashMap<[u8; 2], usize>) -> HashMap<[u8; 2], usize> {
        let mut result = HashMap::new();

        for ([a, c], t) in pairs {
            if let Some(&b) = self.subs.get(&[a, c]) {
                *result.entry([a, b]).or_default() += t;
                *result.entry([b, c]).or_default() += t;
            }
        }

        result
    }
}

fn tallys(pairs: &HashMap<[u8; 2], usize>, last: u8) -> HashMap<u8, usize> {
    let mut result = HashMap::new();
    result.insert(last, 1);
    for ([f, _], &c) in pairs {
        *result.entry(*f).or_default() += c;
    }

    result
}

fn min_max_tally(tally: HashMap<u8, usize>) -> usize {
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for &v in tally.values() {
        min = min.min(v);
        max = max.max(v);
    }
    max - min
}
