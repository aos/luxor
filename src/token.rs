#[derive(Debug)]
pub enum TokenType {
    // Single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two char tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Literal(LiteralKind),

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF,

    // Error
    Error,
}

#[derive(Debug)]
pub enum LiteralKind {
    Str(String),
    Number(f64),
    Identifier(String),
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub line: u32,
}

impl Token {
    pub fn new(kind: TokenType, line: u32) -> Token {
        Token { kind, line }
    }

    pub fn is_unknown(&self) -> bool {
        match self.kind {
            TokenType::Error => true,
            _ => false,
        }
    }
}
