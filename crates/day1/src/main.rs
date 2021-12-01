const INPUT: &str = include_str!("input.txt");

fn main() {
    // println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    numbers(input)
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    numbers(input)
        .windows(3)
        .map(|window| -> i32 { window.iter().sum() })
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

fn numbers(input: &str) -> Vec<i32> {
    let numbers = input
        .trim()
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    numbers
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};

    #[test]
    fn problem1() {
        let input =  r#"
199
200
208
210
200
207
240
269
260
263
"#;
        let expected = 7;
        let actual = solve_problem1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let input = r#"
199
200
208
210
200
207
240
269
260
263
"#;
        let expected = 5;
        let actual = solve_problem2(input);
        assert_eq!(expected, actual);
    }
}
