use std::fmt;

use crate::token::{Literal, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        literal: Literal,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unary { operator, right } => write!(f, "({} {})", operator.get_lexeme(), right),
            Self::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", operator.get_lexeme(), left, right),
            Self::Grouping { expression } => write!(f, "(group {})", expression),
            Self::Literal { literal } => write!(f, "{}", literal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_type::TokenType;

    #[test]
    fn plus() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                literal: Literal::Number(1.0),
            }),
            operator: Token::new(TokenType::Plus, "+".to_owned(), None, 1),
            right: Box::new(Expr::Literal {
                literal: Literal::Number(2.0),
            }),
        };
        assert_eq!(format!("{}", expr), "(+ 1 2)");
    }

    #[test]
    fn minus() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-".to_owned(), None, 1),
                right: Box::new(Expr::Literal {
                    literal: Literal::Number(123.0),
                }),
            }),
            operator: Token::new(TokenType::Star, "*".to_owned(), None, 1),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    literal: Literal::Number(45.67),
                }),
            }),
        };
        assert_eq!(format!("{}", expr), "(* (- 123) (group 45.67))");
    }
}
