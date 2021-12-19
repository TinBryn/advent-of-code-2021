use crate::problem::Problem;

const DATA: [(&str, usize); 8] = [
    ("C200B40A82", 3),
    ("04005AC33890", 54),
    ("880086C3E88112", 7),
    ("CE00C43D881120", 9),
    ("D8005AC2A8F0", 1),
    ("F600BC2D8F", 0),
    ("9C005AC2F8F0", 0),
    ("9C0141080250320F1802104A08", 1),
];

#[test]
fn exp0() {
    test_part_2(0);
}

#[test]
fn exp1() {
    test_part_2(1);
}

#[test]
fn exp2() {
    test_part_2(2);
}
#[test]
fn exp3() {
    test_part_2(3);
}
#[test]
fn exp4() {
    test_part_2(4);
}
#[test]
fn exp5() {
    test_part_2(5);
}
#[test]
fn exp6() {
    test_part_2(6);
}
#[test]
fn exp7() {
    test_part_2(7);
}

fn test_part_2(example: usize) {
    let example = DATA[example];
    let (input, expected) = example;
    let input = input.parse().unwrap();
    let problem = Problem::from_input(input);
    let actual = problem.part2();
    assert_eq!(expected, actual);
}
