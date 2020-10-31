use std::fmt;

#[derive(Debug, PartialEq)]
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

    Error(String),
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Str(String),
    Number(f64),
    Identifier(String),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub line: u32,
}

impl Token {
    pub fn new(kind: TokenType, line: u32) -> Token {
        Token { kind, line }
    }

    pub fn is_error(&self) -> bool {
        match self.kind {
            TokenType::Error(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TokenType::LeftParen    => f.write_str("("),
            TokenType::RightParen   => f.write_str(")"),
            TokenType::LeftBrace    => f.write_str("{{"),
            TokenType::RightBrace   => f.write_str("}}"),
            TokenType::Comma        => f.write_str(","),
            TokenType::Dot          => f.write_str("."),
            TokenType::Minus        => f.write_str("-"),
            TokenType::Plus         => f.write_str("+"),
            TokenType::Semicolon    => f.write_str(";"),
            TokenType::Slash        => f.write_str("/"),
            TokenType::Star         => f.write_str("*"),
            TokenType::Bang         => f.write_str("!"),
            TokenType::BangEqual    => f.write_str("!="),
            TokenType::Equal        => f.write_str("="),
            TokenType::EqualEqual   => f.write_str("=="),
            TokenType::Greater      => f.write_str(">"),
            TokenType::GreaterEqual => f.write_str(">="),
            TokenType::Less         => f.write_str("<"),
            TokenType::LessEqual    => f.write_str("<="),
            TokenType::And          => f.write_str("and"),
            TokenType::Class        => f.write_str("class"),
            TokenType::Else         => f.write_str("else"),
            TokenType::False        => f.write_str("false"),
            TokenType::For          => f.write_str("for"),
            TokenType::Fun          => f.write_str("fun"),
            TokenType::If           => f.write_str("if"),
            TokenType::Nil          => f.write_str("nil"),
            TokenType::Or           => f.write_str("or"),
            TokenType::Print        => f.write_str("print"),
            TokenType::Return       => f.write_str("return"),
            TokenType::Super        => f.write_str("super"),
            TokenType::This         => f.write_str("this"),
            TokenType::True         => f.write_str("true"),
            TokenType::Var          => f.write_str("var"),
            TokenType::While        => f.write_str("while"),
            TokenType::EOF          => f.write_str(""),

            TokenType::Literal(LiteralKind::Str(s))
            | TokenType::Literal(LiteralKind::Identifier(s))
            | TokenType::Error(s)   => f.write_str(s),

            TokenType::Literal(LiteralKind::Number(n)) => write!(f, "{}", n),
        }
    }
}
