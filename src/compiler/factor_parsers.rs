use crate::compiler::expression_parsers::expression;
use crate::compiler::operand_parsers::real;
use crate::compiler::tokens::Token;

use nom::types::CompleteStr;
use nom::*;

named!(pub factor<CompleteStr, Token>,
    ws!(
        do_parse!(
            f: alt!(
                real |
                ws!(delimited!(tag!("("), expression, tag!(")")))
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
        let result = factor(CompleteStr("(1+2)"));
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        println!("{:?}", tree);
    }
}
