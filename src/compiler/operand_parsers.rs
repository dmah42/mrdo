use crate::compiler::tokens::Token;

use nom::types::CompleteStr;
use nom::*;

// TODO: extend real to be "operand" with multiple types.
named!(pub real<CompleteStr, Token>,
    ws!(
        do_parse!(
            sign: opt!(tag!("-")) >>
            real: double >>
            (
                {
                    let mut tmp = String::from("");
                    if sign.is_some() {
                        tmp.push_str("-");
                    }
                    tmp.push_str(&real.to_string());
                    let converted = tmp.parse::<f64>().unwrap();
                    Token::Real{ value: converted }
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real() {
        let result = real(CompleteStr("3.42"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Real { value: 3.42 });

        // negative numbers
        let result = real(CompleteStr("-3.42"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Real { value: -3.42 });

        // failure
        let result = real(CompleteStr("foo"));
        assert!(result.is_err());
    }
}
