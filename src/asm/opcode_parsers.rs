use crate::asm::Opcode;
use crate::asm::Token;

use nom::character::complete::alpha1;
use nom::combinator::map_res;
use nom::*;

pub fn opcode(i: &str) -> IResult<&str, Token> {
    map_res(alpha1, |opcode| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::Op {
            code: Opcode::from(opcode),
        })
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode("load");
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");
    }

    #[test]
    fn test_unknown_opcode() {
        let result = opcode("aold");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
    }
}
