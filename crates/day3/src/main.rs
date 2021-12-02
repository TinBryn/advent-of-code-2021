const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("input: {:?}", INPUT);
    // println!("problem1: {}", solve_problem1(INPUT));
    // println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    todo!()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    todo!()
}


#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};

    #[test]
    #[ignore = "day not available"]
    fn problem1() {
        let input = "";
        let expected = 0;
        let actual = solve_problem1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore = "day not available"]
    fn problem2() {
        let input = "";
        let expected = 0;
        let actual = solve_problem2(input);
        assert_eq!(expected, actual);
    }
}
