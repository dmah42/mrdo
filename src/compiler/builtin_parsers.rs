use crate::compiler::expression_parsers::*;
use crate::compiler::tokens::Token;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::{map_res, opt};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;

pub fn builtin(i: &str) -> IResult<&str, Token> {
    log::debug!("[builtin] parsing '{}'", i);
    map_res(
        pair(
            tag("do"),
            delimited(
                delimited(multispace0, tag("("), multispace0),
                pair(
                    alpha1,
                    opt(preceded(
                        delimited(multispace0, tag(","), multispace0),
                        separated_list1(delimited(multispace0, tag(","), multispace0), rvalue),
                    )),
                ),
                preceded(multispace0, tag(")")),
            ),
        ),
        |(_, (arg0, arg1))| -> Result<Token, nom::error::Error<&str>> {
            log::debug!("[builtin] success ({:?}, {:?})", arg0, arg1);
            Ok(match arg1 {
                Some(rvs) => Token::Builtin {
                    builtin: String::from(arg0),
                    args: rvs,
                },
                None => Token::Builtin {
                    builtin: String::from(arg0),
                    args: vec![],
                },
            })
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use super::*;

    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Debug)
            .try_init();
    }

    #[test]
    fn test_builtin_noargs() {
        init();

        let result = builtin("do(foo)");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Builtin {
                builtin: String::from("foo"),
                args: vec![],
            }
        );
    }
    #[test]
    fn test_builtin_onearg() {
        init();

        let result = builtin("do(write, 42.3)");
        //assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Builtin {
                builtin: String::from("write"),
                args: vec![Token::Arith {
                    left: Box::new(Token::Term {
                        left: Box::new(Token::Factor {
                            value: Box::new(Token::Real { value: 42.3 })
                        }),
                        right: vec![],
                    }),
                    right: vec![],
                }],
            }
        );
    }

    #[test]
    fn test_builtin_multiple_args() {
        init();

        let result = builtin("do (read, foo, 1.0)");
        //assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Builtin {
                builtin: String::from("read"),
                args: vec![
                    Token::Arith {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Identifier {
                                    name: "foo".to_string()
                                })
                            }),
                            right: vec![],
                        }),
                        right: vec![],
                    },
                    Token::Arith {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Integer { value: 1 })
                            }),
                            right: vec![],
                        }),
                        right: vec![],
                    }
                ]
            }
        );
    }
}
