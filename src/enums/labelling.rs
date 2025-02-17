#[derive(Debug)]
pub enum Labelling{
  Numeric,
  Alphabetic,
  Alphanumeric
}

impl From<&str> for Labelling {
  fn from(value: &str) -> Self {
      match  value.to_lowercase().as_str() {
          "alphanumeric" => Self::Alphanumeric, 
          "alphabetic" => Self::Alphabetic,
          _ => Self::Numeric,
      }
  }
}

impl Into<&str> for Labelling {
  fn into(self) -> &'static str {
      match self {
        Self::Alphabetic => "alphabetic",
        Self::Alphanumeric => "alphanumeric",
        Self::Numeric => "numeric",
      }
  }
}