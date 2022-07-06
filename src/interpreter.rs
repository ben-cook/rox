// use std::fmt;

use crate::{
    expr::Expr,
    token::{Literal, Token},
    token_type::TokenType,
};

use anyhow::Result;
// use lazy_static::lazy_static;
use thiserror::Error;

// lazy_static! {
//     pub static ref INTERPRETER: Interpreter = {
//         let interpreter = Interpreter::new();
//         interpreter
//     };
// }

// #[derive(Debug)]
// enum Object {
//     String(String),
//     Number(f32),
//     Bool(bool),
//     Nil,
// }

// impl fmt::Display for Object {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::String(ref string) => write!(f, "{}", string),
//             Self::Number(num) => write!(f, "{}", num),
//             Self::Bool(bool) => write!(f, "{}", bool),
//             Self::Nil => write!(f, "nil"),
//         }
//     }
// }

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        let value = self.evaluate(expr)?;
        println!("{:?}", value);
        Ok(())
    }

    fn evaluate(&self, expr: &Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Literal { literal } => Ok(literal.clone()),
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;

                match operator.token_type {
                    TokenType::Minus => {
                        let number = self.check_number_operand(&operator, &right)?;
                        Ok(Literal::Number(-number))
                    }
                    TokenType::Bang => Ok(Literal::Bool(!self.is_truthy(right))),
                    _ => {
                        unreachable!()
                    }
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match operator.token_type {
                    TokenType::Minus => {
                        let (left, right) = self.check_number_operands(&operator, &left, &right)?;
                        return Ok(Literal::Number(left - right));
                    }
                    TokenType::Slash => {
                        let (left, right) = self.check_number_operands(&operator, &left, &right)?;
                        return Ok(Literal::Number(left / right));
                    }
                    TokenType::Star => {
                        let (left, right) = self.check_number_operands(&operator, &left, &right)?;
                        return Ok(Literal::Number(left * right));
                    }
                    TokenType::Plus => {
                        if let Ok((left, right)) =
                            self.check_number_operands(&operator, &left, &right)
                        {
                            return Ok(Literal::Number(left + right));
                        }

                        if let (Literal::String(left), Literal::String(right)) = (left, right) {
                            return Ok(Literal::String(left.clone() + &right));
                        }

                        Err(RuntimeError::InvalidOperands {
                            operator: operator.clone(),
                        })
                    }
                    TokenType::Greater => {
                        let (left, right) = self.check_number_operands(&operator, &left, &right)?;
                        return Ok(Literal::Bool(left > right));
                    }
                    TokenType::GreaterEqual => {
                        let (left, right) = self.check_number_operands(&operator, &left, &right)?;
                        return Ok(Literal::Bool(left >= right));
                    }
                    TokenType::Less => {
                        let (left, right) = self.check_number_operands(&operator, &left, &right)?;
                        return Ok(Literal::Bool(left < right));
                    }
                    TokenType::LessEqual => {
                        let (left, right) = self.check_number_operands(&operator, &left, &right)?;
                        return Ok(Literal::Bool(left <= right));
                    }
                    TokenType::BangEqual => Ok(Literal::Bool(!self.is_equal(&left, &right))),
                    TokenType::EqualEqual => Ok(Literal::Bool(self.is_equal(&left, &right))),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn check_number_operand(&self, operator: &Token, a: &Literal) -> Result<f32, RuntimeError> {
        if let Literal::Number(a) = a {
            return Ok(*a);
        };

        Err(RuntimeError::InvalidOperands {
            operator: operator.clone(),
        })
    }

    fn check_number_operands(
        &self,
        operator: &Token,
        a: &Literal,
        b: &Literal,
    ) -> Result<(f32, f32), RuntimeError> {
        if let Literal::Number(a) = a {
            if let Literal::Number(b) = b {
                return Ok((*a, *b));
            };
        };

        Err(RuntimeError::InvalidOperands {
            operator: operator.clone(),
        })
    }

    fn is_truthy(&self, literal: Literal) -> bool {
        match literal {
            Literal::Nil => false,
            Literal::Bool(bool) => bool.to_owned(),
            _ => true,
        }
    }

    fn is_equal(&self, left: &Literal, right: &Literal) -> bool {
        match (left, right) {
            (Literal::Number(left), Literal::Number(right)) => left == right,
            (Literal::String(left), Literal::String(right)) => left == right,
            (Literal::Bool(left), Literal::Bool(right)) => left == right,
            (Literal::Nil, Literal::Nil) => true,
            _ => false,
        }
    }
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("expected a different type")]
    ExpectedType,
    #[error("invalid operands for operator {operator:?}")]
    InvalidOperands { operator: Token },
}
