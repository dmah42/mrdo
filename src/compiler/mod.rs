use crate::compiler::error::Error;
use crate::compiler::expression_parsers::expression;
use crate::compiler::program_parser::program;
use crate::compiler::tokens::Token;
use crate::compiler::visitor::Visitor;

use indexmap::IndexSet;
use nom::types::CompleteStr;
use std::collections::HashMap;

pub mod builtin_parsers;
pub mod error;
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
    variables: HashMap<String, u8>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            free_reg: (0..32).collect(),
            used_reg: IndexSet::new(),
            assembly: vec![],
            variables: HashMap::new(),
        }
    }

    pub fn compile(&mut self, source: &str) -> Result<String, Error> {
        self.assembly.clear();
        let (_, tree) = program(CompleteStr(source)).unwrap();
        self.visit_token(&tree)?;
        Ok(self.assembly.join("\n"))
    }

    // NOTE: public for the repl
    pub fn compile_expr(&mut self, source: &str) -> Result<&Vec<String>, Error> {
        self.assembly.clear();
        let (_, tree) = expression(CompleteStr(source)).unwrap();
        self.visit_token(&tree)?;
        Ok(&self.assembly)
    }
}

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) -> Result<(), Error> {
        match node {
            Token::Comment { comment: _ } => {
                //println!("Skipping comment '{}'", comment);
            }

            // Arithmetic
            Token::AdditionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly.push(format!(
                    "add $r{} $r{} $r{}",
                    result_reg, left_reg, right_reg
                ));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::SubtractionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly.push(format!(
                    "sub $r{} $r{} $r{}",
                    result_reg, left_reg, right_reg
                ));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::MultiplicationOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly.push(format!(
                    "mul $r{} $r{} $r{}",
                    result_reg, left_reg, right_reg
                ));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::DivisionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly.push(format!(
                    "div $r{} $r{} $r{}",
                    result_reg, left_reg, right_reg
                ));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }

            // Comparative
            Token::EqualsOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly.push(format!(
                    "eq $r{} $r{} $r{}",
                    result_reg, left_reg, right_reg
                ));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::NotEqualsOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly.push(format!(
                    "neq $r{} $r{} $r{}",
                    result_reg, left_reg, right_reg
                ));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::GreaterThanOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly.push(format!(
                    "gt $r{} $r{} $r{}",
                    result_reg, left_reg, right_reg
                ));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::GreaterThanEqualsOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly.push(format!(
                    "gte $r{} $r{} $r{}",
                    result_reg, left_reg, right_reg
                ));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::LessThanOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly.push(format!(
                    "lt $r{} $r{} $r{}",
                    result_reg, left_reg, right_reg
                ));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::LessThanEqualsOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly.push(format!(
                    "lte $r{} $r{} $r{}",
                    result_reg, left_reg, right_reg
                ));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }

            Token::Compare { left, op, right } => {
                self.visit_token(left)?;
                self.visit_token(right)?;
                self.visit_token(op)?;
            }

            Token::Assign { ident, expr } => {
                // First visit the rhs to make sure we do what we need
                // to find the value we need.
                self.visit_token(expr)?;
                let result_reg = self.used_reg.pop().unwrap();

                // TODO: Allocate storage for the identifier and write value to heap.
                // TODO: Map identifier name to heap offset.
                // TODO: create a 'load from heap' operation in assembly.

                // Temporary: ensure result reg remains 'used' and map name to result reg.
                // 'unassign' old result reg if the variable already exists.
                if self.variables.contains_key(ident) {
                    self.free_reg.insert(self.variables[ident]);
                }

                self.used_reg.insert(result_reg);
                self.variables.insert(ident.to_string(), result_reg);

                println!("inserted variable '{}'", ident.to_string());
            }

            Token::Builtin { builtin, args } => {
                match builtin.as_str() {
                    "write" => {
                        for expr in args {
                            self.visit_token(expr)?;
                        }
                        let reg = self.used_reg.pop().unwrap();
                        // TODO: create a "syscall" instruction in assembly and a syscall
                        // to print from an address, then use that to print out whatever is
                        // being passed in to write.
                        self.assembly.push(format!("somestr: .str 'reg ${}'", reg));
                        self.assembly.push("print @somestr".to_string());
                    }
                    _ => return Err(Error::new(format!("Unknown builtin: {}", builtin))),
                };
            }

            Token::Identifier { name } => {
                println!("referencing variable '{}'", name.to_string());
                if !self.variables.contains_key(name) {
                    return Err(Error::new(format!("Unknown variable '{}'", name)));
                }

                // This adds the current variables register to zero to get the value into a
                // new register ready to be referenced in whatever binary ops are expected.
                let zero_reg = self.free_reg.pop().unwrap();
                self.assembly.push(format!("load $r{} #0", zero_reg));

                let next_reg = self.free_reg.pop().unwrap();
                self.assembly.push(format!(
                    "add $r{} $r{} $r{}",
                    next_reg, zero_reg, self.variables[name]
                ));

                self.used_reg.insert(next_reg);
                self.free_reg.insert(zero_reg);
            }

            // TODO: allow for 'Integer' types maybe.
            Token::Real { value } => {
                let next_reg = self.free_reg.pop().unwrap();
                self.assembly
                    .push(format!("load $r{} #{:.2}", next_reg, value));
                self.used_reg.insert(next_reg);
            }
            Token::Coll { values } => {
                // Allocate memory for the heap and put the base address into a register.
                let alloc_reg = self.free_reg.pop().unwrap();
                self.assembly
                    .push(format!("alloc $i{} #{}", alloc_reg, values.len() * 8));

                // Go through the collection and store each generated real to the heap.
                let vec_reg = self.free_reg.pop().unwrap();
                self.assembly.push(format!("load $i{} #0", vec_reg));
                self.assembly
                    .push(format!("add $i{} $i{} $i{}", vec_reg, vec_reg, alloc_reg));
                for v in values {
                    // Note: this assumes visiting a token ends up with a used reg
                    // equivalent to a real.
                    self.visit_token(v)?;
                    let used_reg = self.used_reg.pop().unwrap();
                    self.assembly
                        .push(format!("sw $i{} $r{}", vec_reg, used_reg));
                    self.free_reg.insert(used_reg);
                    // TODO: skip this last add.
                    self.assembly.push(format!("add $i{} #8", vec_reg));
                }
                self.free_reg.insert(vec_reg);

                // And finally load the heap info into a register.
                let next_reg = self.free_reg.pop().unwrap();
                self.assembly.push(format!(
                    "load $v{} $i{} #{}",
                    next_reg,
                    alloc_reg,
                    values.len() * 8
                ));
                self.free_reg.insert(alloc_reg);
                self.used_reg.insert(next_reg);
            }

            Token::Factor { ref value } => self.visit_token(value)?,
            Token::Term {
                ref left,
                ref right,
            } => {
                self.visit_token(left)?;
                for factor in right {
                    self.visit_token(&factor.1)?;
                    self.visit_token(&factor.0)?;
                }
            }
            Token::Expression {
                ref left,
                ref right,
            } => {
                self.visit_token(left)?;
                for term in right {
                    self.visit_token(&term.1)?;
                    self.visit_token(&term.0)?;
                }
            }
            Token::Program { ref expressions } => {
                self.assembly.push(".data".into());
                self.assembly.push(".code".into());
                for expr in expressions {
                    println!(".. visiting {:?}", expr);
                    self.visit_token(expr)?;
                }
                self.assembly.push("halt".into());
            }
        }
        Ok(())
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
    fn test_collection() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("[0, 1.2]");
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "alloc $i31 #16",
                "load $i30 #0",
                "add $i30 $i30 $i31",
                "load $r29 #0.00",
                "sw $i30 $r29",
                "add $i30 #8",
                "load $r29 #1.20",
                "sw $i30 $r29",
                "add $i30 #8",
                "load $v30 $i31 #16",
                "halt"
            ],
        );
        let mut expected_free: IndexSet<u8> = (0..30).collect();
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {30});
    }

    #[test]
    fn test_addition() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 + 3.4");
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "add $r29 $r31 $r30",
                "halt"
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
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "sub $r29 $r31 $r30",
                "halt"
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
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "mul $r29 $r31 $r30",
                "halt"
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
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "div $r29 $r31 $r30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_equals() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 + 4.1 eq 3.4");
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load $r31 #1.20",
                "load $r30 #4.10",
                "add $r29 $r31 $r30",
                "load $r30 #3.40",
                "eq $r31 $r29 $r30",
                "halt"
            ]
        );
        let expected_free: IndexSet<u8> = (0..31).collect();
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {31});
    }

    #[test]
    fn test_not_equals() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 neq 3.4");
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "neq $r29 $r31 $r30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_greater_than() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 gt 3.4");
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "gt $r29 $r31 $r30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_greater_than_equals() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 gte 3.4");
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "gte $r29 $r31 $r30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_less_than() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 lt 3.4");
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "lt $r29 $r31 $r30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_less_than_equals() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 lte 3.4");
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "lte $r29 $r31 $r30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }
}
