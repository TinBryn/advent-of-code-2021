#[derive(Debug, Clone)]
pub struct Link {
    pub start: String,
    pub end: String,
}

impl Link {
    pub fn from_str(s: &str) -> Self {
        let parts: Vec<_> = s.trim().split('-').collect();
        let start = parts[0].into();
        let end = parts[1].into();
        Self { start, end }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub lines: Vec<Link>,
}

impl Input {
    pub fn from_str(s: &str) -> Self {
        Self {
            lines: s.trim().lines().map(Link::from_str).collect(),
        }
    }
}
