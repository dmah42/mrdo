use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::{delimited, pair};
use nom::IResult;

use crate::compiler::factor_parsers::factor;
use crate::compiler::operator_parsers::{division_op, multiplication_op};
use crate::compiler::tokens::Token;

pub fn term(i: &str) -> IResult<&str, Token> {
    log::debug!("[term] parsing '{}'", i);
    map_res(
        pair(
            factor,
            many0(pair(
                delimited(
                    multispace0,
                    alt((multiplication_op, division_op)),
                    multispace0,
                ),
                factor,
            )),
        ),
        |(left, right)| -> Result<Token, nom::error::Error<&str>> {
            log::debug!("[term] success ({:?}, {:?})", left, right);
            Ok(Token::Term {
                left: Box::new(left),
                right,
            })
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_term() {
        let result = term("3.2 * 4");
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        assert_eq!(
            tree,
            Token::Term {
                left: Box::new(Token::Factor {
                    value: Box::new(Token::Real { value: 3.2 })
                }),
                right: vec![(
                    Token::MultiplicationOp,
                    Token::Factor {
                        value: Box::new(Token::Integer { value: 4 })
                    },
                )]
            }
        )
    }

    #[test]
    fn test_parse_nested_term() {
        let result = term("(3.2 * 4)*2");
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        assert_eq!(
            tree,
            Token::Term {
                left: Box::new(Token::Factor {
                    value: Box::new(Token::Arith {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Real { value: 3.2 })
                            }),
                            right: vec![(
                                Token::MultiplicationOp,
                                Token::Factor {
                                    value: Box::new(Token::Integer { value: 4 }),
                                }
                            )]
                        }),
                        right: vec![],
                    })
                }),
                right: vec![(
                    Token::MultiplicationOp,
                    Token::Factor {
                        value: Box::new(Token::Integer { value: 2 })
                    },
                )]
            }
        )
    }

    #[test]
    fn test_parse_double_nested_term() {
        let result = term("((3 * 4)*2)");
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        assert_eq!(
            tree,
            Token::Term {
                left: Box::new(Token::Factor {
                    value: Box::new(Token::Arith {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Arith {
                                    left: Box::new(Token::Term {
                                        left: Box::new(Token::Factor {
                                            value: Box::new(Token::Integer { value: 3 })
                                        }),
                                        right: vec![(
                                            Token::MultiplicationOp,
                                            Token::Factor {
                                                value: Box::new(Token::Integer { value: 4 }),
                                            }
                                        )]
                                    }),
                                    right: vec![],
                                }),
                            }),
                            right: vec![(
                                Token::MultiplicationOp,
                                Token::Factor {
                                    value: Box::new(Token::Integer { value: 2 })
                                },
                            )],
                        }),
                        right: vec![],
                    }),
                }),
                right: vec![],
            }
        )
    }
}
