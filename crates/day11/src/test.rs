use crate::{input::Input, problem::Problem};

const EXAMPLE: &str = "";

#[test]
fn problem1() {
    let input = Input::from_str(EXAMPLE);
    let problem = Problem::from_input(input);
    let actual = problem.part1();
    let expected = 0;
    assert_eq!(expected, actual);
}

#[test]
fn problem2() {
    let input = Input::from_str(EXAMPLE);
    let problem = Problem::from_input(input);
    let actual = problem.part2();
    let expected = 0;
    assert_eq!(expected, actual);
}
