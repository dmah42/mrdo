use crate::asm::Opcode;
use crate::asm::Token;

use nom::types::CompleteStr;
use nom::*;

named!(pub opcode<CompleteStr, Token>,
    do_parse!(
        opcode: alpha1 >>
        (
            {
                Token::Op{code: Opcode::from(opcode)}
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode(CompleteStr("load"));
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));

        let result = opcode(CompleteStr("aold"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
    }
}
