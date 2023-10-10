//! Module for performing lexical analysis on source code.

use core::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
}

// fn is_letter(ch: char) -> bool {
//     'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
// }

// fn is_digit(ch: char) -> bool {
//     '0' <= ch && ch <= '9'
// }

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            // println!("{:?}", token);
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let next_char = self.chars.next()?;
        match next_char {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '0'..='9' => {
                let mut number = next_char.to_digit(10)? as i32;
                while let Some(next_char) = self.chars.clone().next() {
                    if let Some(digit) = next_char.to_digit(10) {
                        number = number * 10 + digit as i32;
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                Some(Token::Number(number))
            }
            _ => None,
        }
    }
}
