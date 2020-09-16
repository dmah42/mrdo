use crate::compiler::expression_parsers::*;
use crate::compiler::operand_parsers::*;
use crate::compiler::tokens::Token;

use nom::types::CompleteStr;
use nom::*;

named!(pub factor<CompleteStr, Token>,
    ws!(
        do_parse!(
            f: alt!(
                real |
                ws!(delimited!(tag!("("), rvalue, tag!(")"))) |
                ident
            ) >>
            (
                {
                    Token::Factor{value: Box::new(f)}
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor() {
        let result = factor(CompleteStr("(1+2)"));
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        assert_eq!(
            tree,
            Token::Factor {
                value: Box::new(Token::Expression {
                    left: Box::new(Token::Term {
                        left: Box::new(Token::Factor {
                            value: Box::new(Token::Real { value: 1.0 })
                        }),
                        right: vec![]
                    }),
                    right: vec![(
                        Token::AdditionOp,
                        Token::Term {
                            left: Box::new(Token::Factor {
                                value: Box::new(Token::Real { value: 2.0 })
                            }),
                            right: vec![]
                        }
                    )]
                })
            }
        );

        let result = factor(CompleteStr("3.0 + foo"));
        assert!(result.is_ok());
        let (_, tree) = result.unwrap();
        assert_eq!(
            tree,
            Token::Factor {
                value: Box::new(Token::Real { value: 3.0 })
            }
        );
    }
}
