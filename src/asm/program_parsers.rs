use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::newline;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::asm::directive_parsers::*;
use crate::asm::instruction_parsers::*;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

fn comment(i: &str) -> IResult<&str, Instruction> {
    log::debug!("[asm::comment] parsing '{}'", i);
    map_res(
        pair(tag(";"), take_until("\n")),
        |(_, comment)| -> Result<Instruction, nom::error::Error<&str>> {
            log::debug!("[asm::comment] success ({:?})", comment);
            Ok(Instruction::new_comment())
        },
    )(i)
}

fn line(i: &str) -> IResult<&str, Instruction> {
    terminated(alt((comment, instruction, directive)), newline)(i)
}

pub fn program(i: &str) -> IResult<&str, Program> {
    map_res(
        many1(line),
        |instructions| -> Result<Program, nom::error::Error<&str>> { Ok(Program { instructions }) },
    )(i)
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use super::*;
    use crate::asm::opcode::Opcode;
    use crate::asm::Token;

    fn init() {
        let _ = pretty_env_logger::formatted_builder()
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
