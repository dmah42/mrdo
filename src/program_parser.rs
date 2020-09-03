use crate::expression_parsers::expression;
use crate::tokens::Token;
use nom::types::CompleteStr;
use nom::*;

named!(pub program<CompleteStr, Token>,
    ws!(
        do_parse!(
            expressions: many1!(expression) >>
            (
                // TODO: expand to statements, etc.
                Token::Program {
                    expressions
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let result = program(CompleteStr("1.2 + 0.3\n2.4 * 4.0"));
        assert!(result.is_ok());

        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Program {
                expressions: vec![
                    Token::Expression {
                        left: Box::new(Token::Real { value: 1.2 }),
                        op: Box::new(Token::AdditionOp),
                        right: Box::new(Token::Real { value: 0.3 }),
                    },
                    Token::Expression {
                        left: Box::new(Token::Real { value: 2.4 }),
                        op: Box::new(Token::MultiplicationOp),
                        right: Box::new(Token::Real { value: 4.0 }),
                    }
                ]
            }
        );
    }
}
