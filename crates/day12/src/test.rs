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
    check1(EXAMPLE1, 10);
}

#[test]
fn problem1_exp2() {
    check1(EXAMPLE2, 19);
}

#[test]
fn problem1_exp3() {
    check1(EXAMPLE3, 226);
}

#[test]
fn problem2() {
    check2(EXAMPLE1, 36);
}

fn check1(input: &str, expected: usize) {
    let problem = Problem::from_input(Input::from_str(input));
    let actual = problem.part1();
    assert_eq!(expected, actual);
}

fn check2(input: &str, expected: usize) {
    let problem = Problem::from_input(Input::from_str(input));
    let actual = problem.part2();
    assert_eq!(expected, actual);
}
