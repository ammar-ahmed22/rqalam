#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    PERCENT,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    PLUS_EQUAL,
    MINUS_EQUAL,
    STAR_EQUAL,
    SLASH_EQUAL,
    INCREMENT,
    DECREMENT,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    CONST,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    BREAK,
    CONTINUE,
    INHERITS,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a [u8],
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, literal: &'a [u8], line: usize) -> Self {
        Self {
            token_type,
            literal,
            line,
        }
    }
}
