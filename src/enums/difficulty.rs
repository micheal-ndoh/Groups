use derive_more::Display;

#[derive(Debug, Clone, Display)]
pub enum Difficulty {
    Hard,
    Medium,
    Easy,
}

impl From<&str> for Difficulty {
    fn from(value: &str) -> Self {
        match  value.to_lowercase().as_str() {
            "hard" => Self::Hard,
            "medium" => Self::Medium,
            _ => Self::Easy,
        }
    }
}

impl Into<&str> for Difficulty {
    fn into(self) -> &'static str {
        match self {
            Self::Hard => "hard",
            Self::Medium => "medium",
            Self::Easy => "easy",
        }
    }
}
impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self())
    }
}

