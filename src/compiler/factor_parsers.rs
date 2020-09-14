use crate::compiler::expression_parsers::expression;
use crate::compiler::operand_parsers::*;
use crate::compiler::tokens::Token;

use nom::types::CompleteStr;
use nom::*;

named!(pub factor<CompleteStr, Token>,
    ws!(
        do_parse!(
            f: alt!(
                real |
                ws!(delimited!(tag!("("), expression, tag!(")"))) |
                ident
            ) >>
            (
                {
                    Token::Factor{value: Box::new(f)}
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor() {
        // TODO: fill these test cases out.
        let result = factor(CompleteStr("(1+2)"));
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        println!("{:?}", tree);

        let result = factor(CompleteStr("3.0 + foo"));
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        println!("{:?}", tree);
    }
}
