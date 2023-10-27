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

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.chars.next() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => continue, // Skip whitespace
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '0'..='9' => {
                    // Process digits
                    let mut number = ch.to_digit(10).unwrap() as i32;
                    while let Some(next_ch) = self.chars.clone().next() {
                        if next_ch.is_ascii_digit() {
                            number = number * 10 + (next_ch.to_digit(10).unwrap() as i32);
                            self.chars.next(); // Move to the next character
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(number));
                }
                _ => {
                    // Handle invalid characters
                    panic!("Invalid character: {}", ch);
                }
            }
        }

        tokens
    }
}
