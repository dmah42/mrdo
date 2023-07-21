use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    sequence::preceded, IResult,
};

use crate::asm::Token;

fn iregister(i: &str) -> IResult<&str, Token> {
    map_res(
        preceded(tag("$i"), digit1),
        |idx: &str| -> Result<Token, nom::error::Error<&str>> {
            Ok(Token::IntRegister {
                idx: idx.parse::<u8>().unwrap(),
            })
        },
    )(i)
}

fn rregister(i: &str) -> IResult<&str, Token> {
    map_res(
        preceded(tag("$r"), digit1),
        |idx: &str| -> Result<Token, nom::error::Error<&str>> {
            Ok(Token::RealRegister {
                idx: idx.parse::<u8>().unwrap(),
            })
        },
    )(i)
}

fn vregister(i: &str) -> IResult<&str, Token> {
    map_res(
        preceded(tag("$v"), digit1),
        |idx: &str| -> Result<Token, nom::error::Error<&str>> {
            Ok(Token::VectorRegister {
                idx: idx.parse::<u8>().unwrap(),
            })
        },
    )(i)
}

pub fn register(i: &str) -> IResult<&str, Token> {
    alt((iregister, rregister, vregister))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$i1");
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::IntRegister { idx: 1 });

        let result = register("$r1");
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::RealRegister { idx: 1 });

        let result = register("$v30");
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::VectorRegister { idx: 30 });

        let result = register("0");
        assert!(result.is_err());

        let result = register("$a");
        assert!(result.is_err());
    }
}
