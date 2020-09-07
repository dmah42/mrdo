use crate::expression_parsers::expression;
use crate::tokens::Token;
use nom::types::CompleteStr;
use nom::*;

named!(pub program<CompleteStr, Token>,
    ws!(
        do_parse!(
            expressions: many1!(expression) >>
            (
                Token::Program {
                    expressions
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let result = program(CompleteStr("1.2 + 0.3\n2.4 * 4.0"));
        assert!(result.is_ok());

        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Program {
                expressions: vec![
                    Token::Expression {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Real { value: 1.2 }),
                            }),
                            right: vec![]
                        }),
                        right: vec![(
                            Token::AdditionOp,
                            Token::Term {
                                left: Box::new(Token::Factor {
                                    value: Box::new(Token::Real { value: 0.3 })
                                }),
                                right: vec![]
                            }
                        )]
                    },
                    Token::Expression {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Real { value: 2.4 }),
                            }),
                            right: vec![(
                                Token::MultiplicationOp,
                                Token::Factor {
                                    value: Box::new(Token::Real { value: 4.0 })
                                },
                            )]
                        }),
                        right: vec![]
                    },
                ]
            }
        );
    }
}
