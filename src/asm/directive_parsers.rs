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

named!(pub directive<CompleteStr, Instruction>,
    ws!(
        do_parse!(
            l: opt!(label_decl) >>
            name: directive_decl >>
            o0: opt!(operand) >>
            _o1: opt!(operand) >>
            _o2: opt!(operand) >>
            (
                Instruction::new_directive(name, l, o0)
            )
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
        let result = directive(CompleteStr("test: .str 'Hello'"));
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
