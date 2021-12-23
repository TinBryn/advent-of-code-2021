use crate::{problem::Problem, input::Input};

const _EXAMPLE: &str = "target area: x=20..30, y=-10..-5";
const INPUT: Input = Input {
    x_range: 20..=30,
    y_range: -10..=-5,
};

#[test]
fn problem1() {
    let input = INPUT;
    let problem = Problem::from_input(input);
    let actual = problem.part1();
    let expected = 45;
    assert_eq!(expected, actual);
}

#[test]
fn problem2() {
    let input = INPUT;
    let problem = Problem::from_input(input);
    let actual = problem.part2();
    let expected = 112;
    assert_eq!(expected, actual);
}

#[test]
fn missed_point() {
    let input = INPUT;
    let problem = Problem::from_input(input);
    problem.hits(23, -10);
}
