use crate::asm::label_parsers::label_ref;
use crate::asm::register_parsers::register;
use crate::asm::Token;

use nom::digit;
use nom::types::CompleteStr;

named!(pub operand<CompleteStr, Token>,
    alt!(
        real_operand |
        integer_operand |
        label_ref |
        register |
        string
    )
);

named!(integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            sign: opt!(tag!("-")) >>
            integer: digit >>
            (
                {
                    let mut tmp = String::from("");
                    if sign.is_some() {
                        tmp.push_str("-");
                    }
                    tmp.push_str(&integer.to_string());
                    Token::Integer{value: tmp.parse::<i32>().unwrap()}
                }
            )
        )
    )
);

named!(real_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            sign: opt!(tag!("-")) >>
            left:  digit >>
            tag!(".") >>
            right: digit >>
            (
                {
                    let mut tmp = String::from("");
                    if sign.is_some() {
                        tmp.push_str("-");
                    }
                    tmp.push_str(&left.to_string());
                    tmp.push_str(".");
                    tmp.push_str(&right.to_string());
                    Token::Real{value: tmp.parse::<f64>().unwrap()}
                }
            )
        )
    )
);

named!(string<CompleteStr, Token>,
    do_parse!(
        tag!("'") >>
        content: take_until!("'") >>
        tag!("'") >>
        (
            Token::DoString{ value: content.to_string() }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer() {
        let result = integer_operand(CompleteStr("#42"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::Integer { value: 42 });

        let result = integer_operand(CompleteStr("42"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_real() {
        let result = real_operand(CompleteStr("#4.2"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::Real { value: 4.2 });

        let result = real_operand(CompleteStr("4.2"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_operand() {
        let result = operand(CompleteStr("#3.145"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::Real { value: 3.145 });

        let result = operand(CompleteStr("#4"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::Integer { value: 4 });
    }

    #[test]
    fn test_parse_label() {
        let result = operand(CompleteStr("@test"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(
            value,
            Token::LabelRef {
                name: "test".to_string()
            }
        );

        let result = operand(CompleteStr("test"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_string() {
        let result = string(CompleteStr("'hello do'"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(
            value,
            Token::DoString {
                value: "hello do".to_string()
            }
        );

        let result = string(CompleteStr("'invalid"));
        assert!(result.is_err());
    }
}
