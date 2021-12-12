use std::collections::{HashMap, HashSet};

use crate::input::{Input, Link};

#[derive(Debug, Clone, Default)]
pub struct Problem {
    links: HashMap<String, HashSet<String>>,
}

impl Problem {
    fn add_link_dir(&mut self, start: String, end: String) {
        let entry = self.links.entry(start);
        let set = entry.or_default();
        set.insert(end);
    }

    fn add_link(&mut self, a: String, b: String) {
        self.add_link_dir(a.clone(), b.clone());
        self.add_link_dir(b, a)
    }
    pub fn from_input(input: Input) -> Self {
        let mut problem = Self::default();
        for Link { start, end } in input.lines {
            problem.add_link(start, end);
        }
        problem
    }
}

fn is_small(s: &str) -> bool {
    s.chars().all(char::is_lowercase)
}

impl Problem {
    pub fn part1(&self) -> usize {
        self.common(no_dupes)
    }

    pub fn part2(&self) -> usize {
        self.common(single_dup_allowed)
    }

    fn common<P: Fn(&str, &[&str]) -> bool + Copy>(&self, condition: P) -> usize {
        self.dfs("start", vec![], condition)
    }
    pub fn dfs<'a, P: Fn(&str, &[&str]) -> bool + Copy>(
        &self,
        name: &'a str,
        mut path: Vec<&'a str>,
        small_cave_condition: P,
    ) -> usize {
        if let Some(links) = self.links.get(name) {
            if name == "end" {
                return 1;
            }
            if !path.is_empty() && is_small(name) {
                if name == "start" {
                    return 0;
                }
                if !small_cave_condition(name, &path) {
                    return 0;
                }
            }

            path.push(name);

            links
                .iter()
                .map(|name| self.dfs(name, path.clone(), small_cave_condition))
                .sum()
        } else {
            panic!()
        }
    }
}

fn no_dupes(name: &str, path: &[&str]) -> bool {
    !path.contains(&name)
}

fn single_dup_allowed(name: &str, path: &[&str]) -> bool {
    if !path.contains(&name) {
        true
    } else {
        for (i, c) in path.iter().enumerate().filter(|(_, &s)| is_small(s)) {
            if path[(i + 1)..].contains(c) {
                return false;
            }
        }
        true
    }
}
