use crate::problem::Problem;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = INPUT.parse().unwrap();
    let problem = Problem::from_input(input);

    // println!("{:?}", problem);

    println!("problem1: {}", problem.part1());
    println!("problem2: {}", problem.part2());
}

mod input;

mod problem;

#[cfg(test)]
mod test;

// LRFJBJEH
