use crate::compiler::error::Error;
use crate::compiler::expression_parsers::expression;
use crate::compiler::program_parser::program;
use crate::compiler::tokens::Token;
use crate::compiler::visitor::Visitor;
use crate::vm::register::Register;

use nom::types::CompleteStr;
use std::collections::HashMap;
use std::mem::size_of;

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
    free_int_reg: Vec<Register>,
    free_real_reg: Vec<Register>,
    free_vec_reg: Vec<Register>,
    used_reg: Vec<(u8, Register)>,
    assembly: Vec<String>,

    // Maps from a name to an index into `used_reg`.
    variables: HashMap<String, usize>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            free_int_reg: (0..32).map(|_| Register::I(0)).collect(),
            free_real_reg: (0..32).map(|_| Register::R(0.0)).collect(),
            free_vec_reg: (0..32).map(|_| Register::V(vec![])).collect(),
            used_reg: vec![],
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

    fn next_free_int_reg(&mut self) -> (u8, Register) {
        (
            self.free_int_reg.len() as u8 - 1,
            self.free_int_reg.pop().unwrap(),
        )
    }

    fn next_free_real_reg(&mut self) -> (u8, Register) {
        (
            self.free_real_reg.len() as u8 - 1,
            self.free_real_reg.pop().unwrap(),
        )
    }

    fn next_free_vec_reg(&mut self) -> (u8, Register) {
        (
            self.free_vec_reg.len() as u8 - 1,
            self.free_vec_reg.pop().unwrap(),
        )
    }

    fn push_free_reg(&mut self, reg: Register) {
        match reg {
            Register::I(_) => self.free_int_reg.push(reg),
            Register::R(_) => self.free_real_reg.push(reg),
            Register::V(_) => self.free_vec_reg.push(reg),
        };
    }

    fn get_binop_result_reg(&mut self, left: &Register, right: &Register) -> (u8, Register) {
        match left {
            Register::I(_) => match right {
                Register::I(_) => self.next_free_int_reg(),
                Register::R(_) => self.next_free_real_reg(),
                Register::V(_) => self.next_free_vec_reg(),
            },
            Register::R(_) => match right {
                // Promote to a real register.
                Register::I(_) => self.next_free_real_reg(),
                Register::R(_) => self.next_free_real_reg(),
                Register::V(_) => self.next_free_vec_reg(),
            },
            Register::V(_) => self.next_free_vec_reg(),
        }
    }

    fn add_arith_instruction(&mut self, op: &str) {
        let (right_idx, right_reg) = self.used_reg.pop().unwrap();
        let (left_idx, left_reg) = self.used_reg.pop().unwrap();

        let result_reg = self.get_binop_result_reg(&left_reg, &right_reg);

        let result_char = result_reg.1.get_char();
        let left_char = left_reg.get_char();
        let right_char = right_reg.get_char();

        self.assembly.push(format!(
            "{} ${}{} ${}{} ${}{}",
            op, result_char, result_reg.0, left_char, left_idx, right_char, right_idx
        ));

        self.used_reg.push(result_reg);

        self.push_free_reg(left_reg);
        self.push_free_reg(right_reg);
    }

    fn add_compare_instruction(&mut self, op: &str) {
        let (right_idx, right_reg) = self.used_reg.pop().unwrap();
        let (left_idx, left_reg) = self.used_reg.pop().unwrap();

        let result_reg = self.next_free_int_reg();

        let result_char = result_reg.1.get_char();
        let left_char = left_reg.get_char();
        let right_char = right_reg.get_char();

        self.assembly.push(format!(
            "{} ${}{} ${}{} ${}{}",
            op, result_char, result_reg.0, left_char, left_idx, right_char, right_idx
        ));

        self.used_reg.push(result_reg);

        self.push_free_reg(left_reg);
        self.push_free_reg(right_reg);
    }
}

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) -> Result<(), Error> {
        //println!(".. visiting {:?}", node);
        // println!("  [before] {:?}\n    {:?}", self.variables, self.used_reg);
        match node {
            Token::Comment { comment: _ } => {
                //println!("Skipping comment '{}'", comment);
            }

            // Arithmetic
            Token::AdditionOp => self.add_arith_instruction("add"),
            Token::SubtractionOp => self.add_arith_instruction("sub"),
            Token::MultiplicationOp => self.add_arith_instruction("mul"),
            Token::DivisionOp => self.add_arith_instruction("div"),

            // Comparative
            Token::EqualsOp => self.add_compare_instruction("eq"),
            Token::NotEqualsOp => self.add_compare_instruction("neq"),
            Token::GreaterThanOp => self.add_compare_instruction("gt"),
            Token::GreaterThanEqualsOp => self.add_compare_instruction("gte"),
            Token::LessThanOp => self.add_compare_instruction("lt"),
            Token::LessThanEqualsOp => self.add_compare_instruction("lte"),

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
                    let old_used_reg = self.used_reg.remove(self.variables[ident]);
                    // TODO: check that result reg is the same type as the existing variable.
                    match old_used_reg.1 {
                        Register::I(_) => self.free_int_reg.push(old_used_reg.1),
                        Register::R(_) => self.free_real_reg.push(old_used_reg.1),
                        Register::V(_) => self.free_vec_reg.push(old_used_reg.1),
                    }
                }

                self.variables
                    .insert(ident.to_string(), self.used_reg.len());

                println!(
                    "inserted variable '{}': {:?}",
                    ident.to_string(),
                    &result_reg,
                );

                self.used_reg.push(result_reg);
            }

            Token::Builtin { builtin, args } => {
                match builtin.as_str() {
                    "write" => {
                        /*                         for expr in args {
                            self.visit_token(expr)?;
                        }
                        let reg = self.used_reg.pop().unwrap();
                        // TODO: create a "syscall" instruction in assembly and a syscall
                        // to print from an address, then use that to print out whatever is
                        // being passed in to write.
                        // TODO: add the string to rodata.
                        self.assembly.push(format!("somestr: .str 'reg ${}'", reg));
                        self.assembly.push("print @somestr".to_string()); */
                    }
                    _ => return Err(Error::new(format!("Unknown builtin: {}", builtin))),
                };
            }

            Token::Identifier { name } => {
                println!("referencing variable '{}'", name.to_string());
                if !self.variables.contains_key(name) {
                    return Err(Error::new(format!("Unknown variable '{}'", name)));
                }

                let index = self.variables[name];

                println!(".. found at {}", index);

                let copy_reg = match &self.used_reg[index].1 {
                    Register::I(_) => self.next_free_int_reg(),
                    Register::R(_) => self.next_free_real_reg(),
                    Register::V(_) => self.next_free_vec_reg(),
                };

                // Copy the value of the current identifier into the new reg
                // by adding it to zero.
                let zero_reg = self.next_free_int_reg();
                self.assembly.push(format!("load $i{} #0", zero_reg.0));
                self.assembly.push(format!(
                    "add ${}{} $i{} ${}{}",
                    copy_reg.1.get_char(),
                    copy_reg.0,
                    zero_reg.0,
                    self.used_reg[index].1.get_char(),
                    self.used_reg[index].0
                ));

                self.used_reg.push(copy_reg);
                self.free_int_reg.push(zero_reg.1);
            }

            Token::Real { value } => {
                let next_reg = self.next_free_real_reg();
                self.assembly
                    .push(format!("load $r{} #{:.2}", next_reg.0, value));
                self.used_reg.push(next_reg);
            }
            Token::Coll { values } => {
                // Allocate memory for the heap and put the base address into a register.
                let alloc_reg = self.next_free_int_reg();
                self.assembly
                    .push(format!("alloc $i{} #{}", alloc_reg.0, values.len() * 8));

                // Go through the collection and store each generated real to the heap.
                let vec_base_reg = self.next_free_int_reg();
                self.assembly.push(format!("load $i{} #0", vec_base_reg.0));
                self.assembly.push(format!(
                    "add $i{} $i{} $i{}",
                    vec_base_reg.0, vec_base_reg.0, alloc_reg.0
                ));
                for v in values {
                    // Note: this assumes visiting a token ends up with a used reg
                    // equivalent to a real.
                    self.visit_token(v)?;
                    let used_reg = self.used_reg.pop().unwrap();
                    match used_reg.1 {
                        Register::R(_) => {}
                        _ => {
                            return Err(Error::new(
                                "Unable to put non-real into a vector".to_string(),
                            ));
                        }
                    };
                    self.assembly
                        .push(format!("sw $i{} $r{}", vec_base_reg.0, used_reg.0));
                    self.free_real_reg.push(used_reg.1);
                    // TODO: skip this last add on the last iteration.
                    self.assembly
                        .push(format!("add $i{} #{}", vec_base_reg.0, size_of::<f64>()));
                }
                self.free_int_reg.push(vec_base_reg.1);

                // And finally load the heap info into a vector register.
                let vec_reg = self.next_free_vec_reg();
                self.assembly.push(format!(
                    "load $v{} $i{} #{}",
                    vec_reg.0,
                    alloc_reg.0,
                    values.len() * size_of::<f64>()
                ));
                self.free_int_reg.push(alloc_reg.1);
                self.used_reg.push(vec_reg);
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
                    self.visit_token(expr)?;
                }
                self.assembly.push("halt".into());
            }
        };
        //println!("  [after] {:?}\n    {:?}", self.variables, self.used_reg);
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

    fn generate_test_program(listing: &str) -> Token {
        let (_, tree) = program(CompleteStr(listing)).unwrap();
        tree
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
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 31);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![(29, Register::R(0.0))]);

        // TODO: test integer and collection addition.
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
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 31);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![(29, Register::R(0.0))]);
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
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 31);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![(29, Register::R(0.0))]);
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
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 31);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![(29, Register::R(0.0))]);
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
                "eq $i31 $r29 $r30",
                "halt"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![(31, Register::I(0))]);
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
                "neq $i31 $r31 $r30",
                "halt"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![(31, Register::I(0))]);
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
                "gt $i31 $r31 $r30",
                "halt"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![(31, Register::I(0))]);
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
                "gte $i31 $r31 $r30",
                "halt"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![(31, Register::I(0))]);
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
                "lt $i31 $r31 $r30",
                "halt"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![(31, Register::I(0))]);
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
                "lte $i31 $r31 $r30",
                "halt"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![(31, Register::I(0))]);
    }

    #[test]
    fn test_assign() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("foo = 42.0");
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![".data", ".code", "load $r31 #42.00", "halt"]
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 31);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![(31, Register::R(0.0))]);
        assert_eq!(
            compiler.variables,
            [("foo".to_string(), 0 as usize)].iter().cloned().collect()
        );
    }

    #[test]
    fn test_identifier() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("foo = 42.0\nbar = foo");
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load $r31 #42.00",
                "load $i31 #0",
                "add $r30 $i31 $r31",
                "halt"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 30);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![(31, Register::R(0.0)), (30, Register::R(0.0))]
        );
        assert_eq!(
            compiler.variables,
            [
                ("foo".to_string(), 0 as usize),
                ("bar".to_string(), 1 as usize)
            ]
            .iter()
            .cloned()
            .collect()
        );
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
                "load $r31 #0.00",
                "sw $i30 $r31",
                "add $i30 #8",
                "load $r31 #1.20",
                "sw $i30 $r31",
                "add $i30 #8",
                "load $v31 $i31 #16",
                "halt"
            ],
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 31);
        assert_eq!(compiler.used_reg, vec![(31, Register::V(vec![]))]);
    }
}
