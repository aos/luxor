// Recursive descent

use crate::ast::*;
use crate::token::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // expression -> equality ;
    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    // equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Box<Expr> {
        let expr = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::Equal]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: *operator.clone(),
                right,
            });
        }

        expr
    }

    // comparison -> term ( ( "<" | "<=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Box<Expr> {
        let expr = self.term();

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: *operator.clone(),
                right,
            });
        }

        expr
    }

    // term -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Box<Expr> {
        let expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: *operator.clone(),
                right,
            });
        }

        expr
    }

    // factor -> unary ( ( "/" | "*" ) unary )* ;
    fn factor(&self) -> Box<Expr> {
        let expr = self.unary();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: *operator.clone(),
                right,
            });
        }

        expr
    }

    // unary -> ( "!" | "-" ) unary | primary ;
    fn unary(&self) -> Box<Expr> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Box::new(Expr::Unary {
                operator: *operator.clone(),
                right
            })
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.match_tokens(&[TokenType::False]) {

        }
        if self.match_tokens(&[TokenType::True]) {

        }
        if self.match_tokens(&[TokenType::Nil]) {

        }

        if self.is_literal() {
            self.advance();
            return Box::new(Expr::Lit(*self.previous().clone()))
        }
    }

    fn is_literal(&self) -> bool {
        match self.peek().kind {
            TokenType::Literal(_) => true,
            _ => false
        }
    }

    fn match_tokens(&mut self, tkns: &[TokenType]) -> bool {
        for tkn in tkns {
            if self.check(tkn) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.current += 1
        }

        self.previous()
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.at_end() {
            return false;
        }

        &self.peek().kind == t
    }

    fn at_end(&self) -> bool {
        self.peek().kind == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
