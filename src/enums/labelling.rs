#[derive(Debug)]
pub enum Labelling {
    Numeric,
    Alphabetic,
    Alphanumeric
}

impl From<&str> for Labelling {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            _ => Labelling::Numeric,
            "Alphabetic" => Labelling::Alphabetic,
            "Alphanumeric" => Labelling::Alphanumeric,
        }
    }
}

impl Into<&str> for Labelling {
    fn into(self) -> &'static str {
        match self {
            Self::Alphanumeric => "Alphanumeric",
            Self::Alphabetic => "Alphabetic",
            Self::Numeric => "Numeric",
        }
    }
}
