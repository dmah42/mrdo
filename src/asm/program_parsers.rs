use crate::asm::directive_parsers::*;
use crate::asm::instruction_parsers::*;

use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

named!(pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(alt!(instruction | directive)) >>
        (
            Program { instructions }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::opcode::Opcode;
    use crate::asm::Token;

    #[test]
    fn test_parse_program() {
        let result = program(CompleteStr("load $1 #42\nload %2 #10.4\n"));
        assert!(result.is_ok());

        let (left, program) = result.unwrap();
        assert_eq!(left, CompleteStr(""));
        assert_eq!(2, program.instructions.len());
        assert_eq!(
            program.instructions,
            vec![
                Instruction::new_opcode(
                    Token::Op { code: Opcode::LOAD },
                    Some(Token::IntRegister { idx: 1 }),
                    Some(Token::Integer { value: 42 }),
                    None
                ),
                Instruction::new_opcode(
                    Token::Op { code: Opcode::LOAD },
                    Some(Token::RealRegister { idx: 2 }),
                    Some(Token::Real { value: 10.4 }),
                    None
                ),
            ]
        )
    }

    #[test]
    fn test_complete_program() {
        let test_program = CompleteStr(".data\nhello: .str 'Hello!'\n.code\nhlt");
        let result = program(test_program);
        assert!(result.is_ok());
    }
}
