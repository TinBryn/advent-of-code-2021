use crate::{input::Input, problem::{Problem, neighbours}};

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
    for i in 0..10 {
        let l = i * 10;
        let u = l + 10;
        println!("{:?}", &input.data[l..u])
    }
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

#[test]
fn adjacent() {
    for v in neighbours(0) {
        println!("x: {}, y: {}", v % 10, v / 10)
    }
}
