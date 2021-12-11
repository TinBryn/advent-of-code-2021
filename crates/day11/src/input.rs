#[derive(Debug, Clone)]
pub struct Input {
    pub data: Vec<u8>,
    pub width: usize,
}

impl Input {
    pub fn from_str(s: &str) -> Self {
        let data: Vec<u8> = s
            .chars()
            .filter(|c| ('0'..='9').contains(c))
            .map(|c| (c as u8 - b'0'))
            .collect();
        let width = s.trim().lines().next().unwrap().trim().len();
        Self { data, width }
    }
}
