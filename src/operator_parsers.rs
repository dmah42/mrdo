use crate::tokens::Token;
use nom::types::CompleteStr;
use nom::*;

named!(addition_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("+") >>
            (
                Token::AdditionOp
            )
        )
    )
);

named!(subtraction_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("-") >>
            (
                Token::SubtractionOp
            )
        )
    )
);

named!(multiplication_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("*") >>
            (
                Token::MultiplicationOp
            )
        )
    )
);

named!(division_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("/") >>
            (
                Token::DivisionOp
            )
        )
    )
);

named!(pub operator<CompleteStr, Token>,
    ws!(
        alt!(
            addition_op |
            subtraction_op |
            multiplication_op |
            division_op
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator() {
        let result = operator(CompleteStr("+"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::AdditionOp);

        let result = operator(CompleteStr("-"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::SubtractionOp);

        let result = operator(CompleteStr("*"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::MultiplicationOp);

        let result = operator(CompleteStr("/"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::DivisionOp);
    }
}
