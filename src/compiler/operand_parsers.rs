use crate::compiler::expression_parsers::*;
use crate::compiler::tokens::Token;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    combinator::map_res,
    multi::separated_list0,
    number::complete::double,
    sequence::delimited,
    IResult,
};

pub fn num(i: &str) -> IResult<&str, Token> {
    map_res(double, |value| -> Result<Token, nom::error::Error<&str>> {
        Ok(if value == (value as i32) as f64 {
            Token::Integer {
                value: value as i32,
            }
        } else {
            Token::Real { value }
        })
    })(i)
}

pub fn ident(i: &str) -> IResult<&str, Token> {
    map_res(alpha1, |name| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::Identifier {
            name: String::from(name),
        })
    })(i)
}

pub fn coll(i: &str) -> IResult<&str, Token> {
    map_res(
        delimited(
            tag("["),
            separated_list0(delimited(multispace0, tag(","), multispace0), rvalue),
            tag("]"),
        ),
        |values| -> Result<Token, nom::error::Error<&str>> { Ok(Token::Coll { values }) },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num() {
        let result = num("3.42");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Real { value: 3.42 });

        // negative numbers
        let result = num("-3.42");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Real { value: -3.42 });

        // negative integer
        let result = num("-42");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Integer { value: -42 });

        // failure
        let result = num("foo");
        assert!(result.is_err());
    }

    #[test]
    fn test_ident() {
        let result = ident("foo");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Identifier {
                name: "foo".to_string()
            }
        );
    }

    #[test]
    fn test_coll() {
        let result = coll("[]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().1, Token::Coll { values: vec![] });

        let result = coll("[3+4, 42.3, foo]");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1,
            Token::Coll {
                values: vec![
                    Token::Arith {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Integer { value: 3 })
                            }),
                            right: vec![]
                        }),
                        right: vec![(
                            Token::AdditionOp,
                            Token::Term {
                                left: Box::new(Token::Factor {
                                    value: Box::new(Token::Integer { value: 4 })
                                }),
                                right: vec![],
                            },
                        )],
                    },
                    Token::Arith {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Real { value: 42.3 })
                            }),
                            right: vec![],
                        }),
                        right: vec![],
                    },
                    Token::Arith {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Identifier {
                                    name: "foo".to_string()
                                }),
                            }),
                            right: vec![],
                        }),
                        right: vec![],
                    },
                ]
            }
        );
    }
}
