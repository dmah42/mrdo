use crate::compiler::program_parser::program;
use crate::compiler::tokens::Token;
use crate::compiler::visitor::Visitor;

use indexmap::IndexSet;
use nom::types::CompleteStr;

pub mod expression_parsers;
pub mod factor_parsers;
pub mod operand_parsers;
pub mod operator_parsers;
pub mod program_parser;
pub mod term_parsers;
pub mod tokens;
pub mod visitor;

pub struct Compiler {
    free_reg: IndexSet<u8>,
    used_reg: IndexSet<u8>,
    assembly: Vec<String>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            free_reg: (0..32).collect(),
            used_reg: IndexSet::new(),
            assembly: vec![],
        }
    }

    pub fn compile(&mut self, source: &str) -> String {
        let (_, tree) = program(CompleteStr(source)).unwrap();
        self.visit_token(&tree);
        self.assembly.join("\n")
    }
}

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) {
        match node {
            Token::AdditionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("add %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::SubtractionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("sub %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::MultiplicationOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("mul %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::DivisionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("div %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::Real { value } => {
                let next_reg = self.free_reg.pop().unwrap();
                let line = format!("load %{} #{:.2}", next_reg, value);
                self.assembly.push(line);
                self.used_reg.insert(next_reg);
            }
            Token::Factor { ref value } => self.visit_token(value),
            Token::Term {
                ref left,
                ref right,
            } => {
                self.visit_token(left);
                for factor in right {
                    self.visit_token(&factor.1);
                    self.visit_token(&factor.0);
                }
            }
            Token::Expression {
                ref left,
                ref right,
            } => {
                self.visit_token(left);
                for term in right {
                    self.visit_token(&term.1);
                    self.visit_token(&term.0);
                }
            }
            Token::Program { ref expressions } => {
                self.assembly.push(".data".into());
                self.assembly.push(".code".into());
                for expr in expressions {
                    self.visit_token(expr);
                }
                self.assembly.push("halt".into());
            }
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indexmap::indexset;

    fn generate_test_program(listing: &str) -> Token {
        let (_, tree) = program(CompleteStr(listing)).unwrap();
        tree
    }

    #[test]
    fn test_addition() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 + 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.2",
                "load %30 #3.4",
                "add %29 %31 %30"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_subtraction() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 - 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.2",
                "load %30 #3.4",
                "sub %29 %31 %30"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_multiplication() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 * 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.2",
                "load %30 #3.4",
                "mul %29 %31 %30"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_division() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 / 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.2",
                "load %30 #3.4",
                "div %29 %31 %30"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }
}
