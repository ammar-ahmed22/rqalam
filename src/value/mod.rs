use std::fmt::Display;

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Null,
    String(String),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Null, Self::Null) => true,
            (Self::String(a), Self::String(b)) => a == b,
            _ => false,
        }
    }
}

impl Value {
    pub fn is_falsy(&self) -> bool {
        if let Value::Bool(bool) = self {
            return !bool;
        } else if let Value::Null = self {
            return true;
        } else {
            return false;
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{:.4}", num),
            Self::Bool(bool) => write!(f, "{}", if *bool { "haqq" } else { "batil" }),
            Self::Null => write!(f, "ghaib"),
            Self::String(string) => write!(f, "\"{}\"", string),
        }
    }
}
