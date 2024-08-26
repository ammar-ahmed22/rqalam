use std::fmt;


#[derive(Debug)]
pub enum ErrorType {
  Syntax,
  Compile,
  Runtime
}

impl fmt::Display for ErrorType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let str = match self {
        Self::Compile => "CompileError",
        Self::Runtime => "RuntimError",
        Self::Syntax => "SyntaxError"
      };
      write!(f, "{}", str)
  }
}


#[derive(Debug)]
pub struct QalamError {
  details: String,
  err_type: ErrorType
}

impl QalamError {
  pub fn new(details: &str, err_type: ErrorType) -> Self {
    Self {
      details: details.to_string(),
      err_type
    }
  }

  pub fn new_compile(details: &str) -> Self {
    Self::new(details, ErrorType::Compile)
  }

  pub fn new_runtime(details: &str) -> Self {
    Self::new(details, ErrorType::Runtime)
  }

  pub fn new_syntax(details: &str) -> Self {
    Self::new(details, ErrorType::Syntax)
  }
}

impl fmt::Display for QalamError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}: {}", self.err_type, self.details)
  }
}

impl std::error::Error for QalamError {}