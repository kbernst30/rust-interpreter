use super::token::Token;
use super::token::TokenType;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {

    pub fn new(input: Peekable<Chars<'a>>) -> Lexer {
        Lexer { input: input }
    }

    pub fn get_token(&mut self) -> Token {

        while self.is_next_whitespace() {
            self.input.next();
        }

        match self.input.next() {
            Some('+')                     => Token::new(TokenType::PLUS, String::from("+")),
            Some('-')                     => Token::new(TokenType::MINUS, String::from("-")),
            Some('*')                     => Token::new(TokenType::ASTERISK, String::from("*")),
            Some('/')                     => Token::new(TokenType::SLASH, String::from("/")),
            Some('=')                     => {
                if self.is_next_check('=') {
                    self.input.next();
                    Token::new(TokenType::EQEQ, String::from("=="))
                } else {
                    Token::new(TokenType::EQ, String::from("="))
                }
            },
            Some('>')                     => {
                if self.is_next_check('=') {
                    self.input.next();
                    Token::new(TokenType::GTEQ, String::from(">="))
                } else {
                    Token::new(TokenType::GT, String::from(">"))
                }
            },
            Some('<')                     => {
                if self.is_next_check('<') {
                    self.input.next();
                    Token::new(TokenType::LTEQ, String::from("<="))
                } else {
                    Token::new(TokenType::LT, String::from("<"))
                }
            },
            Some('!')                     => {
                if self.is_next_check('=') {
                    self.input.next();
                    Token::new(TokenType::NOTEQ, String::from("!="))
                } else {
                    panic!("Invalid token found");
                }
            },
            Some('"')                     => self.process_string(),
            Some(c) if c.is_ascii_digit() => self.process_number(&c),
            Some(c) if c.is_alphabetic()  => self.process_alpha(&c),
            Some(';')                     => Token::new(TokenType::SEMICOLON, String::from(";")),
            Some(_)                       => Token::new(TokenType::ILLEGAL, String::from("")),
            None                          => Token::new(TokenType::EOF, String::from("\0")),
        }

    }

    pub fn has_next_token(&mut self) -> bool {
        match self.input.peek() {
            Some(&_) => true,
            None => false
        }
    }

    fn process_string(&mut self) -> Token {
        let mut value = String::from("");
        let mut end_value = '"';
        while let Some(c) = self.input.next() {
            // If end of string, with quote, break
            end_value = c;
            if c == '"' {
                break;
            }

            value.push_str(&c.to_string());
        }

        if end_value != '"' {
            // End value was not end quote, bad string - exit lexer
            // as we can't process this
            panic!("Unclosed string literal found");
        }

        Token::new(TokenType::STRING, value)
    }

    fn process_number(&mut self, start_char: &char) -> Token {
        let mut value = start_char.to_string();
        while self.is_next_digit() {
            // Safe to unwrap from the above digit check
            let next = self.input.next().unwrap();
            value.push_str(&next.to_string());
        }

        // Check for decimal
        if self.is_next_check('.') {
            // Safe to unwrap here
            value.push_str(&(self.input.next().unwrap()).to_string());

            // Check for more digits on right side of decimal
            if !self.is_next_digit() {
                panic!("Invalid number found");
            }

            while self.is_next_digit() {
                // Safe to unwrap from the above digit check
                let next = self.input.next().unwrap();
                value.push_str(&next.to_string());
            }
        }

        Token::new(TokenType::NUMBER, value)
    }

    fn process_alpha(&mut self, start_char: &char) -> Token {
        let mut value = start_char.to_string();
        while self.is_next_alphanumeric() {
            // Safe to unwrap from the above alphenumeric check
            let next = self.input.next().unwrap();
            value.push_str(&next.to_string());
        }

        let token_type = TokenType::get_keyword_token(&value);

        // If we found a keyword, return that token, otherwise
        // Random alphanumeric non-quoted string will be an ident
        match token_type {
            Some(t) => Token::new(t, value),
            None    => Token::new(TokenType::IDENT, value)
        }
    }

    fn is_next_whitespace(&mut self) -> bool {
        match self.input.peek() {
            Some(&c) => c.is_whitespace(),
            None => false
        }
    }

    fn is_next_alphanumeric(&mut self) -> bool {
        match self.input.peek() {
            Some(&c) => c.is_alphanumeric(),
            None => false
        }
    }

    fn is_next_digit(&mut self) -> bool {
        match self.input.peek() {
            Some(&c) => c.is_ascii_digit(),
            None => false
        }
    }

    fn is_next_check(&mut self, check: char) -> bool {
        match self.input.peek() {
            Some(&c) => c == check,
            None => false
        }
    }
}
