use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::{Literal, Token};
use crate::token_type::TokenType;

use anyhow::{Context, Result};
use thiserror::Error;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement());
        }

        Ok(statements)
    }

    fn statement(&self) -> Result<Stmt> {
        if self.matches(TokenType::Print) {
            return self.print_statement();
        };

        self.expression_statement()
    }

    fn print_statement(&self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon)
            .context("Expect ';' after value.")?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon)
            .context("Expect ';' after value.")?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality().map(|x| *x)
    }

    fn equality(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.comparison()?;

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual].into_iter()) {
            let operator = self.previous().clone();
            let right = self.comparison()?.clone();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.term()?;

        while self.matches(
            vec![
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ]
            .into_iter(),
        ) {
            let operator = self.previous().clone();
            let right = self.term()?.clone();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.factor()?;

        while self.matches(vec![TokenType::Minus, TokenType::Plus].into_iter()) {
            let operator = self.previous().clone();
            let right = self.factor()?.clone();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.unary()?;

        while self.matches(vec![TokenType::Slash, TokenType::Star].into_iter()) {
            let operator = self.previous().clone();
            let right = self.unary()?.clone();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>> {
        let match_vec = vec![TokenType::Bang, TokenType::Minus].into_iter();

        while self.matches(match_vec) {
            let operator = self.previous().clone();
            let right = self.unary()?.clone();
            return Ok(Box::new(Expr::Unary { operator, right }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>> {
        if self.matches(vec![TokenType::False].into_iter()) {
            return Ok(Box::new(Expr::Literal {
                literal: Literal::Bool(false),
            }));
        }

        if self.matches(vec![TokenType::True].into_iter()) {
            return Ok(Box::new(Expr::Literal {
                literal: Literal::Bool(true),
            }));
        }

        if self.matches(vec![TokenType::Nil].into_iter()) {
            return Ok(Box::new(Expr::Literal {
                literal: Literal::Nil,
            }));
        }

        if self.matches(vec![TokenType::Number, TokenType::String].into_iter()) {
            return Ok(Box::new(Expr::Literal {
                literal: self.previous().literal.clone().unwrap(),
            }));
        }

        if self.matches(vec![TokenType::LeftParen].into_iter()) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen)
                .context("Expect ')' after expression.")?;

            return Ok(Box::new(Expr::Grouping { expression: expr }));
        }

        Err(ParseError::ExpectExpression.into())
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }
        }

        self.advance();
    }

    fn consume(&mut self, token_type: TokenType) -> Result<&Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(ParseError::Consume.into())
    }

    fn matches(&mut self, token_types: impl Iterator<Item = TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("failed to consume")]
    Consume,
    #[error("expected expression")]
    ExpectExpression,
}
