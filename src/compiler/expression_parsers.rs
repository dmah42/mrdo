use crate::compiler::builtin_parsers::builtin;
use crate::compiler::operand_parsers::coll;
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

named!(bin_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            left: rvalue >>
            op: alt!(
                eq_op | neq_op | gte_op | gt_op | lte_op | lt_op |
                and_op | or_op
            ) >>
            right: rvalue >>
            (
                Token::BinOp{ left: Box::new(left), op: Box::new(op), right: Box::new(right) }
            )
        )
    )
);

named!(unary_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            op: not_op >>
            right: rvalue >>
            (
                Token::UnaryOp {op: Box::new(op), right: Box::new(right)}
            )
        )
    )
);

// FIXME: unable to assign to expressions.  figure out the ebnf
named!(assign<CompleteStr, Token>,
    ws!(
        do_parse!(
            ident: alpha >>
            tag!("=") >>
            expr: rvalue >>
            (
                Token::Assign{ ident: ident.to_string(), expr: Box::new(expr) }
            )
        )
    )
);

named!(comment<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!(";") >>
            comment: alt!(
                take_until!("\n") |
                take_while1!(|ch: char| ch.is_ascii())
            ) >>
            (
                Token::Comment{comment: comment.to_string()}
            )
        )
    )
);

named!(pub rvalue<CompleteStr, Token>,
    alt!(
        builtin | arith | coll
    )
);

named!(pub expression<CompleteStr, Token>,
    alt!(
        comment | assign | bin_op | unary_op | rvalue
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
    fn test_binop() {
        let result = bin_op(CompleteStr("1.3 + 4.1 neq 2.1"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::BinOp {
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

    #[test]
    fn test_unary_op() {
        let result = unary_op(CompleteStr("not 42.0"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::UnaryOp {
                op: Box::new(Token::NotOp),
                right: Box::new(Token::Expression {
                    left: Box::new(Token::Term {
                        left: Box::new(Token::Factor {
                            value: Box::new(Token::Real { value: 42.0 }),
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
        let result = comment(CompleteStr("; this! is a comment"));
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Comment {
                comment: "this! is a comment".to_string()
            }
        );
        assert_eq!(rest, CompleteStr(""));

        let result = comment(CompleteStr("; this! is a comment\n42.0 + 3.0"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Comment {
                comment: "this! is a comment".to_string()
            }
        );
    }
}
