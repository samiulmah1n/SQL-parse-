use crate::token::{Token, Keyword};
use std::str::Chars;
use std::iter::Peekable;

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(&c) = self.input.peek() {
            match c {
                '+' => { self.input.next(); tokens.push(Token::Plus); }
                '-' => { self.input.next(); tokens.push(Token::Minus); }
                '*' => { self.input.next(); tokens.push(Token::Star); }
                '/' => { self.input.next(); tokens.push(Token::Slash); }
                '=' => { self.input.next(); tokens.push(Token::Eq); }
                _ => { self.input.next(); } // placeholder
            }
        }
        Ok(tokens)
    }
}