
#[derive(Debug, Clone)]
pub enum Difficulty {
    Hard,
    Medium,
    Easy,
}

impl From<&str> for Difficulty {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "medium" => Difficulty::Medium,
            "hard" => Difficulty::Hard,
            _ => Difficulty::Easy,
        }
    }
}

impl Into<&str> for Difficulty {
    fn into(self) -> &'static str {
        match self {
            Difficulty::Hard => "hard",
            Difficulty::Medium => "medium",
            Difficulty::Easy => "easy",
        }
    }
}
