use std::fmt;

#[derive(Debug, Clone)]
pub enum Difficulty {
    Hard,
    Medium,
    Easy,
}

impl From<&str> for Difficulty {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "hard" => Self::Hard,
            "medium" => Self::Medium,
            _ => Self::Easy,
        }
    }
}

impl Into<String> for Difficulty {
    fn into(self) -> String {
        match self {
            Self::Hard => "hard".to_string(),
            Self::Medium => "medium".to_string(),
            Self::Easy => "easy".to_string(),
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the Into trait to convert the enum to a string
        write!(f, "{}", self)
    }
}
