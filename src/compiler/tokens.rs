#[derive(Debug, PartialEq)]
pub enum Token {
    // Arithmetic
    AdditionOp,
    SubtractionOp,
    MultiplicationOp,
    DivisionOp,

    // Comparative
    EqualsOp,
    NotEqualsOp,
    GreaterThanOp,
    GreaterThanEqualsOp,
    LessThanOp,
    LessThanEqualsOp,

    Compare {
        left: Box<Token>,
        op: Box<Token>,
        right: Box<Token>,
    },

    Assign {
        ident: String,
        expr: Box<Token>,
    },

    Builtin {
        builtin: String,
        args: Vec<Token>,
    },

    Identifier {
        name: String,
    },
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
