use crate::compiler::{builtin::Builtin, expression_parsers::*, tokens::Token};

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
            match Builtin::try_from(arg0) {
                Ok(builtin) => Ok(match arg1 {
                    Some(rvs) => Token::Builtin { builtin, args: rvs },
                    None => Token::Builtin {
                        builtin,
                        args: vec![],
                    },
                }),
                Err(e) => {
                    log::error!("Unknown builtin: {}", arg0);
                    Err(e)
                }
            }
        },
    )(i)
}

#[cfg(test)]
mod tests {

    use super::*;

    //use log::LevelFilter;
    fn init() {
        //let _ = pretty_env_logger::formatted_builder()
        //    .is_test(true)
        //    .filter_level(LevelFilter::Debug)
        //    .try_init();
    }

    #[test]
    fn test_unknown_builtin() {
        init();

        let result = builtin("do(foo)");
        assert!(result.is_err());
    }

    #[test]
    fn test_builtin_noargs() {
        init();

        let result = builtin("do(map)");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Builtin {
                builtin: Builtin::try_from("map").unwrap(),
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
                builtin: Builtin::try_from("write").unwrap(),
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

        let result = builtin("do (fold, foo, 1.0)");
        //assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Builtin {
                builtin: Builtin::try_from("fold").unwrap(),
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
