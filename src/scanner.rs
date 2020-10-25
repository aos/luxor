use std::str::Chars;
use std::iter::Peekable;
use crate::token::*;

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    total_len: u32,
    tokens: Vec<Token>,
    start_pos: u32,
    current_pos: u32,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: source.chars().peekable(),
            total_len: source.len() as u32,
            tokens: Vec::new(),
            start_pos: 0,
            current_pos: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.at_end() {
            self.start_pos = self.current_pos;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            self.line,
        ));

        &self.tokens
    }

    fn at_end(&self) -> bool {
        self.current_pos >= self.total_len
    }

    fn advance(&mut self) -> Option<char> {
        self.current_pos += 1;
        self.source.next()
    }

    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '*' => self.add_token(TokenType::Star),
                _ => println!("{}", c)
            }
        }
    }

    fn add_token(&mut self, t: TokenType) {
        self.tokens.push(Token::new(t, self.line));
    }
}
