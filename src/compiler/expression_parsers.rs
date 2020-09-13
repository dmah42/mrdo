use crate::compiler::builtin_parsers::builtin;
use crate::compiler::operator_parsers::*;
use crate::compiler::term_parsers::term;
use crate::compiler::tokens::Token;

use nom::types::CompleteStr;
use nom::*;

named!(arith<CompleteStr, Token>,
    ws!(
        do_parse!(
            left: term >>
            right: many0!(
                tuple!(
                    alt!(
                        addition_op |
                        subtraction_op
                    ),
                    term
                )
            ) >>
            (
                {
                    Token::Expression{ left: Box::new(left), right }
                }
            )
        )
    )
);

named!(compare<CompleteStr, Token>,
    ws!(
        do_parse!(
            left: arith >>
            op: alt!(
                eq_op | neq_op | gte_op | gt_op | lte_op | lt_op
            ) >>
            right: arith >>
            (
                {
                    Token::Compare{ left: Box::new(left), op: Box::new(op), right: Box::new(right) }
                }
            )
        )
    )
);

named!(assign<CompleteStr, Token>,
    ws!(
        do_parse!(
            ident: alpha >>
            tag!("=") >>
            expr: arith >>
            (
                {
                    Token::Assign{ ident: ident.to_string(), expr: Box::new(expr) }
                }
            )
        )
    )
);

named!(pub expression<CompleteStr, Token>,
    alt!(
        builtin | assign | compare | arith
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arith() {
        let result = arith(CompleteStr("3.2 * 1.4"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Expression {
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
        let result = assign(CompleteStr("foo = 1.3 + 4.1"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Assign {
                ident: "foo".to_string(),
                expr: Box::new(Token::Expression {
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
    fn test_compare() {
        let result = compare(CompleteStr("1.3 + 4.1 neq 2.1"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Compare {
                left: Box::new(Token::Expression {
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
                right: Box::new(Token::Expression {
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
}
