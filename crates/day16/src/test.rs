use crate::problem::Problem;

const EXAMPLE: &str = "";

#[test]
fn problem1() {
    let input = EXAMPLE.parse().unwrap();
    let problem = Problem::from_input(input);
    let actual = problem.part1();
    let expected = 0;
    assert_eq!(expected, actual);
}

#[test]
fn problem2() {
    let input = EXAMPLE.parse().unwrap();
    let problem = Problem::from_input(input);
    let actual = problem.part2();
    let expected = 0;
    assert_eq!(expected, actual);
}
