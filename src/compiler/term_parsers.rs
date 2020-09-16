use crate::compiler::factor_parsers::factor;
use crate::compiler::operator_parsers::{division_op, multiplication_op};
use crate::compiler::tokens::Token;

use nom::types::CompleteStr;
use nom::*;

named!(pub term<CompleteStr, Token>,
    ws!(
        do_parse!(
            left: factor >>
            right: many0!(
                tuple!(
                    alt!(
                        multiplication_op |
                        division_op
                    ),
                    factor
                )
            ) >>
            (
                {
                    Token::Term{left: Box::new(left), right}
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_term() {
        let result = term(CompleteStr("3 * 4"));
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        assert_eq!(
            tree,
            Token::Term {
                left: Box::new(Token::Factor {
                    value: Box::new(Token::Real { value: 3.0 })
                }),
                right: vec![(
                    Token::MultiplicationOp,
                    Token::Factor {
                        value: Box::new(Token::Real { value: 4.0 })
                    },
                )]
            }
        )
    }

    #[test]
    fn test_parse_nested_term() {
        let result = term(CompleteStr("(3 * 4)*2"));
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        assert_eq!(
            tree,
            Token::Term {
                left: Box::new(Token::Factor {
                    value: Box::new(Token::Expression {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Real { value: 3.0 })
                            }),
                            right: vec![(
                                Token::MultiplicationOp,
                                Token::Factor {
                                    value: Box::new(Token::Real { value: 4.0 }),
                                }
                            )]
                        }),
                        right: vec![],
                    })
                }),
                right: vec![(
                    Token::MultiplicationOp,
                    Token::Factor {
                        value: Box::new(Token::Real { value: 2.0 })
                    },
                )]
            }
        )
    }

    #[test]
    fn test_parse_double_nested_term() {
        let result = term(CompleteStr("((3 * 4)*2)"));
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        assert_eq!(
            tree,
            Token::Term {
                left: Box::new(Token::Factor {
                    value: Box::new(Token::Expression {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Expression {
                                    left: Box::new(Token::Term {
                                        left: Box::new(Token::Factor {
                                            value: Box::new(Token::Real { value: 3.0 })
                                        }),
                                        right: vec![(
                                            Token::MultiplicationOp,
                                            Token::Factor {
                                                value: Box::new(Token::Real { value: 4.0 }),
                                            }
                                        )]
                                    }),
                                    right: vec![],
                                }),
                            }),
                            right: vec![(
                                Token::MultiplicationOp,
                                Token::Factor {
                                    value: Box::new(Token::Real { value: 2.0 })
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
