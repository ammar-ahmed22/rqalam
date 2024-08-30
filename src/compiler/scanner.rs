use super::token::{Token, TokenType};
use std::{cell::RefCell, marker::PhantomData};

use crate::error::QalamError;

pub struct Scanner<'a> {
    stream: Vec<u8>,
    current: RefCell<usize>,
    start: RefCell<usize>,
    line: RefCell<usize>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Scanner<'a> {
    pub fn new(stream: Vec<u8>) -> Self {
        Self {
            stream,
            current: RefCell::new(0),
            start: RefCell::new(0),
            line: RefCell::new(1),
            phantom: PhantomData,
        }
    }

    pub fn is_at_end(&self) -> bool {
        return *self.current.borrow() >= (self.stream.len() - 1);
    }

    fn make_token(&'a self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            &self.stream[*self.start.borrow()..*self.current.borrow()],
            *self.line.borrow(),
        )
    }

    fn advance(&self) -> char {
        let c = self.stream[*self.current.borrow()];
        if *self.current.borrow() < self.stream.len() {
            self.current.replace_with(|&mut old| old + 1);
        }
        return c as char;
    }

    fn peek(&self) -> char {
        return self.stream[*self.current.borrow()] as char;
    }

    fn peek_next(&self) -> char {
        return self.stream[*self.current.borrow() + 1] as char;
    }

    fn skip_whitespace(&self) {
        loop {
            if self.is_at_end() {
                break;
            }
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line.replace_with(|&mut old| old + 1);
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn match_char(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let curr = self.stream[*self.current.borrow()] as char;
        if curr != expected {
            return false;
        }
        self.current.replace_with(|&mut old| old + 1);
        return true;
    }

    fn string(&self) -> Result<Token, QalamError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line.replace_with(|&mut old| old + 1);
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(QalamError::new_syntax("Unterminated string!"));
        }

        self.advance();
        return Ok(self.make_token(TokenType::STRING));
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn check_keyword(&self, start: usize, rest: &str, token_type: TokenType) -> TokenType {
        let diff = *self.current.borrow() - *self.start.borrow();
        let length = rest.len();
        if diff == start + length {
            let moved_start = *self.start.borrow() + start;
            let check_slice = &self.stream[moved_start..(moved_start + length)];
            if check_slice == rest.as_bytes() {
                return token_type;
            }
        }

        return TokenType::IDENTIFIER;
    }

    fn identifier_type(&self) -> TokenType {
        match self.stream[*self.start.borrow()] as char {
            'a' => {
                if *self.current.borrow() - *self.start.borrow() > 1 {
                    match self.stream[*self.start.borrow() + 1] as char {
                        'w' => return TokenType::OR,
                        'm' => return self.check_keyword(2, "al", TokenType::FUN),
                        _ => {}
                    }
                }
            }
            'b' => {
                if *self.current.borrow() - *self.start.borrow() > 1 {
                    match self.stream[*self.start.borrow() + 1] as char {
                        'a' => {
                            if *self.current.borrow() - *self.start.borrow() > 2 {
                                match self.stream[*self.start.borrow() + 2] as char {
                                    't' => return self.check_keyword(3, "il", TokenType::FALSE),
                                    'y' => return self.check_keyword(3, "nama", TokenType::WHILE),
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            'g' => return self.check_keyword(1, "haib", TokenType::NIL),
            'h' => return self.check_keyword(1, "aqq", TokenType::TRUE),
            'i' => {
                if *self.current.borrow() - *self.start.borrow() > 1 {
                    match self.stream[*self.start.borrow() + 1] as char {
                        'f' => return self.check_keyword(2, "tar", TokenType::BREAK),
                        'b' => return self.check_keyword(2, "n", TokenType::INHERITS),
                        'l' => return self.check_keyword(2, "la", TokenType::ELSE),
                        't' => return self.check_keyword(2, "ha", TokenType::IF),
                        _ => {}
                    }
                }
            }
            'k' => return self.check_keyword(1, "itab", TokenType::CLASS),
            'l' => {
                if *self.current.borrow() - *self.start.borrow() > 1 {
                    match self.stream[*self.start.borrow() + 1] as char {
                        'a' => {
                            if *self.current.borrow() - *self.start.borrow() > 2 {
                                match self.stream[*self.start.borrow() + 2] as char {
                                    'z' => return self.check_keyword(3, "im", TokenType::CONST),
                                    _ => {}
                                }
                            } else {
                                return TokenType::BANG;
                            }
                        }
                        _ => {}
                    }
                }
            }
            'n' => {
                if *self.current.borrow() - *self.start.borrow() > 1 {
                    match self.stream[*self.start.borrow() + 1] as char {
                        'a' => return self.check_keyword(2, "fs", TokenType::THIS),
                        _ => {}
                    }
                }
            }
            'q' => {
                return self.check_keyword(1, "ul", TokenType::PRINT);
            }
            'r' => {
                return self.check_keyword(1, "add", TokenType::RETURN);
            }
            's' => {
                if *self.current.borrow() - *self.start.borrow() > 1 {
                    match self.stream[*self.start.borrow() + 1] as char {
                        'h' => return self.check_keyword(2, "ai", TokenType::VAR),
                        'a' => return self.check_keyword(2, "far", TokenType::CONTINUE),
                        _ => {}
                    }
                }
            }
            't' => {
                return self.check_keyword(1, "awaf", TokenType::FOR);
            }
            'u' => return self.check_keyword(1, "lya", TokenType::SUPER),
            'w' => return self.check_keyword(1, "a", TokenType::AND),
            _ => {}
        }
        return TokenType::IDENTIFIER;
    }

    fn identifier(&self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }

        return self.make_token(self.identifier_type());
    }

    fn number(&self) -> Token {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        return self.make_token(TokenType::NUMBER);
    }

    pub fn scan(&self) -> Result<Token, QalamError> {
        self.skip_whitespace();
        *self.start.borrow_mut() = *self.current.borrow();
        if self.is_at_end() {
            return Ok(Token::new(
                TokenType::EOF,
                "eof".as_bytes(),
                *self.line.borrow(),
            ));
        }

        let c = self.advance();

        if self.is_alpha(c) {
            return Ok(self.identifier());
        }

        if self.is_digit(c) {
            return Ok(self.number());
        }
        match c {
            '(' => return Ok(self.make_token(TokenType::LEFT_PAREN)),
            ')' => return Ok(self.make_token(TokenType::RIGHT_PAREN)),
            '{' => return Ok(self.make_token(TokenType::LEFT_BRACE)),
            '}' => return Ok(self.make_token(TokenType::RIGHT_BRACE)),
            ';' => return Ok(self.make_token(TokenType::SEMICOLON)),
            ',' => return Ok(self.make_token(TokenType::COMMA)),
            '.' => return Ok(self.make_token(TokenType::DOT)),
            '-' => return Ok(self.make_token(TokenType::MINUS)),
            '+' => return Ok(self.make_token(TokenType::PLUS)),
            '*' => return Ok(self.make_token(TokenType::STAR)),
            '%' => return Ok(self.make_token(TokenType::PERCENT)),
            '/' => return Ok(self.make_token(TokenType::SLASH)),
            '&' => {
                if self.match_char('&') {
                    return Ok(self.make_token(TokenType::AND))
                }
            },
            '|' => {
                if self.match_char('|') {
                    return Ok(self.make_token(TokenType::OR))
                }
            },
            '!' => {
                if self.match_char('=') {
                    return Ok(self.make_token(TokenType::BANG_EQUAL));
                } else {
                    return Ok(self.make_token(TokenType::BANG));
                }
            }
            '=' => {
                if self.match_char('=') {
                    return Ok(self.make_token(TokenType::EQUAL_EQUAL));
                } else {
                    return Ok(self.make_token(TokenType::EQUAL));
                }
            }
            '<' => {
                if self.match_char('=') {
                    return Ok(self.make_token(TokenType::LESS_EQUAL));
                } else {
                    return Ok(self.make_token(TokenType::LESS));
                }
            }
            '>' => {
                if self.match_char('=') {
                    return Ok(self.make_token(TokenType::GREATER_EQUAL));
                } else {
                    return Ok(self.make_token(TokenType::GREATER));
                }
            }
            '"' => return Ok(self.string()?),

            _ => {}
        }
        return Err(QalamError::new_syntax(&format!(
            "Unexpected character.\n\tat line {}\n\tat '{}'",
            self.line.borrow(),
            c
        )));
    }
}
