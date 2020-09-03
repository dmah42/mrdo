use crate::operand_parsers::real;
use crate::operator_parsers::operator;
use crate::tokens::Token;
use nom::types::CompleteStr;
use nom::*;

// TODO: strictly speaking this is "arith" but for now it'll do.
named!(pub expression<CompleteStr, Token>,
    ws!(
        do_parse!(
            left: real >>
            op: operator >>
            right: real >>
            (
                Token::Expression{
                    left: Box::new(left),
                    right: Box::new(right),
                    op: Box::new(op)
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
                left: Box::new(Token::Real { value: 3.2 }),
                op: Box::new(Token::MultiplicationOp),
                right: Box::new(Token::Real { value: 1.4 }),
            }
        );
    }
}
