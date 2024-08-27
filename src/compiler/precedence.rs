use super::parser::Parser;
use super::token::TokenType;
use crate::error::QalamError;

pub type ParseFn = fn(&Parser) -> Result<(), QalamError>;
pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn new(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: Precedence) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }

    pub fn only_prefix(prefix: ParseFn) -> Self {
        Self {
            prefix: Some(prefix),
            infix: None,
            precedence: Precedence::None,
        }
    }

    pub fn none() -> Self {
        Self {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        }
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl std::ops::Add<usize> for Precedence {
    type Output = Option<Precedence>;
    fn add(self, rhs: usize) -> Self::Output {
        let result = match self {
            Self::None => 0,
            Self::Assignment => 1,
            Self::Or => 2,
            Self::And => 3,
            Self::Equality => 4,
            Self::Comparison => 5,
            Self::Term => 6,
            Self::Factor => 7,
            Self::Unary => 8,
            Self::Call => 9,
            Self::Primary => 10,
        } + rhs;
        match result {
            0 => Some(Self::None),
            1 => Some(Self::Assignment),
            2 => Some(Self::Or),
            3 => Some(Self::And),
            4 => Some(Self::Equality),
            5 => Some(Self::Comparison),
            6 => Some(Self::Term),
            7 => Some(Self::Factor),
            8 => Some(Self::Unary),
            9 => Some(Self::Call),
            10 => Some(Self::Primary),
            _ => None,
        }
    }
}

impl Precedence {
    pub fn get_rule(token_type: TokenType) -> ParseRule {
        match token_type {
            TokenType::LEFT_PAREN => ParseRule::only_prefix(|parser| parser.grouping()),
            TokenType::MINUS => ParseRule::new(
                Some(|parser| parser.unary()),
                Some(|parser| parser.binary()),
                Precedence::Term,
            ),
            TokenType::PLUS => {
                ParseRule::new(None, Some(|parser| parser.binary()), Precedence::Term)
            }
            TokenType::SLASH => {
                ParseRule::new(None, Some(|parser| parser.binary()), Precedence::Factor)
            }
            TokenType::STAR => {
                ParseRule::new(None, Some(|parser| parser.binary()), Precedence::Factor)
            }
            TokenType::NUMBER => ParseRule::only_prefix(|parser| parser.number()),
            TokenType::FALSE => ParseRule::only_prefix(|parser| parser.literal()),
            TokenType::TRUE => ParseRule::only_prefix(|parser| parser.literal()),
            TokenType::NIL => ParseRule::only_prefix(|parser| parser.literal()),
            TokenType::BANG => ParseRule::only_prefix(|parser| parser.unary()),
            TokenType::BANG_EQUAL => {
                ParseRule::new(None, Some(|parser| parser.binary()), Precedence::Equality)
            }
            TokenType::EQUAL_EQUAL => {
                ParseRule::new(None, Some(|parser| parser.binary()), Precedence::Equality)
            }
            TokenType::GREATER_EQUAL => {
                ParseRule::new(None, Some(|parser| parser.binary()), Precedence::Comparison)
            }
            TokenType::GREATER => {
                ParseRule::new(None, Some(|parser| parser.binary()), Precedence::Comparison)
            }
            TokenType::LESS_EQUAL => {
                ParseRule::new(None, Some(|parser| parser.binary()), Precedence::Comparison)
            }
            TokenType::LESS => {
                ParseRule::new(None, Some(|parser| parser.binary()), Precedence::Comparison)
            }
            TokenType::STRING => ParseRule::only_prefix(|parser| parser.literal()),
            _ => ParseRule::none(),
        }
    }
}
