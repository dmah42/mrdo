use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::sequence::delimited;
use nom::IResult;

use crate::compiler::expression_parsers::*;
use crate::compiler::operand_parsers::*;
use crate::compiler::tokens::Token;

pub fn factor(i: &str) -> IResult<&str, Token> {
    log::debug!("[factor] parsing '{}'", i);
    map_res(
        alt((num, coll, delimited(tag("("), rvalue, tag(")")), ident)),
        |factor| -> Result<Token, nom::error::Error<&str>> {
            log::debug!("[factor] success ({:?})", factor);
            Ok(Token::Factor {
                value: Box::new(factor),
            })
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor() {
        let result = factor("(1+2)");
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        assert_eq!(
            tree,
            Token::Factor {
                value: Box::new(Token::Arith {
                    left: Box::new(Token::Term {
                        left: Box::new(Token::Factor {
                            value: Box::new(Token::Integer { value: 1 })
                        }),
                        right: vec![]
                    }),
                    right: vec![(
                        Token::AdditionOp,
                        Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Integer { value: 2 })
                            }),
                            right: vec![]
                        }
                    )]
                })
            }
        );

        let result = factor("3.1 + foo");
        assert!(result.is_ok());
        let (rest, tree) = result.unwrap();
        assert_eq!(
            tree,
            Token::Factor {
                value: Box::new(Token::Real { value: 3.1 })
            }
        );
        assert_eq!(rest, " + foo");
    }
}
