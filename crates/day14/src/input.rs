use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct Insertion {
    pub pattern: [u8; 2],
    pub insert: u8,
}

impl Display for Insertion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} -> {}",
            self.pattern[0] as char, self.pattern[1] as char, self.insert as char
        )
    }
}

impl FromStr for Insertion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");

        let pattern = parts.next().ok_or(Error::Bad)?.as_bytes();
        let pattern = [
            *pattern.get(0).ok_or(Error::Bad)?,
            *pattern.get(1).ok_or(Error::Bad)?,
        ];

        let insert = *parts
            .next()
            .ok_or(Error::Bad)?
            .as_bytes()
            .get(0)
            .ok_or(Error::Bad)?;

        Ok(Self { pattern, insert })
    }
}

#[derive(Debug)]
pub enum Error {
    Bad,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct Input {
    pub template: String,
    pub lines: Vec<Insertion>,
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.template)?;
        for line in &self.lines {
            write!(f, "\n{}", line)?;
        }
        Ok(())
    }
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        if let Some(template) = lines.next() {
            let template = template.to_owned();
            let lines = lines
                .filter(|line| !line.is_empty())
                .map(|s| s.trim().parse())
                .collect::<Result<_, _>>()?;

            Ok(Self { template, lines })
        } else {
            Err(Error::Bad)
        }
    }
}

impl Input {}
