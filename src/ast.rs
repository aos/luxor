use crate::token::*;
use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Lit(Token),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Expr::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", operator, left, right),
            Expr::Grouping { expression } => write!(f, "(group {})", expression),
            Expr::Lit(t) => write!(f, "{}", t),
            Expr::Unary { operator, right } => write!(f, "({} {})", operator, right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_print() {
        let x = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, 1),
                right: Box::new(Expr::Lit(Token::new(
                    TokenType::Literal(LiteralKind::Number(123.0)),
                    1,
                ))),
            }),
            operator: Token::new(TokenType::Star, 1),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Lit(Token::new(
                    TokenType::Literal(LiteralKind::Number(45.67)),
                    1,
                ))),
            }),
        };

        assert_eq!(x.to_string(), "(* (- 123) (group 45.67))");
    }
}
