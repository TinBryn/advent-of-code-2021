use crate::{
    input::{Element, Snailfish},
    problem::Problem,
};

const EXAMPLE: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

#[test]
fn problem1() {
    let input = EXAMPLE.parse().unwrap();
    let problem = Problem::from_input(input);
    let actual = problem.part1();
    let expected = 4140;
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

#[test]
fn parse_simple_pair() {
    let actual: Snailfish = "[1,2]".parse().unwrap();
    let expected = Snailfish::new(Element::Number(1), Element::Number(2));
    assert_eq!(actual, expected)
}

#[test]
fn parse_nested_snailfish() {
    let actual: Snailfish = "[[1, 2], 3]".parse().unwrap();
    let expected = Snailfish::new(
        Element::Pair(Snailfish::new(Element::Number(1), Element::Number(2))),
        Element::Number(3),
    );
    println!("{}", actual);
    assert_eq!(actual, expected)
}

#[test]
fn print_complex_snailfish() {
    let input = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
    let sf: Snailfish = input.parse().unwrap();

    let s = sf.to_string();

    assert_eq!(s, input);
}

#[test]
fn magnitude_simple() {
    let s: Snailfish = "[9,1]".parse().unwrap();
    assert_eq!(s.magnitude(), 29);

    let s: Snailfish = "[[1,2],[[3,4],5]]".parse().unwrap();
    assert_eq!(s.magnitude(), 143);
}

macro_rules! snailfish {
    ($s: literal) => {
        <Snailfish as std::str::FromStr>::from_str($s).unwrap()
    };
}

#[test]
fn explode_left() {
    let mut s: Snailfish = snailfish!("[[[[[9,8],1],2],3],4]");
    assert!(s.explode());
    let expected = snailfish!("[[[[0,9],2],3],4]");
    assert_eq!(s, expected);
}
