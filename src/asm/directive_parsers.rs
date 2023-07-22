use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace1};
use nom::combinator::{map_res, opt};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

use crate::asm::instruction_parsers::Instruction;
use crate::asm::label_parsers::label_decl;
use crate::asm::operand_parsers::operand;
use crate::asm::Token;

fn directive_decl(i: &str) -> IResult<&str, Token> {
    map_res(
        pair(tag("."), alpha1),
        |(_, name)| -> Result<Token, nom::error::Error<&str>> {
            Ok(Token::Directive {
                name: String::from(name),
            })
        },
    )(i)
}

pub fn directive(i: &str) -> IResult<&str, Instruction> {
    log::debug!("[asm::directive] parsing '{}'", i);
    map_res(
        tuple((
            opt(label_decl),
            directive_decl,
            opt(preceded(multispace1, operand)),
            opt(preceded(multispace1, operand)),
            opt(preceded(multispace1, operand)),
        )),
        |(l, name, o0, _o1, _o2)| -> Result<Instruction, nom::error::Error<&str>> {
            log::debug!("[asm::directive] success ({:?}, {:?})", l, name);
            Ok(Instruction::new_directive(name, l, o0))
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_directive() {
        let result = directive_decl(".data");
        assert!(result.is_ok());
        let (_, directive) = result.unwrap();
        assert_eq!(
            directive,
            Token::Directive {
                name: "data".to_string()
            }
        )
    }

    #[test]
    fn test_string_directive() {
        let result = directive("test: .str 'Hello'");
        assert!(result.is_ok());
        let (_, directive) = result.unwrap();

        let expected = Instruction::new_directive(
            Token::Directive {
                name: "str".to_string(),
            },
            Some(Token::LabelDecl {
                name: "test".to_string(),
            }),
            Some(Token::DoString {
                value: "Hello".to_string(),
            }),
        );

        assert_eq!(directive, expected);
    }
}
