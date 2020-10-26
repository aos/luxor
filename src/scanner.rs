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
        while let Some(c) = self.advance() {
            self.start_pos = self.current_pos;
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
                '!' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::BangEqual);
                    } else {
                        self.add_token(TokenType::Bang);
                    }
                },
                '=' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::EqualEqual);
                    } else {
                        self.add_token(TokenType::Equal);
                    }
                },
                '<' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::LessEqual);
                    } else {
                        self.add_token(TokenType::Less);
                    }
                },
                '>' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::GreaterEqual);
                    } else {
                        self.add_token(TokenType::Greater);
                    }
                },
                '/' => {
                    if self.match_char('/') {
                        // we found two forward slashes, consume until end of line
                        while let Some(c) = self.peek() {
                            if *c != '\n' {
                                self.advance();
                            }
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                },
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,
                '"' => self.read_string(),
                '0'..='9' => self.read_number(c),
                _ => self.add_token(TokenType::Error("Unrecognized token".to_string())),
            }
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            self.line,
        ));

        &self.tokens
    }

    fn advance(&mut self) -> Option<char> {
        self.current_pos += 1;
        self.source.next()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if let Some(c) = self.peek() {
            if *c == expected {
                self.advance();
                return true
            }
        }
        false
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn read_string(&mut self) {
        let mut s = String::new();

        loop {
            match self.peek() {
                Some(c) => {
                    if *c == '"' {
                        break;
                    }

                    s.push(*c);
                    if *c == '\n' {
                        self.line += 1;
                    }

                    self.advance();
                },
                None => {
                    self.add_token(TokenType::Error("Unterminated string".to_string()));
                    return
                }
            }
        }

        self.advance();
        self.add_token(TokenType::Literal(LiteralKind::Str(s)));
    }

    fn read_number(&mut self, n: char) {
        let num = n.to_string();
    }

    fn add_token(&mut self, t: TokenType) {
        self.tokens.push(Token::new(t, self.line));
    }
}
