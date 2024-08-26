use std::fmt::Display;

#[derive(Clone)]
pub enum Value {
  Number(f64),
  Bool(bool),
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Self::Number(num) => write!(f, "{:.4}", num),
        Self::Bool(bool) => write!(f, "{}", bool)
      }
  }
}