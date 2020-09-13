use crate::compiler::expression_parsers::expression;
use crate::compiler::tokens::Token;

use nom::types::CompleteStr;
use nom::*;

named!(pub builtin<CompleteStr, Token>,
    ws!(
        do_parse!(
            builtin: alpha >>
            args: ws!(delimited!(tag!("("), separated_list!(tag!(","), expression), tag!(")"))) >>
            (
                {
                    Token::Builtin{ builtin: builtin.to_string(), args }
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin() {
        let result = builtin(CompleteStr("foo()"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Builtin {
                builtin: "foo".to_string(),
                args: vec![]
            }
        );

        let result = builtin(CompleteStr("write(42.0)"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Builtin {
                builtin: "write".to_string(),
                args: vec![Token::Expression {
                    left: Box::new(Token::Term {
                        left: Box::new(Token::Factor {
                            value: Box::new(Token::Real { value: 42.0 })
                        }),
                        right: vec![],
                    }),
                    right: vec![],
                }]
            }
        );

        let result = builtin(CompleteStr("read(foo, 1.0)"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Builtin {
                builtin: "read".to_string(),
                args: vec![
                    Token::Expression {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Identifier {
                                    name: "foo".to_string()
                                })
                            }),
                            right: vec![],
                        }),
                        right: vec![],
                    },
                    Token::Expression {
                        left: Box::new(Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Real { value: 1.0 })
                            }),
                            right: vec![],
                        }),
                        right: vec![],
                    }
                ]
            }
        );
    }
}
