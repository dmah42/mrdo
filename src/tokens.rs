#[derive(Debug, PartialEq)]
pub enum Token {
    AdditionOp,
    SubtractionOp,
    MultiplicationOp,
    DivisionOp,
    Real {
        value: f64,
    },
    Factor {
        value: Box<Token>,
    },
    Term {
        left: Box<Token>,
        right: Vec<(Token, Token)>,
    },
    Expression {
        left: Box<Token>,
        right: Vec<(Token, Token)>,
    },
    Program {
        expressions: Vec<Token>,
    },
}
