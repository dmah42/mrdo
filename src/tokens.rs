#[derive(Debug, PartialEq)]
pub enum Token {
    AdditionOp,
    SubtractionOp,
    MultiplicationOp,
    DivisionOp,
    Real {
        value: f64,
    },
    Expression {
        left: Box<Token>,
        op: Box<Token>,
        right: Box<Token>,
    },
    Program {
        expressions: Vec<Token>,
    },
}
