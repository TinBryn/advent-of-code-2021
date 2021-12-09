use crate::input::Input;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::from_str(INPUT);
    // println!("input: {:?}", input);

    println!("problem1: {}", input.problem1());
    println!("problem2: {}", input.problem2());
}

mod input {
    use std::fmt::Display;

    #[derive(Debug)]
    pub struct Line {
        inputs: Vec<Pattern>,
        outputs: Vec<Pattern>,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Pattern {
        pub bits: u8,
    }

    impl Pattern {
        pub fn new(i: &str) -> Self {
            let mut bits = 0;
            for b in i.chars() {
                bits |= 1 << (b as u8 - b'a');
            }
            Self { bits }
        }

        pub fn contains(&self, index: usize) -> bool {
            (self.bits & (1 << index)) > 0
        }

        pub fn len(&self) -> usize {
            (0..7).filter(|&i| self.contains(i)).count()
        }

        pub fn overlap(self, other: Self) -> usize {
            (0..7)
                .filter(|&i| {
                    let bits = self.bits & other.bits;
                    Self { bits }.contains(i)
                })
                .count()
        }
    }

    impl Display for Pattern {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for b in 0..7 {
                if self.contains(b) {
                    write!(f, "{}", (b as u8 + b'a') as char)?;
                }
            }
            Ok(())
        }
    }

    impl Line {
        pub fn from_str(s: &str) -> Self {
            let parts: Vec<_> = s.trim().split('|').map(|p| p.trim()).collect();
            if let [input, output] = parts[..] {
                let mut inputs: Vec<_> = input.split(' ').map(Pattern::new).collect();
                inputs.sort_by_key(|s| s.len());
                let outputs = output.split(' ').map(Pattern::new).collect();
                Self { inputs, outputs }
            } else {
                panic!()
            }
        }

        fn decode_number(pattern: Pattern, one: Pattern, four: Pattern) -> usize {
            match pattern.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                5 => {
                    if pattern.overlap(one) == 2 {
                        3
                    } else if pattern.overlap(four) == 2 {
                        2
                    } else {
                        5
                    }
                }
                6 => {
                    if pattern.overlap(one) == 1 {
                        6
                    } else if pattern.overlap(four) == 4 {
                        9
                    } else {
                        0
                    }
                }
                7 => 8,
                _ => unreachable!("bad pattern"),
            }
        }

        pub fn decode_all(&self) -> usize {
            self.outputs
                .iter()
                .copied()
                .enumerate()
                .map(|(i, s)| {
                    10usize.pow(3 - i as u32)
                        * Self::decode_number(s, self.inputs[0], self.inputs[2])
                })
                .sum()
        }
    }

    impl Display for Line {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for input in &self.inputs {
                write!(f, "{} ", input)?;
            }
            write!(f, "| ")?;
            for output in &self.outputs {
                write!(f, "{} ", output)?;
            }
            Ok(())
        }
    }

    #[derive(Debug)]
    pub struct Input {
        pub lines: Vec<Line>,
    }

    impl Display for Input {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for line in &self.lines {
                writeln!(f, "{}", line)?;
            }
            Ok(())
        }
    }

    fn unique(s: &Pattern) -> bool {
        s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7
    }

    impl Input {
        pub fn from_str(s: &str) -> Self {
            Self {
                lines: s.trim().lines().map(Line::from_str).collect(),
            }
        }

        pub fn problem1(&self) -> usize {
            self.lines
                .iter()
                .flat_map(|line| &line.outputs)
                .copied()
                .filter(unique)
                .count()
        }

        pub fn problem2(&self) -> usize {
            self.lines.iter().map(|line| line.decode_all()).sum()
        }
    }
}
#[cfg(test)]
mod test {
    use crate::{input::Pattern, Input};

    const INPUT: &str = "
    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    ";

    #[test]
    fn problem1() {
        let input = Input::from_str(INPUT);
        println!("{}", input);
        let expected = 26;
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let input = Input::from_str(INPUT);
        let expected = 61229;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }

    #[test]
    fn patterns() {
        let input = "bafeg";
        let pattern = Pattern::new(input);
        println!("{:b}", pattern.bits);
        let expected = "abefg";
        let actual = pattern.to_string();

        assert_eq!(expected, actual);
    }
}
