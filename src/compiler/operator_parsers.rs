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

named!(pub eq_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("eq") >>
            (
                Token::EqualsOp
            )
        )
    )
);

named!(pub neq_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("neq") >>
            (
                Token::NotEqualsOp
            )
        )
    )
);

named!(pub gt_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("gt") >>
            (
                Token::GreaterThanOp
            )
        )
    )
);

named!(pub gte_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("gte") >>
            (
                Token::GreaterThanEqualsOp
            )
        )
    )
);

named!(pub lt_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("lt") >>
            (
                Token::LessThanOp
            )
        )
    )
);

named!(pub lte_op<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("lte") >>
            (
                Token::LessThanEqualsOp
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

        let result = eq_op(CompleteStr("eq"));
        assert!(result.is_ok());

        let result = neq_op(CompleteStr("neq"));
        assert!(result.is_ok());

        let result = gt_op(CompleteStr("gt"));
        assert!(result.is_ok());

        let result = gte_op(CompleteStr("gte"));
        assert!(result.is_ok());

        let result = lt_op(CompleteStr("lt"));
        assert!(result.is_ok());

        let result = lte_op(CompleteStr("lte"));
        assert!(result.is_ok());
    }
}
