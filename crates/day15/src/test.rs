use crate::problem::Problem;

const EXAMPLE: &str = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

#[test]
fn problem1() {
    let input = EXAMPLE.parse().unwrap();
    let problem = Problem::from_input(input);
    println!("{}", problem.grid);
    let actual = problem.part1();
    let expected = 40;
    assert_eq!(expected, actual);
}

#[test]
fn problem2() {
    let input = EXAMPLE.parse().unwrap();
    let problem = Problem::from_input(input);
    let actual = problem.part2();
    let expected = 315;
    assert_eq!(expected, actual);
}
