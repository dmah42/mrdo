use crate::Token;
use nom::digit;
use nom::types::CompleteStr;

named!(iregister<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_idx: digit >>
            (
                Token::IntRegister{
                    idx: reg_idx.parse::<u8>().unwrap()
                }
            )
        )
    )
);

named!(rregister<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("%") >>
            reg_idx: digit >>
            (
                Token::RealRegister{
                    idx: reg_idx.parse::<u8>().unwrap()
                }
            )
        )
    )
);

named!(pub register<CompleteStr, Token>,
    do_parse!(
        reg: alt!(
            iregister | rregister
        ) >>
        (
            reg
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$1"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::IntRegister { idx: 1 });

        let result = register(CompleteStr("%1"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::RealRegister { idx: 1 });

        let result = register(CompleteStr("0"));
        assert!(result.is_err());

        let result = register(CompleteStr("$a"));
        assert!(result.is_err());

        let result = register(CompleteStr("%a"));
        assert!(result.is_err());
    }
}
