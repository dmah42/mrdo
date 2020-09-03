use crate::tokens::Token;
use crate::visitor::Visitor;

use indexmap::IndexSet;

pub struct Compiler {
    free_reg: IndexSet<u8>,
    used_reg: IndexSet<u8>,
    assembled: Vec<String>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            free_reg: (0..32).collect(),
            used_reg: IndexSet::new(),
            assembled: vec![],
        }
    }
}

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) {
        match node {
            Token::AdditionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                self.assembled
                    .push(format!("add ${} ${} ${}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::SubtractionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                self.assembled
                    .push(format!("sub ${} ${} ${}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::MultiplicationOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                self.assembled
                    .push(format!("mul ${} ${} ${}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::DivisionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                self.assembled
                    .push(format!("div ${} ${} ${}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::Real { value } => {
                let next_reg = self.free_reg.pop().unwrap();
                let line = format!("load ${} #{}", next_reg, value);
                self.assembled.push(line);
                self.used_reg.insert(next_reg);
            }
            Token::Expression {
                ref left,
                ref op,
                ref right,
            } => {
                self.visit_token(left);
                self.visit_token(right);
                self.visit_token(op);
            }
            Token::Program { ref expressions } => {
                for expr in expressions {
                    self.visit_token(expr);
                }
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

    use crate::program_parser::program;
    use indexmap::indexset;
    use nom::types::CompleteStr;

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
            compiler.assembled,
            vec!["load $31 #1.2", "load $30 #3.4", "add $29 $30 $31"]
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
            compiler.assembled,
            vec!["load $31 #1.2", "load $30 #3.4", "sub $29 $30 $31"]
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
            compiler.assembled,
            vec!["load $31 #1.2", "load $30 #3.4", "mul $29 $30 $31"]
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
            compiler.assembled,
            vec!["load $31 #1.2", "load $30 #3.4", "div $29 $30 $31"]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }
}
