use std::{fmt::Debug, ops::RangeInclusive};

#[derive(Debug)]
pub struct Error {}

#[derive(Debug, Clone)]
pub struct Input {
    pub x_range: RangeInclusive<isize>,
    pub y_range: RangeInclusive<isize>,
}

impl Input {}
