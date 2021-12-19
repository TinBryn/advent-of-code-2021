use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Debug)]
pub struct Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct Line {
    pub data: String,
}

impl FromStr for Line {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { data: s.trim().into() })
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub line: Line,
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(|line| Self {line})
    }
}

impl Input {
    
}
