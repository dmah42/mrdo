use crate::asm::directive_parsers::*;
use crate::asm::instruction_parsers::*;
use crate::asm::symbols::Table;

use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self, symbols: &Table) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes(symbols));
        }
        program
    }
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

    #[test]
    fn test_parse_program() {
        let result = program(CompleteStr("load $1 #42\nload $2 #10\n"));
        assert!(result.is_ok());

        let (left, program) = result.unwrap();
        assert_eq!(left, CompleteStr(""));
        assert_eq!(2, program.instructions.len());
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program(CompleteStr("load $1 #42\nload %2 #10.4\n"));
        assert!(result.is_ok());

        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes(&Table::new());
        println!("bytecode: {:?}", bytecode);
        assert_eq!(bytecode.len(), 16);
    }

    #[test]
    fn test_complete_program() {
        let test_program = CompleteStr(".data\nhello: .str 'Hello!'\n.code\nhlt");
        let result = program(test_program);
        assert!(result.is_ok());
    }
}
