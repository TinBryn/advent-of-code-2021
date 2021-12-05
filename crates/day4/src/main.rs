use crate::input::Input;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::from_string(INPUT);
    print_input(&input);

    println!("problem1: {}", input.problem1());
    println!("problem2: {}", input.problem2());
}

fn print_input(input: &Input) {
    println!("{:?}", input.inputs);
    for board in &input.boards {
        for i in 0..5 {
            let lower = i * 5;
            let upper = i * 5 + 5;
            println!("{:?}", &board.numbers[lower..upper])
        }
        println!("{:?}", board);
    }
}

mod input {
    #[derive(Debug, Clone)]
    pub struct Board {
        pub numbers: Vec<usize>,
    }

    impl Board {
        fn read(input: &[&str]) -> Vec<usize> {
            input
                .iter()
                .flat_map(|&line| line.split_ascii_whitespace().map(|i| i.parse().unwrap()))
                .collect()
        }
    }

    #[derive(Debug)]
    pub struct Input {
        pub inputs: Vec<usize>,
        pub boards: Vec<Board>,
    }

    impl Input {
        pub fn from_string(s: &str) -> Self {
            let lines = s.trim().lines().collect::<Vec<_>>();

            let (first, rest) = lines.split_first().unwrap();

            let inputs = first
                .split(',')
                .map(|i| i.parse())
                .collect::<Result<_, _>>()
                .unwrap();

            let boards = rest
                .chunks(6)
                .skip(1)
                .map(|board| Board {
                    numbers: Board::read(board),
                })
                .collect();

            Self { inputs, boards }
        }

        pub fn problem1(&self) -> usize {
            let mut state = BoardState::clone_with_state(&self.boards);

            for number in &self.inputs {
                for board in state.iter_mut() {
                    if let Some(res) = board.call(*number) {
                        return res;
                    }
                }
            }
            0
        }

        pub fn problem2(&self) -> usize {
            let mut state = BoardState::clone_with_state(&self.boards);

            let mut won = vec![false; state.len()];
            let mut order = 0;

            for &number in &self.inputs {
                for (i, board) in state.iter_mut().enumerate() {
                    if !won[i] {
                        if let Some(res) = board.call(number) {
                            won[i] = true;
                            order += 1;
                            if order == self.boards.len() {
                                return res;
                            }
                        }
                    }
                }
            }

            0
        }
    }

    #[derive(Debug)]
    struct BoardState {
        board: Board,
        called: Vec<bool>,
    }

    fn find(slice: &[usize], number: usize) -> Option<usize> {
        for (i, &item) in slice.iter().enumerate() {
            if item == number {
                return Some(i);
            }
        }
        None
    }

    impl BoardState {
        fn new(board: Board) -> Self {
            let called = vec![false; board.numbers.len()];
            Self { called, board }
        }

        fn clone_with_state(boards: &[Board]) -> Vec<Self> {
            boards
                .iter()
                .cloned()
                .map(BoardState::new)
                .collect()
        }

        fn call(&mut self, number: usize) -> Option<usize> {
            let location = find(&self.board.numbers, number)?;
            self.called[location] = true;

            self.check_rows_cols().map(|r| r * number)
        }

        fn check_row(&self, row: usize) -> Option<usize> {
            for i in 0..5 {
                let index = row * 5 + i;
                if !self.called[index] {
                    return None;
                }
            }
            let count = self
                .board
                .numbers
                .iter()
                .zip(&self.called)
                .filter_map(|(n, b)| (!b).then(|| *n))
                .sum();
            Some(count)
        }

        fn check_col(&self, col: usize) -> Option<usize> {
            for i in 0..5 {
                let index = i * 5 + col;
                if !self.called[index] {
                    return None;
                }
            }
            let count = self
                .board
                .numbers
                .iter()
                .zip(&self.called)
                .filter_map(|(n, b)| (!b).then(|| *n))
                .sum();
            Some(count)
        }

        fn check_rows_cols(&self) -> Option<usize> {
            for res in (0..5)
                .map(|row| self.check_row(row))
                .chain((0..5).map(|col| self.check_col(col)))
            {
                if res.is_some() {
                    return res;
                }
            }
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Input;

    const INPUT: &str = "
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

    #[test]
    fn problem1() {
        let input = Input::from_string(INPUT);
        let expected = 4512;
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let input = Input::from_string(INPUT);
        let expected = 1924;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
