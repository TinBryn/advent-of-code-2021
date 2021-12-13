use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Debug)]
#[allow(unused)]
pub struct Error {
    data: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub enum Line {
    Point(Point),
    Fold(Fold),
}

impl FromStr for Line {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("fold") {
            let mut parts = s[11..].split('=');
            let dir = parts.next().ok_or(Error { data: s.into() })?;
            let pos = parts
                .next()
                .ok_or(Error { data: s.into() })?
                .parse()
                .map_err(|_| Error { data: s.into() })?;
            match dir {
                "x" => Ok(Line::Fold(Fold::X(pos))),
                "y" => Ok(Line::Fold(Fold::Y(pos))),
                _ => Err(Error { data: s.into() }),
            }
        } else {
            let mut parts = s.split(',');
            let x = parts
                .next()
                .ok_or(Error { data: s.into() })?
                .parse()
                .map_err(|_| Error { data: s.into() })?;
            let y = parts
                .next()
                .ok_or(Error { data: s.into() })?
                .parse()
                .map_err(|_| Error { data: s.into() })?;
            Ok(Line::Point(Point { x, y }))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug, Clone)]
pub struct Input {
    pub points: Vec<Point>,
    pub folds: Vec<Fold>,
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = vec![];
        let mut folds = vec![];
        for line in s.trim().lines().filter(|line| !line.is_empty()) {
            let line = line.parse()?;
            match line {
                Line::Point(point) => points.push(point),
                Line::Fold(fold) => folds.push(fold),
            }
        }
        Ok(Self { points, folds })
    }
}

impl Input {}
