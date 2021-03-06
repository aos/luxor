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
                }
                '=' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::EqualEqual);
                    } else {
                        self.add_token(TokenType::Equal);
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::LessEqual);
                    } else {
                        self.add_token(TokenType::Less);
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::GreaterEqual);
                    } else {
                        self.add_token(TokenType::Greater);
                    }
                }
                '/' => {
                    if self.match_char('/') {
                        // we found two forward slashes, consume until end of line
                        while let Some(c) = self.peek() {
                            if *c == '\n' {
                                break;
                            }

                            self.advance();
                        }
                    } else if self.match_char('*') {
                        // C-style /* ... */ block comment
                        while let Some(c) = self.peek() {
                            match c {
                                '\n' => {
                                    self.line += 1;
                                    self.advance();
                                }
                                '*' => {
                                    if let Some(next) = self.peek_next() {
                                        match next {
                                            '/' => {
                                                // consume */ and exit
                                                self.advance();
                                                self.advance();
                                                break;
                                            }
                                            _ => {
                                              self.advance();
                                            }
                                        }
                                    }
                                }
                                _ => {
                                  self.advance();
                                }
                            }
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,
                '"' => self.read_string(),
                '0'..='9' => self.read_number(c),
                'A'..='Z' | 'a'..='z' | '_' => self.read_identifier(c),
                _ => self.add_token(TokenType::Error("Unrecognized token".to_string())),
            }
        }

        self.tokens.push(Token::new( TokenType::EOF, self.line));

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

    fn peek_next(&mut self) -> Option<char> {
        let mut iter_dup = self.source.clone();
        if let Some(_) = iter_dup.next() {
            iter_dup.next()
        } else {
            None
        }

    }

    fn read_string(&mut self) {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            match c {
                '"' => {
                    self.advance(); // consume the last `"`
                    self.add_token(TokenType::Literal(LiteralKind::Str(s)));
                    return
                }
                _ => {
                    s.push(*c);
                    if *c == '\n' {
                        self.line += 1;
                    }

                    self.advance();
                }
            }
        }

        self.add_token(TokenType::Error("Unterminated string".to_string()));
    }

    fn read_number(&mut self, n: char) {
        let mut num = n.to_string();
        while let Some(c) = self.peek() {
            match *c {
                d if c.is_digit(10) => {
                    num.push(d);
                    self.advance();
                }
                '.' => {
                    if let Some(cn) = self.peek_next() {
                        if cn.is_digit(10) {
                            num.push('.');
                            self.advance();
                        }
                        break
                    }
                }
                _ => break
            }
        }

        while let Some(c) = self.peek() {
            if !c.is_digit(10) {
                break;
            };

            num.push(c.clone());
            self.advance();
        }

        if let Ok(f) = num.parse::<f64>() {
            self.add_token(TokenType::Literal(LiteralKind::Number(f)));
        }
    }

    fn read_identifier(&mut self, c: char) {
        let mut s = String::from(c);
        while let Some(chr) = self.peek() {
            match *chr {
                y if Self::is_alphanumeric(y) => {
                    s.push(y);
                    self.advance();
                }
                _ => break
            }
        }

        self.add_token(Self::lookup_ident(s));
    }

    fn add_token(&mut self, t: TokenType) {
        self.tokens.push(Token::new(t, self.line));
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    fn lookup_ident(s: String) -> TokenType {
        match s.as_str() {
            "and"    => TokenType::And,
            "class"  => TokenType::Class,
            "else"   => TokenType::Else,
            "false"  => TokenType::False,
            "for"    => TokenType::For,
            "fun"    => TokenType::Fun,
            "if"     => TokenType::If,
            "nil"    => TokenType::Nil,
            "or"     => TokenType::Or,
            "print"  => TokenType::Print,
            "return" => TokenType::Return,
            "super"  => TokenType::Super,
            "this"   => TokenType::This,
            "true"   => TokenType::True,
            "var"    => TokenType::Var,
            "while"  => TokenType::While,
            _        => TokenType::Literal(LiteralKind::Identifier(s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_decimal() {
        let num = "123.45";
        let mut sc = Scanner::new(num);
        let tokens = sc.scan_tokens();
        let expected = vec![
            Token::new(TokenType::Literal(LiteralKind::Number(123.45)), 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(tokens, &expected);
    }

    #[test]
    fn test_multiple_dots() {
        let num = "123.45..5.5 3.1 != 6";
        let mut sc = Scanner::new(num);
        let tokens = sc.scan_tokens();
        let expected = vec![
            Token::new(TokenType::Literal(LiteralKind::Number(123.45)), 1),
            Token::new(TokenType::Dot, 1),
            Token::new(TokenType::Dot, 1),
            Token::new(TokenType::Literal(LiteralKind::Number(5.5)), 1),
            Token::new(TokenType::Literal(LiteralKind::Number(3.1)), 1),
            Token::new(TokenType::BangEqual, 1),
            Token::new(TokenType::Literal(LiteralKind::Number(6.0)), 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(tokens, &expected);
    }

    #[test]
    fn test_quoted_string() {
        let s = r#""howdy
partner""#;
        let mut sc = Scanner::new(s);
        let tokens = sc.scan_tokens();
        let expected = vec![
            Token::new(TokenType::Literal(LiteralKind::Str("howdy\npartner".to_string())), 2),
            Token::new(TokenType::EOF, 2),
        ];
        assert_eq!(tokens, &expected);
    }

    #[test]
    fn test_equal_tokens() {
        let s = "!= <= !! >= ==";
        let mut sc = Scanner::new(s);
        let tokens = sc.scan_tokens();
        let expected = vec![
            Token::new(TokenType::BangEqual, 1),
            Token::new(TokenType::LessEqual, 1),
            Token::new(TokenType::Bang, 1),
            Token::new(TokenType::Bang, 1),
            Token::new(TokenType::GreaterEqual, 1),
            Token::new(TokenType::EqualEqual, 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(tokens, &expected);
    }

    #[test]
    fn test_double_slash_comment() {
        let s = r#""string here" != 56 // Nothing followed"#;
        let mut sc = Scanner::new(s);
        let tokens = sc.scan_tokens();
        let expected = vec![
            Token::new(TokenType::Literal(LiteralKind::Str("string here".to_string())), 1),
            Token::new(TokenType::BangEqual, 1),
            Token::new(TokenType::Literal(LiteralKind::Number(56.0)), 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(tokens, &expected);
    }

    #[test]
    fn test_reserved_words() {
        let s = r#"if 3 and 5 or "hello" else nil"#;
        let mut sc = Scanner::new(s);
        let tokens = sc.scan_tokens();
        let expected = vec![
            Token::new(TokenType::If, 1),
            Token::new(TokenType::Literal(LiteralKind::Number(3.0)), 1),
            Token::new(TokenType::And, 1),
            Token::new(TokenType::Literal(LiteralKind::Number(5.0)), 1),
            Token::new(TokenType::Or, 1),
            Token::new(TokenType::Literal(LiteralKind::Str("hello".to_string())), 1),
            Token::new(TokenType::Else, 1),
            Token::new(TokenType::Nil, 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(tokens, &expected);
    }

    #[test]
    fn test_statement() {
        let s = "var x = 3.5 + 1;";
        let mut sc = Scanner::new(s);
        let tokens = sc.scan_tokens();
        let expected = vec![
            Token::new(TokenType::Var, 1),
            Token::new(TokenType::Literal(LiteralKind::Identifier("x".to_string())), 1),
            Token::new(TokenType::Equal, 1),
            Token::new(TokenType::Literal(LiteralKind::Number(3.5)), 1),
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::Literal(LiteralKind::Number(1.0)), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(tokens, &expected);
    }

    #[test]
    fn test_small_program() {
        let input = "var three = 3;
// this is a comment
var x = 8;

/* and a block
 * multi-line comment
 */
var add = fun(i, j) {
    return i + j;
}

var result = add(three /* inside a statement */, x);

if (3 < 8) {
    return true;
} else {
    return false;
};";
        let mut sc = Scanner::new(input);
        let tokens = sc.scan_tokens();
        let expected = vec![
            Token::new(TokenType::Var, 1),
            Token::new(TokenType::Literal(LiteralKind::Identifier("three".to_string())), 1),
            Token::new(TokenType::Equal, 1),
            Token::new(TokenType::Literal(LiteralKind::Number(3.0)), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Var, 3),
            Token::new(TokenType::Literal(LiteralKind::Identifier("x".to_string())), 3),
            Token::new(TokenType::Equal, 3),
            Token::new(TokenType::Literal(LiteralKind::Number(8.0)), 3),
            Token::new(TokenType::Semicolon, 3),
            Token::new(TokenType::Var, 8),
            Token::new(TokenType::Literal(LiteralKind::Identifier("add".to_string())), 8),
            Token::new(TokenType::Equal, 8),
            Token::new(TokenType::Fun, 8),
            Token::new(TokenType::LeftParen, 8),
            Token::new(TokenType::Literal(LiteralKind::Identifier("i".to_string())), 8),
            Token::new(TokenType::Comma, 8),
            Token::new(TokenType::Literal(LiteralKind::Identifier("j".to_string())), 8),
            Token::new(TokenType::RightParen, 8),
            Token::new(TokenType::LeftBrace, 8),
            Token::new(TokenType::Return, 9),
            Token::new(TokenType::Literal(LiteralKind::Identifier("i".to_string())), 9),
            Token::new(TokenType::Plus, 9),
            Token::new(TokenType::Literal(LiteralKind::Identifier("j".to_string())), 9),
            Token::new(TokenType::Semicolon, 9),
            Token::new(TokenType::RightBrace, 10),
            Token::new(TokenType::Var, 12),
            Token::new(TokenType::Literal(LiteralKind::Identifier("result".to_string())), 12),
            Token::new(TokenType::Equal, 12),
            Token::new(TokenType::Literal(LiteralKind::Identifier("add".to_string())), 12),
            Token::new(TokenType::LeftParen, 12),
            Token::new(TokenType::Literal(LiteralKind::Identifier("three".to_string())), 12),
            Token::new(TokenType::Comma, 12),
            Token::new(TokenType::Literal(LiteralKind::Identifier("x".to_string())), 12),
            Token::new(TokenType::RightParen, 12),
            Token::new(TokenType::Semicolon, 12),
            Token::new(TokenType::If, 14),
            Token::new(TokenType::LeftParen, 14),
            Token::new(TokenType::Literal(LiteralKind::Number(3.0)), 14),
            Token::new(TokenType::Less, 14),
            Token::new(TokenType::Literal(LiteralKind::Number(8.0)), 14),
            Token::new(TokenType::RightParen, 14),
            Token::new(TokenType::LeftBrace, 14),
            Token::new(TokenType::Return, 15),
            Token::new(TokenType::True, 15),
            Token::new(TokenType::Semicolon, 15),
            Token::new(TokenType::RightBrace, 16),
            Token::new(TokenType::Else, 16),
            Token::new(TokenType::LeftBrace, 16),
            Token::new(TokenType::Return, 17),
            Token::new(TokenType::False, 17),
            Token::new(TokenType::Semicolon, 17),
            Token::new(TokenType::RightBrace, 18),
            Token::new(TokenType::Semicolon, 18),
            Token::new(TokenType::EOF, 18),
        ];
        assert_eq!(tokens, &expected);
    }
}
