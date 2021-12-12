use crate::{input::Input, problem::Problem};

const EXAMPLE1: &str = "
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

const EXAMPLE2: &str = "
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

const EXAMPLE3: &str = "
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

#[test]
fn problem1_exp1() {
    let input = Input::from_str(EXAMPLE1);
    let problem = Problem::from_input(input);
    let actual = problem.part1();
    let expected = 10;
    assert_eq!(expected, actual);
}

#[test]
fn problem1_exp2() {
    let input = Input::from_str(EXAMPLE2);
    let problem = Problem::from_input(input);
    let actual = problem.part1();
    let expected = 19;
    assert_eq!(expected, actual);
}

#[test]
fn problem1_exp3() {
    let input = Input::from_str(EXAMPLE3);
    let problem = Problem::from_input(input);
    let actual = problem.part1();
    let expected = 226;
    assert_eq!(expected, actual);
}

#[test]
fn problem2() {
    let input = Input::from_str(EXAMPLE1);
    let problem = Problem::from_input(input);
    let actual = problem.part2();
    let expected = 36;
    assert_eq!(expected, actual);
}
