use crate::compiler::expression_parsers::*;
use crate::compiler::tokens::Token;

use nom::types::CompleteStr;
use nom::*;

// TODO: extend real to be "operand" with multiple types. requres rules to
// be set for operators (arithmetic and logical) for coll vs coll and coll vs real
// (see readme)
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

named!(pub ident<CompleteStr, Token>,
    ws!(
        do_parse!(
            name: alpha >>
            (
                Token::Identifier{ name: name.to_string() }
            )
        )
    )
);

named!(pub coll<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("[") >>
            values: separated_list!(tag!(","), rvalue) >>
            tag!("]") >>
            (
                Token::Coll { values }
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

    #[test]
    fn test_ident() {
        let result = ident(CompleteStr("foo"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Identifier {
                name: "foo".to_string()
            }
        );
    }

    #[test]
    fn test_coll() {
        let result = coll(CompleteStr("[]"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().1, Token::Coll { values: vec![] });

        let result = coll(CompleteStr("[3+4, 42, foo]"));
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1,
            Token::Coll {
                values: vec![
                    Token::Expression {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Real { value: 3.0 })
                            }),
                            right: vec![]
                        }),
                        right: vec![(
                            Token::AdditionOp,
                            Token::Term {
                                left: Box::new(Token::Factor {
                                    value: Box::new(Token::Real { value: 4.0 })
                                }),
                                right: vec![],
                            },
                        )],
                    },
                    Token::Expression {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Real { value: 42.0 })
                            }),
                            right: vec![],
                        }),
                        right: vec![],
                    },
                    Token::Expression {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Identifier {
                                    name: "foo".to_string()
                                }),
                            }),
                            right: vec![],
                        }),
                        right: vec![],
                    },
                ]
            }
        );
    }
}
