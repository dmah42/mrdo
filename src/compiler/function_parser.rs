use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use super::{expression_parsers::expression, r#type::Type, tokens::Token};

fn type_ident(i: &str) -> IResult<&str, Type> {
    map_res(
        alt((tag("real"), tag("integer"), tag("coll"))),
        |t| -> Result<Type, nom::error::Error<&str>> { Type::try_from(t) },
    )(i)
}

fn arg(i: &str) -> IResult<&str, Token> {
    map_res(
        tuple((
            alpha1,
            delimited(multispace0, tag(":"), multispace0),
            type_ident,
        )),
        |(name, _, typ)| -> Result<Token, nom::error::Error<&str>> {
            Ok(Token::Arg {
                ident: String::from(name),
                typ,
            })
        },
    )(i)
}

pub fn function(i: &str) -> IResult<&str, Option<Token>> {
    map_res(
        tuple((
            preceded(terminated(tag("func"), multispace1), alpha1),
            delimited(
                delimited(multispace0, tag("("), multispace0),
                separated_list1(delimited(multispace0, tag(","), multispace0), arg),
                delimited(multispace0, tag(")"), multispace0),
            ),
            delimited(
                delimited(multispace0, tag("{"), multispace0),
                many1(expression),
                delimited(multispace0, tag("}"), multispace0),
            ),
        )),
        |(name, args, body)| -> Result<Option<Token>, nom::error::Error<&str>> {
            Ok(Some(Token::Function {
                name: String::from(name),
                args,
                body,
            }))
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_ident() {
        assert_eq!(type_ident("real"), Ok(("", Type::Real)));
        assert!(type_ident("foo").is_err());
    }

    #[test]
    fn test_arg() {
        assert_eq!(
            arg("foo: coll"),
            Ok((
                "",
                Token::Arg {
                    ident: String::from("foo"),
                    typ: Type::Coll
                }
            ))
        );
    }

    #[test]
    fn test_function() {
        let test_function = "func foobar(baz: real) {
baz = baz + 1
}";

        assert_eq!(
            function(test_function),
            Ok((
                "",
                Some(Token::Function {
                    name: String::from("foobar"),
                    args: vec![Token::Arg {
                        ident: String::from("baz"),
                        typ: Type::Real
                    }],
                    body: vec![Some(Token::Expression {
                        source: String::from("baz = baz + 1"),
                        token: Box::new(Token::Assign {
                            ident: String::from("baz"),
                            expr: Box::new(Token::Arith {
                                left: Box::new(Token::Term {
                                    left: Box::new(Token::Factor {
                                        value: Box::new(Token::Identifier {
                                            name: String::from("baz")
                                        })
                                    }),
                                    right: vec![]
                                }),
                                right: vec![(
                                    Token::AdditionOp {},
                                    Token::Term {
                                        left: Box::new(Token::Factor {
                                            value: Box::new(Token::Integer { value: 1 })
                                        }),
                                        right: vec![]
                                    }
                                )]
                            })
                        })
                    })]
                })
            ))
        );
    }
}
