use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Debug)]
pub enum Error {
    Open,
    Close,
    Comma,
    Number,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element {
    Number(usize),
    Pair(Snailfish),
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Pair(n) => write!(f, "{}", n),
        }
    }
}

impl Element {
    pub fn parser(s: &str) -> Result<(Self, &str), Error> {
        if s.starts_with('[') {
            let (n, rest) = Snailfish::parser(s.trim())?;
            Ok((Self::Pair(n), rest))
        } else {
            let (n, rest): (usize, &str) = parse_number(s.trim()).ok_or(Error::Number)?;
            Ok((Self::Number(n), rest))
        }
    }

    pub fn magnitude(&self) -> usize {
        match self {
            Element::Number(n) => *n,
            Element::Pair(n) => n.magnitude(),
        }
    }
}

fn parse_number(s: &str) -> Option<(usize, &str)> {
    let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    let n: usize = digits.parse().ok()?;
    Some((n, &s[digits.len()..]))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snailfish(pub Box<Element>, pub Box<Element>);

impl Display for Snailfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl Snailfish {
    pub fn new(first: Element, second: Element) -> Self {
        Self(Box::new(first), Box::new(second))
    }

    pub fn parser(s: &str) -> Result<(Self, &str), Error> {
        let tail = s.strip_prefix('[').ok_or(Error::Open)?;
        let (element1, rest) = Element::parser(tail)?;
        let tail = rest.trim().strip_prefix(',').ok_or(Error::Comma)?;
        let (element2, rest) = Element::parser(tail)?;
        rest.trim()
            .strip_prefix(']')
            .map(|r| (Self::new(element1, element2), r))
            .ok_or(Error::Close)
    }
}

impl Snailfish {
    pub fn reduce(mut self) -> Self {
        if self.explode() {
            return self.reduce();
        }
        if self.split() {
            return self.reduce();
        }
        self
    }
    pub fn explode(&mut self) -> bool {
        false
    }

    pub fn split(&mut self) -> bool {
        false
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(Element::Pair(self.clone()), Element::Pair(other.clone())).reduce()
    }

    pub fn magnitude(&self) -> usize {
        3 * self.0.magnitude() + 2 * self.1.magnitude()
    }
}

impl FromStr for Snailfish {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parser(s).map(|r| r.0)
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub numbers: Vec<Snailfish>,
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .trim()
            .lines()
            .map(|s| s.trim().parse())
            .collect::<Result<_, _>>()?;
        Ok(Self { numbers: lines })
    }
}

impl Input {}
