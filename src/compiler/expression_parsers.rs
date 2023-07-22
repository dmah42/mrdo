use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, multispace0, newline};
use nom::combinator::{consumed, map_res, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

use crate::compiler::builtin_parsers::builtin;
use crate::compiler::operand_parsers::coll;
use crate::compiler::operator_parsers::*;
use crate::compiler::term_parsers::term;
use crate::compiler::tokens::Token;

fn arith(i: &str) -> IResult<&str, Token> {
    log::debug!("[arith] parsing '{}'", i);
    map_res(
        pair(
            term,
            many0(pair(
                delimited(multispace0, alt((addition_op, subtraction_op)), multispace0),
                term,
            )),
        ),
        |(left, right)| -> Result<Token, nom::error::Error<&str>> {
            log::debug!("[arith] success ({:?}, {:?})", left, right);
            Ok(Token::Arith {
                left: Box::new(left),
                right,
            })
        },
    )(i)
}

fn bin_op(i: &str) -> IResult<&str, Token> {
    log::debug!("[binop] parsing '{}'", i);
    map_res(
        tuple((
            rvalue,
            delimited(
                multispace0,
                alt((eq_op, neq_op, gte_op, gt_op, lte_op, lt_op, and_op, or_op)),
                multispace0,
            ),
            rvalue,
        )),
        |(left, op, right)| -> Result<Token, nom::error::Error<&str>> {
            log::debug!("[binop] success ({:?}, {:?}, {:?})", left, op, right);
            Ok(Token::BinOp {
                left: Box::new(left),
                op: Box::new(op),
                right: Box::new(right),
            })
        },
    )(i)
}

fn unary_op(i: &str) -> IResult<&str, Token> {
    log::debug!("[unaryop] parsing '{}'", i);
    map_res(
        pair(not_op, preceded(multispace0, rvalue)),
        |(op, right)| -> Result<Token, nom::error::Error<&str>> {
            log::debug!("[unaryop] success ({:?}, {:?})", op, right);
            Ok(Token::UnaryOp {
                op: Box::new(op),
                right: Box::new(right),
            })
        },
    )(i)
}

fn assign(i: &str) -> IResult<&str, Token> {
    log::debug!("[assign] parsing '{}'", i);
    map_res(
        tuple((
            alpha1,
            delimited(multispace0, tag("="), multispace0),
            rvalue,
        )),
        |(ident, _, expr)| -> Result<Token, nom::error::Error<&str>> {
            log::debug!("[assign] success ({:?}, {:?})", ident, expr);
            Ok(Token::Assign {
                ident: String::from(ident),
                expr: Box::new(expr),
            })
        },
    )(i)
}

fn comment(i: &str) -> IResult<&str, Token> {
    log::debug!("[comment] parsing '{}'", i);
    map_res(
        pair(tag(";"), take_until("\n")),
        |(_, comment)| -> Result<Token, nom::error::Error<&str>> {
            log::debug!("[comment] success ({:?})", comment);
            Ok(Token::Comment {
                comment: String::from(comment),
            })
        },
    )(i)
}

pub fn rvalue(i: &str) -> IResult<&str, Token> {
    log::debug!("[rvalue] parsing '{}'", i);
    alt((builtin, arith, coll))(i)
}

pub fn expression(i: &str) -> IResult<&str, Option<Token>> {
    log::debug!("[expression] parsing '{}'", i);
    map_res(
        terminated(
            consumed(opt(alt((comment, assign, bin_op, unary_op, rvalue)))),
            newline,
        ),
        |(parsed_source, opt_expr)| -> Result<Option<Token>, nom::error::Error<&str>> {
            Ok(opt_expr.map(|expr| Token::Expression {
                source: String::from(parsed_source),
                token: Box::new(expr),
            }))
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arith() {
        let result = arith("3.2 * 1.4");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Arith {
                left: Box::new(Token::Term {
                    left: Box::new(Token::Factor {
                        value: Box::new(Token::Real { value: 3.2 })
                    }),
                    right: vec![(
                        Token::MultiplicationOp,
                        Token::Factor {
                            value: Box::new(Token::Real { value: 1.4 })
                        }
                    )]
                }),
                right: vec![]
            }
        );
    }

    #[test]
    fn test_assign() {
        let result = assign("foo = 1.3 + 4.1");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Assign {
                ident: "foo".to_string(),
                expr: Box::new(Token::Arith {
                    left: Box::new(Token::Term {
                        left: Box::new(Token::Factor {
                            value: Box::new(Token::Real { value: 1.3 })
                        }),
                        right: vec![],
                    }),
                    right: vec![(
                        Token::AdditionOp,
                        Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Real { value: 4.1 })
                            }),
                            right: vec![],
                        }
                    )]
                })
            }
        )
    }

    #[test]
    fn test_binop() {
        let result = bin_op("1.3 + 4.1 neq 2.1");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::BinOp {
                left: Box::new(Token::Arith {
                    left: Box::new(Token::Term {
                        left: Box::new(Token::Factor {
                            value: Box::new(Token::Real { value: 1.3 }),
                        }),
                        right: vec![],
                    }),
                    right: vec![(
                        Token::AdditionOp,
                        Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Real { value: 4.1 })
                            }),
                            right: vec![],
                        }
                    )]
                }),
                op: Box::new(Token::NotEqualsOp),
                right: Box::new(Token::Arith {
                    left: Box::new(Token::Term {
                        left: Box::new(Token::Factor {
                            value: Box::new(Token::Real { value: 2.1 }),
                        }),
                        right: vec![],
                    }),
                    right: vec![],
                })
            }
        );
    }

    #[test]
    fn test_unary_op() {
        let result = unary_op("not 42.0");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::UnaryOp {
                op: Box::new(Token::NotOp),
                right: Box::new(Token::Arith {
                    left: Box::new(Token::Term {
                        left: Box::new(Token::Factor {
                            value: Box::new(Token::Integer { value: 42 }),
                        }),
                        right: vec![],
                    }),
                    right: vec![],
                }),
            }
        );
    }

    #[test]
    fn test_comment() {
        let result = comment("; this! is a comment\n");
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Comment {
                comment: " this! is a comment".to_string()
            }
        );
        assert_eq!(rest, "\n");

        let result = comment("; this! is a comment\n42.0 + 3.0");
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Comment {
                comment: " this! is a comment".to_string()
            }
        );
        assert_eq!(rest, "\n42.0 + 3.0")
    }
}
