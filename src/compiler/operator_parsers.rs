use nom::{bytes::complete::tag, combinator::map_res, IResult};

use crate::compiler::tokens::Token;

//named!(pub addition_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("+") >> ( Token::AdditionOp )
//        )
//    )
//);

pub fn addition_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("+"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::AdditionOp)
    })(i)
}

//named!(pub subtraction_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("-") >> ( Token::SubtractionOp )
//        )
//    )
//);
pub fn subtraction_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("-"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::SubtractionOp)
    })(i)
}

//named!(pub multiplication_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("*") >> ( Token::MultiplicationOp )
//        )
//    )
//);
pub fn multiplication_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("*"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::MultiplicationOp)
    })(i)
}

//named!(pub division_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("/") >> ( Token::DivisionOp )
//        )
//    )
//);
pub fn division_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("/"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::DivisionOp)
    })(i)
}

//named!(pub eq_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("eq") >> ( Token::EqualsOp )
//        )
//    )
//);
pub fn eq_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("eq"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::EqualsOp)
    })(i)
}

//named!(pub neq_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("neq") >> ( Token::NotEqualsOp )
//        )
//    )
//);
pub fn neq_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("neq"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::NotEqualsOp)
    })(i)
}

//named!(pub gt_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("gt") >> ( Token::GreaterThanOp )
//        )
//    )
//);
pub fn gt_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("gt"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::GreaterThanOp)
    })(i)
}
//
//named!(pub gte_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("gte") >> ( Token::GreaterThanEqualsOp )
//        )
//    )
//);
pub fn gte_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("gte"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::GreaterThanEqualsOp)
    })(i)
}
//
//named!(pub lt_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("lt") >> ( Token::LessThanOp )
//        )
//    )
//);
pub fn lt_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("lt"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::LessThanOp)
    })(i)
}
//
//named!(pub lte_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("lte") >> ( Token::LessThanEqualsOp )
//        )
//    )
//);
pub fn lte_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("lte"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::LessThanEqualsOp)
    })(i)
}
//
//named!(pub and_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("and") >> ( Token::AndOp )
//        )
//    )
//);
pub fn and_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("and"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::AndOp)
    })(i)
}
//
//named!(pub or_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("or") >> ( Token::OrOp )
//        )
//    )
//);
pub fn or_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("or"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::OrOp)
    })(i)
}
//
//named!(pub not_op<CompleteStr, Token>,
//    ws!(
//        do_parse!(
//            tag!("not") >> ( Token::NotOp )
//        )
//    )
//);
pub fn not_op(i: &str) -> IResult<&str, Token> {
    map_res(tag("not"), |_| -> Result<Token, nom::error::Error<&str>> {
        Ok(Token::NotOp)
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator() {
        let result = addition_op("+");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::AdditionOp);

        let result = subtraction_op("-");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::SubtractionOp);

        let result = multiplication_op("*");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::MultiplicationOp);

        let result = division_op("/");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::DivisionOp);

        let result = eq_op("eq");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::EqualsOp);

        let result = neq_op("neq");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::NotEqualsOp);

        let result = gt_op("gt");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::GreaterThanOp);

        let result = gte_op("gte");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::GreaterThanEqualsOp);

        let result = lt_op("lt");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::LessThanOp);

        let result = lte_op("lte");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::LessThanEqualsOp);
    }
}
