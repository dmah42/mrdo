use crate::asm::instruction_parsers::Instruction;
use crate::asm::label_parsers::label_decl;
use crate::asm::operand_parsers::operand;
use crate::asm::Token;

use nom::alpha1;
use nom::types::CompleteStr;

named!(directive_decl<CompleteStr, Token>,
    do_parse!(
        tag!(".") >>
        name: alpha1 >>
        (
            Token::Directive{name: name.to_string()}
        )
    )
);

named!(directive_comb<CompleteStr, Instruction>,
    ws!(
        do_parse!(
            l: opt!(label_decl) >>
            name: directive_decl >>
            o0: opt!(operand) >>
            o1: opt!(operand) >>
            o2: opt!(operand) >>
            (
                Instruction{
                    opcode: None,
                    directive: Some(name),
                    label: l,
                    operand0: o0,
                    operand1: o1,
                    operand2: o2,
                }
            )
        )
    )
);

named!(pub directive<CompleteStr, Instruction>,
    do_parse!(
        instr: alt!(
            directive_comb
        ) >>
        (
            instr
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_directive() {
        let result = directive_decl(CompleteStr(".data"));
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
        let result = directive_comb(CompleteStr("test: .str 'Hello'"));
        assert!(result.is_ok());
        let (_, directive) = result.unwrap();

        let expected = Instruction {
            opcode: None,
            label: Some(Token::LabelDecl {
                name: "test".to_string(),
            }),
            directive: Some(Token::Directive {
                name: "str".to_string(),
            }),
            operand0: Some(Token::DoString {
                value: "Hello".to_string(),
            }),
            operand1: None,
            operand2: None,
        };

        assert_eq!(directive, expected);
    }
}
