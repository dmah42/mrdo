use crate::compiler::tokens::Token;

use nom::types::CompleteStr;
use nom::*;

named!(pub addition_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("+") >>
            (
                Token::AdditionOp
            )
        )
    )
);

named!(pub subtraction_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("-") >>
            (
                Token::SubtractionOp
            )
        )
    )
);

named!(pub multiplication_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("*") >>
            (
                Token::MultiplicationOp
            )
        )
    )
);

named!(pub division_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("/") >>
            (
                Token::DivisionOp
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator() {
        let result = addition_op(CompleteStr("+"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::AdditionOp);

        let result = subtraction_op(CompleteStr("-"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::SubtractionOp);

        let result = multiplication_op(CompleteStr("*"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::MultiplicationOp);

        let result = division_op(CompleteStr("/"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::DivisionOp);
    }
}
