use crate::problem::Problem;

#[test]
fn exp1() {
    let example = "8A004A801A8002F478";
    let expected = 16;
    test_example(example, expected);
}

#[test]
fn exp2() {
    let example = "620080001611562C8802118E34";
    let expected = 12;
    test_example(example, expected);
}

#[test]
fn exp3() {
    let example = "C0015000016115A2E0802F182340";
    let expected = 23;
    test_example(example, expected);
}

#[test]
fn exp4() {
    let example = "A0016C880162017C3686B18A3D4780";
    let expected = 31;
    test_example(example, expected);
}

fn test_example(example: &str, expected: usize) {
    let input = example.parse().unwrap();
    let problem = Problem::from_input(input);
    let actual = problem.part1();
    assert_eq!(expected, actual);
}
