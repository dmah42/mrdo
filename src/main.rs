use crate::compiler::Compiler;

pub mod compiler;
pub mod expression_parsers;
pub mod factor_parsers;
pub mod operand_parsers;
pub mod operator_parsers;
pub mod program_parser;
pub mod term_parsers;
pub mod tokens;
pub mod visitor;

fn main() {
    const LISTING: &str = "(3.4 + 1.0) - 2.8";

    let mut compiler = Compiler::new();

    println!("{}", compiler.compile(LISTING));
}
