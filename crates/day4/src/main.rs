use crate::input::Input;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::from_str(INPUT);
    println!("input: {:?}", input);

    println!("problem1: {}", input.problem1());
    println!("problem2: {}", input.problem2());
}

#[allow(unused)]
mod input {
    #[derive(Debug)]
    pub struct Line {}

    impl Line {
        pub fn from_str(s: &str) -> Self {
            Self {}
        }
    }

    #[derive(Debug)]
    pub struct Input {
        inner: Vec<Line>,
    }

    impl Input {
        pub fn from_str(s: &str) -> Self {
            Self {
                inner: s.lines().map(Line::from_str).collect(),
            }
        }

        pub fn problem1(&self) -> i32 {
            todo!()
        }

        pub fn problem2(&self) -> i32 {
            todo!()
        }
    }
}
#[cfg(test)]
mod test {
    use crate::Input;

    #[test]
    fn problem1() {
        let input = "";
        let input = Input::from_str(input);
        let expected = 0;
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let input = "";
        let input = Input::from_str(input);
        let expected = 0;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
