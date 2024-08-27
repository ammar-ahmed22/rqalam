use std::fmt;

use crate::compiler::token::Token;

#[derive(Debug)]
pub enum ErrorType {
    Syntax,
    Compile,
    Runtime,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::Compile => "CompileError",
            Self::Runtime => "RuntimeError",
            Self::Syntax => "SyntaxError",
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub struct QalamError {
    details: String,
    err_type: ErrorType,
}

impl QalamError {
    pub fn new(details: &str, err_type: ErrorType) -> Self {
        Self {
            details: details.to_string(),
            err_type,
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

    fn token_message(details: &str, token: &Token) -> String {
        return format!(
            "{}\n\tat line {}\n\tat '{}'",
            details,
            token.line,
            std::str::from_utf8(token.literal).unwrap()
        );
    }

    pub fn from_token_syntax(details: &str, token: &Token) -> Self {
        Self::new(&Self::token_message(details, token), ErrorType::Syntax)
    }

    pub fn from_token_compile(details: &str, token: &Token) -> Self {
        Self::new(&Self::token_message(details, token), ErrorType::Compile)
    }

    pub fn from_token_runtime(details: &str, token: &Token) -> Self {
        Self::new(&Self::token_message(details, token), ErrorType::Runtime)
    }

    fn message_with_line(details: &str, line: usize) -> String {
        return format!("{}\n\tat line {}", details, line);
    }

    pub fn with_line_syntax(details: &str, line: usize) -> Self {
        Self::new(&Self::message_with_line(details, line), ErrorType::Syntax)
    }

    pub fn with_line_compile(details: &str, line: usize) -> Self {
        Self::new(&Self::message_with_line(details, line), ErrorType::Compile)
    }

    pub fn with_line_runtime(details: &str, line: usize) -> Self {
        Self::new(&Self::message_with_line(details, line), ErrorType::Runtime)
    }
}

impl fmt::Display for QalamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.err_type, self.details)
    }
}

impl std::error::Error for QalamError {}
