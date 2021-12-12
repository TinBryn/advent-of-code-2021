#[derive(Debug, Clone)]
pub struct Line {
    pub data: String,
}

impl Line {
    pub fn from_str(s: &str) -> Self {
        Self { data: s.into() }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub lines: Vec<Line>,
}

impl Input {
    pub fn from_str(s: &str) -> Self {
        Self {
            lines: s.trim().lines().map(Line::from_str).collect(),
        }
    }
}
