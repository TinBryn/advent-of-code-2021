use std::collections::HashMap;

use crate::input::{Input, Insertion};

#[derive(Debug, Clone)]
pub struct Problem {
    pub template: String,
    pub lines: Vec<Insertion>,
    pub subs: HashMap<[u8; 2], u8>,
}

impl Problem {
    pub fn from_input(input: Input) -> Self {
        let lines = input.lines;
        let mut subs = HashMap::new();
        for &Insertion {
            pattern: [a, c],
            insert: b,
        } in &lines
        {
            subs.insert([a, c], b);
        }
        Self {
            template: input.template,
            lines,
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

        for i in 0..iter {
            println!("cycle {}", i + 1);
            pairs = self.replace(pairs);
        }

        let &last = self.template.as_bytes().last().unwrap();
        let tally = tallys(&pairs, last);

        min_max_tally(tally)
    }

    fn replace(&self, pairs: HashMap<[u8; 2], usize>) -> HashMap<[u8; 2], usize> {
        let mut result = HashMap::new();

        for ([a, c], t) in pairs {
            if let Some(&b) = self.subs.get(&[a, c]) {
                add_or_insert(&mut result, [a, b], t);
                add_or_insert(&mut result, [b, c], t);
            }
        }

        result
    }
}

fn add_or_insert(result: &mut HashMap<[u8; 2], usize>, a: [u8; 2], c: usize) {
    if let Some(t) = result.get_mut(&a) {
        *t += c;
    } else {
        result.insert(a, c);
    }
}

fn tallys(pairs: &HashMap<[u8; 2], usize>, last: u8) -> HashMap<u8, usize> {
    let mut result = HashMap::new();
    result.insert(last, 1);
    for ([f, _], &c) in pairs {
        if let Some(t) = result.get_mut(f) {
            *t += c;
        } else {
            result.insert(*f, c);
        }
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
