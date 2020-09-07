use crate::Token;
use nom::types::CompleteStr;
use nom::{alphanumeric, multispace};

named!(pub label_decl<CompleteStr, Token>,
    ws!(
        do_parse!(
            name: alphanumeric >>
            tag!(":") >>
            opt!(multispace) >>
            (
                Token::LabelDecl{name: name.to_string()}
            )
        )
    )
);

named!(pub label_ref<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("@") >>
            name: alphanumeric >>
            opt!(multispace) >>
            (
                Token::LabelRef{name: name.to_string()}
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label_decl() {
        let result = label_decl(CompleteStr("test:"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::LabelDecl {
                name: "test".to_string()
            }
        );

        let result = label_decl(CompleteStr("test"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_label_ref() {
        let result = label_ref(CompleteStr("@test"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::LabelRef {
                name: "test".to_string()
            }
        );

        let result = label_ref(CompleteStr("test"));
        assert!(result.is_err());
    }
}
