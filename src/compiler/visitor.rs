use crate::compiler::error::Error;
use crate::compiler::tokens::Token;

pub trait Visitor {
    fn visit_token(&mut self, node: &Token) -> Result<(), Error>;
}
