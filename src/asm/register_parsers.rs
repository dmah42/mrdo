use crate::asm::Token;

use nom::digit;
use nom::types::CompleteStr;

// $i0 for integer register 0
named!(iregister<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$i") >>
            reg_idx: digit >>
            (
                Token::IntRegister{
                    idx: reg_idx.parse::<u8>().unwrap()
                }
            )
        )
    )
);

// $r0 for real register 0
named!(rregister<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$r") >>
            reg_idx: digit >>
            (
                Token::RealRegister{
                    idx: reg_idx.parse::<u8>().unwrap()
                }
            )
        )
    )
);

// $v0 for vector register 0
named!(vregister<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$v") >>
            reg_idx: digit >>
            (
                Token::VectorRegister{
                    idx: reg_idx.parse::<u8>().unwrap()
                }
            )
        )
    )
);

named!(pub register<CompleteStr, Token>,
    do_parse!(
        reg: alt!(
            iregister | rregister | vregister
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
        let result = register(CompleteStr("$i1"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::IntRegister { idx: 1 });

        let result = register(CompleteStr("$r1"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::RealRegister { idx: 1 });

        let result = register(CompleteStr("$v30"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::VectorRegister { idx: 30 });

        let result = register(CompleteStr("0"));
        assert!(result.is_err());

        let result = register(CompleteStr("$a"));
        assert!(result.is_err());
    }
}
