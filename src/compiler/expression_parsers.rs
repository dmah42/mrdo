use crate::compiler::operator_parsers::{addition_op, subtraction_op};
use crate::compiler::term_parsers::term;
use crate::compiler::tokens::Token;

use nom::types::CompleteStr;
use nom::*;

named!(pub expression<CompleteStr, Token>,
    ws!(
        do_parse!(
            left: term >>
            right: many0!(
                tuple!(
                    alt!(
                        addition_op |
                        subtraction_op
                    ),
                    term
                )
            ) >>
            (
                {
                    Token::Expression{ left: Box::new(left), right }
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() {
        let result = expression(CompleteStr("3.2 * 1.4"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Expression {
                left: Box::new(Token::Term {
                    left: Box::new(Token::Factor {
                        value: Box::new(Token::Real { value: 3.2 })
                    }),
                    right: vec![(
                        Token::MultiplicationOp,
                        Token::Factor {
                            value: Box::new(Token::Real { value: 1.4 })
                        }
                    )]
                }),
                right: vec![]
            }
        );
    }
}
