use crate::asm::Token;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0},
    combinator::{map_res, opt},
    sequence::{delimited, tuple},
    IResult,
};

pub fn label_decl(i: &str) -> IResult<&str, Token> {
    map_res(
        tuple((alphanumeric1, tag(":"), opt(multispace0))),
        |(name, _, _)| -> Result<Token, nom::error::Error<&str>> {
            Ok(Token::LabelDecl {
                name: String::from(name),
            })
        },
    )(i)
}

pub fn label_ref(i: &str) -> IResult<&str, Token> {
    map_res(
        delimited(tag("@"), alphanumeric1, multispace0),
        |name| -> Result<Token, nom::error::Error<&str>> {
            Ok(Token::LabelRef {
                name: String::from(name),
            })
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label_decl() {
        let result = label_decl("test:");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::LabelDecl {
                name: "test".to_string()
            }
        );

        let result = label_decl("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_label_ref() {
        let result = label_ref("@test");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::LabelRef {
                name: "test".to_string()
            }
        );

        let result = label_ref("test");
        assert!(result.is_err());
    }
}
