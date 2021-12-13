use crate::problem::Problem;

const EXAMPLE: &str = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

#[test]
fn problem1() {
    let input = EXAMPLE.parse().unwrap();
    let problem = Problem::from_input(input);
    let actual = problem.part1();
    let expected = 17;
    assert_eq!(expected, actual);
}
