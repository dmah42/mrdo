use nom::branch::alt;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

use crate::asm::directive_parsers::*;
use crate::asm::instruction_parsers::*;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

pub fn program(i: &str) -> IResult<&str, Program> {
    log::debug!("[asm::program] parsing '{}'", i);
    map_res(
        many1(alt((instruction, directive))),
        |instructions| -> Result<Program, nom::error::Error<&str>> {
            log::debug!("[asm::program] success ({:?})", instructions);
            Ok(Program { instructions })
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use super::*;
    use crate::asm::opcode::Opcode;
    use crate::asm::Token;

    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Debug)
            .try_init();
    }

    #[test]
    fn test_parse_program() {
        init();

        let result = program("load $i1 #42\nload $r2 #10.4\n");
        assert!(result.is_ok());

        let (left, program) = result.unwrap();
        assert_eq!(left, "");
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
        init();

        let test_program = ".data\nhello: .str 'Hello!'\n.code\nhlt";
        let result = program(test_program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_adhoc() {
        init();
        let test_program = "copy $r30 $r29\nload $i31 #0\nsyscall $i31 $r30";
        let result = program(test_program);
        assert!(result.is_ok());
    }
}
