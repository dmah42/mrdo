use super::{builtin::Builtin, r#type::Type};

#[derive(Debug, PartialEq)]
pub enum Token {
    Comment {
        comment: String,
    },

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

    // Logical
    AndOp,
    OrOp,
    NotOp,

    UnaryOp {
        op: Box<Token>,
        right: Box<Token>,
    },

    BinOp {
        left: Box<Token>,
        op: Box<Token>,
        right: Box<Token>,
    },

    Assign {
        ident: String,
        // TODO: type!
        expr: Box<Token>,
    },

    Builtin {
        builtin: Builtin,
        args: Vec<Token>,
    },

    Function {
        name: String,
        args: Vec<Token>,         // Args
        body: Vec<Option<Token>>, // Expressions
    },

    Arg {
        ident: String,
        typ: Type,
    },

    Identifier {
        name: String,
    },
    Coll {
        values: Vec<Token>,
    },
    Integer {
        value: i32,
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
    Arith {
        left: Box<Token>,
        right: Vec<(Token, Token)>,
    },
    Expression {
        source: String,
        token: Box<Token>,
    },
    Program {
        statements: Vec<Option<Token>>,
    },
}
