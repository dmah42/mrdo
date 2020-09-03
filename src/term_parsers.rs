use nom::types::CompleteStr;
use nom::*;

use crate::factor_parsers::factor;
use crate::operator_parsers::{division_op, multiplication_op};
use crate::tokens::Token;

named!(pub term<CompleteStr, Token>,
    ws!(
        do_parse!(
            left: factor >>
            right: many0!(
                tuple!(
                    alt!(
                        multiplication_op |
                        division_op
                    ),
                    factor
                )
            ) >>
            (
                {
                    Token::Term{left: Box::new(left), right}
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_term() {
        let result = term(CompleteStr("3 * 4"));
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        println!("{:#?}", tree);
    }

    #[test]
    fn test_parse_nested_term() {
        let result = term(CompleteStr("(3 * 4)*2"));
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        println!("{:#?}", tree);
    }

    #[test]
    fn test_parse_double_nested_term() {
        let result = term(CompleteStr("((3 * 4)*2)"));
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        println!("{:#?}", tree);
    }
}
