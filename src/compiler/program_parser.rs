use nom::branch::alt;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

use crate::compiler::expression_parsers::expression;
use crate::compiler::tokens::Token;

use super::function_parser::function;

pub fn program(i: &str) -> IResult<&str, Token> {
    map_res(
        many1(alt((function, expression))),
        |funcs_or_exprs| -> Result<Token, nom::error::Error<&str>> {
            Ok(Token::Program {
                statements: funcs_or_exprs,
            })
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let result = program("1.2 + 0.3\n2.4 * 4.0\n");
        assert!(result.is_ok());

        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Program {
                statements: vec![
                    Some(Token::Expression {
                        source: String::from("1.2 + 0.3"),
                        token: Box::new(Token::Arith {
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
                        },)
                    }),
                    Some(Token::Expression {
                        source: String::from("2.4 * 4.0"),
                        token: Box::new(Token::Arith {
                            left: Box::new(Token::Term {
                                left: Box::new(Token::Factor {
                                    value: Box::new(Token::Real { value: 2.4 }),
                                }),
                                right: vec![(
                                    Token::MultiplicationOp,
                                    Token::Factor {
                                        value: Box::new(Token::Integer { value: 4 })
                                    },
                                )]
                            }),
                            right: vec![]
                        },)
                    })
                ]
            }
        );
    }
}
