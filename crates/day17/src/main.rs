use crate::{input::Input, problem::Problem};

const _INPUT: &str = "target area: x=175..227, y=-134..-79";

fn main() {
    let input = Input {
        x_range: 175..=227,
        y_range: -134..=-79,
    };
    let problem = Problem::from_input(input);

    println!("problem1: {}", problem.part1());
    println!("problem2: {}", problem.part2());
}

mod input;

mod problem;

#[cfg(test)]
mod test;
