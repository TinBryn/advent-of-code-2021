use crate::{input::Input, problem::Problem};

const EXAMPLE: &str = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

#[test]
fn problem1() {
    let input = Input::from_str(EXAMPLE);
    let problem = Problem::from_input(input);
    let actual = problem.part1();
    let expected = 1656;
    assert_eq!(expected, actual);
}

#[test]
fn problem2() {
    let input = Input::from_str(EXAMPLE);
    let problem = Problem::from_input(input);
    let actual = problem.part2();
    let expected = 195;
    assert_eq!(expected, actual);
}

const SMALL: &str = "
11111
19991
19191
19991
11111";

#[test]
#[ignore = "debugging, no actual check"]
fn small_example() {
    let input = Input::from_str(SMALL);
    let mut problem = Problem::from_input(input);

    println!("{}", problem);

    for _ in 0..10 {
        problem.update();
        println!("{}", problem);
    }
}
