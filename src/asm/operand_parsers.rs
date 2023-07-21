use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::map_res;
use nom::number::complete::double;
use nom::sequence::{delimited, preceded};
use nom::IResult;

use crate::asm::label_parsers::label_ref;
use crate::asm::register_parsers::register;
use crate::asm::Token;

pub fn operand(i: &str) -> IResult<&str, Token> {
    alt((num_operand, label_ref, register, string))(i)
}

pub fn num_operand(i: &str) -> IResult<&str, Token> {
    map_res(
        preceded(tag("#"), double),
        |value| -> Result<Token, nom::error::Error<&str>> {
            if value == (value as i32) as f64 {
                Ok(Token::Integer {
                    value: value as i32,
                })
            } else {
                Ok(Token::Real { value })
            }
        },
    )(i)
}

pub fn string(i: &str) -> IResult<&str, Token> {
    map_res(
        delimited(tag("'"), take_until("'"), tag("'")),
        |content| -> Result<Token, nom::error::Error<&str>> {
            Ok(Token::DoString {
                value: String::from(content),
            })
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer() {
        let result = num_operand("#42");
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::Integer { value: 42 });

        let result = num_operand("42");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_real() {
        let result = num_operand("#4.2");
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::Real { value: 4.2 });

        let result = num_operand("4.2");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_operand() {
        let result = operand("#3.145");
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::Real { value: 3.145 });

        let result = operand("#4");
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::Integer { value: 4 });
    }

    #[test]
    fn test_parse_label() {
        let result = operand("@test");
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            value,
            Token::LabelRef {
                name: "test".to_string()
            }
        );

        let result = operand("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_string() {
        let result = string("'hello do'");
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            value,
            Token::DoString {
                value: "hello do".to_string()
            }
        );

        let result = string("'invalid");
        assert!(result.is_err());
    }
}
