const INPUT: &str = include_str!("input.txt");

fn main() {
    let numbers = numbers(INPUT);
    println!("problem1: {}", solve_problem1(&numbers));
    println!("problem2: {}", solve_problem2(&numbers));
}

fn solve_problem1(numbers: &[i32]) -> usize {
    count_increases(numbers, 1)
}

fn solve_problem2(numbers: &[i32]) -> usize {
    count_increases(numbers, 3)
}

fn count_increases(numbers: &[i32], window: usize) -> usize {
    numbers
        .iter()
        .zip(numbers.iter().skip(window))
        .filter(|(l, u)| l < u)
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

    const NUMBERS: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn problem1() {
        let expected = 7;
        let actual = solve_problem1(&NUMBERS);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let expected = 5;
        let actual = solve_problem2(&NUMBERS);
        assert_eq!(expected, actual);
    }
}
