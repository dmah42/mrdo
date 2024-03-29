use crate::asm::syscalls::Syscall;
use crate::compiler::{
    builtin::Builtin, error::Error, expression_parsers::expression, program_parser::program,
    tokens::Token, visitor::Visitor,
};
use crate::vm::register::Register as VmRegister;

use std::collections::HashMap;
use std::mem::size_of;

use self::r#type::Type;

mod builtin;
mod builtin_parsers;
mod error;
mod expression_parsers;
mod factor_parsers;
mod function_parser;
mod operand_parsers;
mod operator_parsers;
mod program_parser;
mod term_parsers;
mod tokens;
mod r#type;
mod visitor;

#[derive(Debug, PartialEq)]
struct Register {
    idx: u8,
    reg: VmRegister,
}

impl Register {
    pub fn get_char(&self) -> char {
        match self.reg {
            VmRegister::I(_) => 'i',
            VmRegister::R(_) => 'r',
            VmRegister::V(_) => 'v',
        }
    }
}

#[derive(Debug)]
pub struct Compiler {
    free_int_reg: Vec<Register>,
    free_real_reg: Vec<Register>,
    free_vec_reg: Vec<Register>,
    used_reg: Vec<Register>,
    rodata: Vec<String>,
    assembly: Vec<String>,

    // Maps from a name to an index into `used_reg`.
    variables: HashMap<String, usize>,
    local_variables: Vec<String>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            free_int_reg: (0..32)
                .map(|i| Register {
                    idx: i,
                    reg: VmRegister::I(0),
                })
                .collect(),
            free_real_reg: (0..32)
                .map(|i| Register {
                    idx: i,
                    reg: VmRegister::R(0.0),
                })
                .collect(),
            free_vec_reg: (0..32)
                .map(|i| Register {
                    idx: i,
                    reg: VmRegister::V(vec![]),
                })
                .collect(),
            used_reg: vec![],
            rodata: vec![],
            assembly: vec![],
            variables: HashMap::new(),
            local_variables: vec![],
        }
    }

    pub fn compile(&mut self, source: &str) -> Result<String, Error> {
        self.assembly.clear();
        let (_, tree) = program(source).unwrap();
        self.visit_token(&tree)?;
        Ok([self.rodata.join("\n"), self.assembly.join("\n")].join("\n"))
    }

    // NOTE: public for the repl
    pub fn compile_expr(&mut self, source: &str) -> Result<&Vec<String>, Error> {
        self.assembly.clear();
        let (_, tree) = expression(source).map_err(|e| Error::new(e.to_string()))?;
        if let Some(valid_tree) = tree {
            self.visit_token(&valid_tree)?;
        }
        Ok(&self.assembly)
    }

    fn integrity_check(&self) {
        for used_reg in &self.used_reg {
            let free_reg = match used_reg.reg {
                VmRegister::I(_) => &self.free_int_reg,
                VmRegister::R(_) => &self.free_real_reg,
                VmRegister::V(_) => &self.free_vec_reg,
            };
            if free_reg.contains(used_reg) {
                panic!("Integrity check failed");
            }
        }
    }

    fn push_free_reg(&mut self, reg: Register) {
        match reg.reg {
            VmRegister::I(_) => self.free_int_reg.push(reg),
            VmRegister::R(_) => self.free_real_reg.push(reg),
            VmRegister::V(_) => self.free_vec_reg.push(reg),
        };
    }

    fn get_binop_result_reg(&mut self, left: &Register, right: &Register) -> Register {
        match left.reg {
            VmRegister::I(_) => match right.reg {
                VmRegister::I(_) => self.free_int_reg.pop().unwrap(),
                VmRegister::R(_) => self.free_real_reg.pop().unwrap(),
                VmRegister::V(_) => self.free_vec_reg.pop().unwrap(),
            },
            VmRegister::R(_) => match right.reg {
                // Promote to a real register.
                VmRegister::I(_) => self.free_real_reg.pop().unwrap(),
                VmRegister::R(_) => self.free_real_reg.pop().unwrap(),
                VmRegister::V(_) => self.free_vec_reg.pop().unwrap(),
            },
            VmRegister::V(_) => self.free_vec_reg.pop().unwrap(),
        }
    }

    fn add_arith_instruction(&mut self, op: &str) {
        let right_reg = self.used_reg.pop().unwrap();
        let left_reg = self.used_reg.pop().unwrap();

        let result_reg = self.get_binop_result_reg(&left_reg, &right_reg);

        let result_char = result_reg.get_char();
        let left_char = left_reg.get_char();
        let right_char = right_reg.get_char();

        self.assembly.push(format!(
            "{} ${}{} ${}{} ${}{}",
            op, result_char, result_reg.idx, left_char, left_reg.idx, right_char, right_reg.idx
        ));

        self.used_reg.push(result_reg);

        self.push_free_reg(left_reg);
        self.push_free_reg(right_reg);
    }

    fn add_compare_instruction(&mut self, op: &str) {
        let right_reg = self.used_reg.pop().unwrap();
        let left_reg = self.used_reg.pop().unwrap();

        let result_reg = self.free_int_reg.pop().unwrap();

        let result_char = result_reg.get_char();
        let left_char = left_reg.get_char();
        let right_char = right_reg.get_char();

        self.assembly.push(format!(
            "{} ${}{} ${}{} ${}{}",
            op, result_char, result_reg.idx, left_char, left_reg.idx, right_char, right_reg.idx
        ));

        self.used_reg.push(result_reg);

        self.push_free_reg(left_reg);
        self.push_free_reg(right_reg);
    }

    fn add_logical_instruction(&mut self, op: &str) {
        if op == "not" {
            let right_reg = self.used_reg.pop().unwrap();
            let result_reg = self.free_int_reg.pop().unwrap();

            let result_char = result_reg.get_char();
            let right_char = right_reg.get_char();

            self.assembly.push(format!(
                "{} ${}{} ${}{}",
                op, result_char, result_reg.idx, right_char, right_reg.idx
            ));

            self.used_reg.push(result_reg);
            self.push_free_reg(right_reg);
        } else {
            let right_reg = self.used_reg.pop().unwrap();
            let left_reg = self.used_reg.pop().unwrap();
            let result_reg = self.free_int_reg.pop().unwrap();

            let result_char = result_reg.get_char();
            let left_char = left_reg.get_char();
            let right_char = right_reg.get_char();

            self.assembly.push(format!(
                "{} ${}{} ${}{} ${}{}",
                op, result_char, result_reg.idx, left_char, left_reg.idx, right_char, right_reg.idx
            ));

            self.used_reg.push(result_reg);
            self.push_free_reg(left_reg);
            self.push_free_reg(right_reg);
        }
    }
}

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) -> Result<(), Error> {
        // println!(".. visiting {:?}", node);
        // println!("  [before] {:?}\t    {:?}", self.variables, self.used_reg);
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

            // Logical
            Token::AndOp => self.add_logical_instruction("and"),
            Token::OrOp => self.add_logical_instruction("or"),
            Token::NotOp => self.add_logical_instruction("not"),

            Token::UnaryOp { op, right } => {
                self.visit_token(right)?;
                self.visit_token(op)?;
            }

            Token::BinOp { left, op, right } => {
                self.visit_token(left)?;
                self.visit_token(right)?;
                self.visit_token(op)?;
            }

            Token::Assign { ident, expr } => {
                // First visit the rhs to make sure we do what we need
                // to find the value we need.
                self.visit_token(expr)?;
                let result_reg = self.used_reg.pop().unwrap();

                if !self.local_variables.contains(ident) {
                    if self.variables.contains_key(ident) {
                        // do not allow shadowing.  any variables needed must be passed in.
                        return Err(Error::new(format!(
                            "Cannot assign to global variable '{}' in function scope.",
                            ident
                        )));
                    }

                    // add new variables to list so they can be cleared on function definition exit
                    self.local_variables.push(ident.clone());
                }

                // Ensure result reg remains 'used' and map name to result reg.
                // 'unassign' old result reg if the variable already exists.
                if self.variables.contains_key(ident) {
                    let old_used_reg = self.used_reg.remove(self.variables[ident]);
                    if result_reg.reg != old_used_reg.reg {
                        return Err(Error::new(format!(
                            "Variable '{}' was {:?} and is now {:?}",
                            ident, old_used_reg.reg, result_reg.reg
                        )));
                    }
                    self.push_free_reg(old_used_reg);
                }

                self.variables
                    .insert(ident.to_string(), self.used_reg.len());

                /*                 println!(
                    "inserted variable '{}': {:?}",
                    ident.to_string(),
                    &result_reg,
                ); */

                self.used_reg.push(result_reg);
            }

            Token::Builtin { builtin, args } => {
                match builtin {
                    Builtin::Write => {
                        if args.len() != 1 {
                            return Err(Error::new(
                                "'write' expects a single argument".to_string(),
                            ));
                        }
                        self.visit_token(&args[0])?;
                        let reg = self.used_reg.pop().unwrap();
                        let call_reg = self.free_int_reg.pop().unwrap();

                        self.assembly.push(format!(
                            "load $i{} #{}",
                            call_reg.idx,
                            Syscall::PrintReg as u8
                        ));

                        self.assembly.push(format!(
                            "syscall $i{} ${}{}",
                            call_reg.idx,
                            reg.get_char(),
                            reg.idx,
                        ));

                        self.free_int_reg.push(call_reg);

                        self.push_free_reg(reg);
                    }
                    // TODO: string parser to create a constant and print it using
                    // a syscall.
                    /*
                    "print" => {
                        self.rodata.push(format!(
                            "somestr: .str 'reg ${}{}'",
                            reg.1.get_char(),
                            reg.0
                        ));
                        self.assembly.push("print @somestr".to_string());
                    }*/
                    _ => return Err(Error::new(format!("Unknown builtin: {}", builtin))),
                };
            }

            Token::Function { name, args, body } => {
                self.assembly.push(format!("; [start func] {}", name));
                self.assembly.push(format!("func_{}:", name));

                for arg in args {
                    self.visit_token(arg)?;
                }

                body.iter()
                    .flatten()
                    .try_for_each(|expr| self.visit_token(expr))?;

                // TODO: return?

                log::debug!("{:#?}", self.variables);
                log::debug!("{:#?}", self.local_variables);
                log::debug!("{:#?}", self.used_reg);

                // clean up any local variables (in reverse order)
                let mut free_regs = vec![];
                for var in self.local_variables.iter().rev() {
                    let old_used_reg = self.used_reg.remove(self.variables[var]);
                    free_regs.push(old_used_reg);
                    self.variables.remove(var);
                }
                for reg in free_regs {
                    self.push_free_reg(reg);
                }
                self.assembly.push(format!("; [end func] {}", name));
            }

            Token::Arg { ident, typ } => {
                let reg = match typ {
                    Type::Real => self.free_real_reg.pop().unwrap(),
                    Type::Integer => self.free_int_reg.pop().unwrap(),
                    Type::Coll => self.free_vec_reg.pop().unwrap(),
                };
                self.variables.insert(ident.clone(), self.used_reg.len());
                log::debug!("{:#?}", self.variables);
                self.local_variables.push(ident.clone());
                log::debug!("{:#?}", self.local_variables);
                self.used_reg.push(reg);
                log::debug!("{:#?}", self.used_reg);
            }

            Token::Identifier { name } => {
                // println!("referencing variable '{}'", name.to_string());
                if !self.variables.contains_key(name) {
                    return Err(Error::new(format!("Unknown variable '{}'", name)));
                }

                let index = self.variables[name];

                // println!(".. found at {}", index);

                let copy_reg = match &self.used_reg[index].reg {
                    VmRegister::I(_) => self.free_int_reg.pop().unwrap(),
                    VmRegister::R(_) => self.free_real_reg.pop().unwrap(),
                    VmRegister::V(_) => self.free_vec_reg.pop().unwrap(),
                };

                // Copy the value of the current identifier into the new reg
                self.assembly.push(format!(
                    "copy ${}{} ${}{}",
                    copy_reg.get_char(),
                    copy_reg.idx,
                    self.used_reg[index].get_char(),
                    self.used_reg[index].idx
                ));

                self.used_reg.push(copy_reg);
            }

            Token::Real { value } => {
                let next_reg = self.free_real_reg.pop().unwrap();
                self.assembly
                    .push(format!("load $r{} #{:.2}", next_reg.idx, value));
                self.used_reg.push(next_reg);
            }

            Token::Integer { value } => {
                let next_reg = self.free_int_reg.pop().unwrap();
                self.assembly
                    .push(format!("load $i{} #{}", next_reg.idx, value));
                self.used_reg.push(next_reg);
            }

            Token::Coll { values } => {
                // Allocate memory for the heap and put the base address into a register.
                let alloc_reg = self.free_int_reg.pop().unwrap();
                self.assembly
                    .push(format!("alloc $i{} #{}", alloc_reg.idx, values.len() * 8));

                // Go through the collection and store each generated real to the heap.
                let vec_base_reg = self.free_int_reg.pop().unwrap();
                self.assembly
                    .push(format!("copy $i{} $i{}", vec_base_reg.idx, alloc_reg.idx));

                let mut value_it = values.iter().peekable();
                while let Some(v) = value_it.next() {
                    self.visit_token(v)?;
                    let mut used_reg = self.used_reg.pop().unwrap();
                    match used_reg.reg {
                        VmRegister::R(_) => {}
                        VmRegister::I(_) => {
                            // promote an integer to a real for storage in the collection
                            let real_reg = self.free_real_reg.pop().unwrap();
                            self.assembly
                                .push(format!("copy $r{} $i{}", real_reg.idx, used_reg.idx));
                            self.free_int_reg.push(used_reg);
                            used_reg = real_reg;
                        }
                        // TODO: nested collections
                        VmRegister::V(_) => {
                            return Err(Error::new(
                                "Unable to put collection into a collection".to_string(),
                            ));
                        }
                    };
                    self.assembly
                        .push(format!("sw $i{} $r{}", vec_base_reg.idx, used_reg.idx));
                    self.free_real_reg.push(used_reg);

                    // If we will be going round the loop again, increment the base index.
                    if value_it.peek().is_some() {
                        let inc_reg = self.free_int_reg.pop().unwrap();
                        self.assembly
                            .push(format!("load $i{} #{}", inc_reg.idx, size_of::<f64>()));
                        self.assembly.push(format!(
                            "add $i{} $i{} $i{}",
                            vec_base_reg.idx, vec_base_reg.idx, inc_reg.idx
                        ));
                        self.free_int_reg.push(inc_reg);
                    }
                }
                self.free_int_reg.push(vec_base_reg);

                // And finally load the heap info into a vector register.
                let vec_reg = self.free_vec_reg.pop().unwrap();
                self.assembly.push(format!(
                    "load $v{} $i{} #{}",
                    vec_reg.idx,
                    alloc_reg.idx,
                    values.len() * size_of::<f64>()
                ));
                self.free_int_reg.push(alloc_reg);
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
            Token::Arith {
                ref left,
                ref right,
            } => {
                self.visit_token(left)?;
                for term in right {
                    self.visit_token(&term.1)?;
                    self.visit_token(&term.0)?;
                }
            }
            Token::Expression {
                ref source,
                ref token,
            } => {
                self.assembly.push(format!("; {}", source));
                log::debug!("writing assembly for '{}'", source);
                self.visit_token(token)?;
            }
            Token::Program { ref statements } => {
                self.rodata.push(".data".into());
                self.assembly.push(".code".into());
                statements
                    .iter()
                    .flatten()
                    .try_for_each(|expr| self.visit_token(expr))?;

                self.assembly.push("halt\n".into());
            }
        };
        //println!("  [after] {:?}\t    {:?}", self.variables, self.used_reg);
        //println!(".. done visiting {:?}", node);
        self.integrity_check();
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
    use nom::IResult;

    use super::*;

    #[test]
    fn test_get_char() {
        let i: char = Register {
            idx: 0,
            reg: VmRegister::I(42),
        }
        .get_char();
        assert_eq!(i, 'i');

        let r: char = Register {
            idx: 0,
            reg: VmRegister::R(42.0),
        }
        .get_char();
        assert_eq!(r, 'r');

        let v: char = Register {
            idx: 0,
            reg: VmRegister::V(vec![]),
        }
        .get_char();
        assert_eq!(v, 'v');
    }

    fn generate_test_program(listing: &str) -> IResult<&str, Token> {
        match program(listing) {
            Ok((rest, tree)) => {
                assert!(rest.is_empty());
                Ok((rest, tree))
            }
            Err(e) => {
                println!("ERROR generating test program: {:?}", e);
                Err(e)
            }
        }
    }

    #[test]
    fn test_addition_real() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("1.2 + 3.4\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; 1.2 + 3.4",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "add $r29 $r31 $r30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 31);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 29,
                reg: VmRegister::R(0.0)
            }]
        );
    }

    #[test]
    fn test_addition_integer() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("1 + 3\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; 1 + 3",
                "load $i31 #1",
                "load $i30 #3",
                "add $i29 $i31 $i30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 29,
                reg: VmRegister::I(0)
            }]
        );
    }

    #[test]
    fn test_addition_vector() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("[1.2, 3.4] + [3.4, 1.2]\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; [1.2, 3.4] + [3.4, 1.2]",
                "alloc $i31 #16",
                "copy $i30 $i31",
                "load $r31 #1.20",
                "sw $i30 $r31",
                "load $i29 #8",
                "add $i30 $i30 $i29",
                "load $r31 #3.40",
                "sw $i30 $r31",
                "load $v31 $i31 #16",
                "alloc $i31 #16",
                "copy $i30 $i31",
                "load $r31 #3.40",
                "sw $i30 $r31",
                "load $i29 #8",
                "add $i30 $i30 $i29",
                "load $r31 #1.20",
                "sw $i30 $r31",
                "load $v30 $i31 #16",
                "add $v29 $v31 $v30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 31);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 29,
                reg: VmRegister::V(vec![])
            }]
        );
    }

    #[test]
    fn test_subtraction() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("1.2 - 3.4\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; 1.2 - 3.4",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "sub $r29 $r31 $r30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 31);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 29,
                reg: VmRegister::R(0.0)
            }]
        );
    }

    #[test]
    fn test_multiplication() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("1.2 * 3.4\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; 1.2 * 3.4",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "mul $r29 $r31 $r30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 31);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 29,
                reg: VmRegister::R(0.0)
            }]
        );
    }

    #[test]
    fn test_division() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("1.2 / 3.4\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; 1.2 / 3.4",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "div $r29 $r31 $r30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 31);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 29,
                reg: VmRegister::R(0.0)
            }]
        );
    }

    #[test]
    fn test_equals() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("1.2 + 4.1 eq 3.4\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; 1.2 + 4.1 eq 3.4",
                "load $r31 #1.20",
                "load $r30 #4.10",
                "add $r29 $r31 $r30",
                "load $r30 #3.40",
                "eq $i31 $r29 $r30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 31,
                reg: VmRegister::I(0)
            }]
        );
    }

    #[test]
    fn test_not_equals() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("1.2 neq 3.4\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; 1.2 neq 3.4",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "neq $i31 $r31 $r30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 31,
                reg: VmRegister::I(0)
            }]
        );
    }

    #[test]
    fn test_greater_than() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("1.2 gt 3.4\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; 1.2 gt 3.4",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "gt $i31 $r31 $r30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 31,
                reg: VmRegister::I(0)
            }]
        );
    }

    #[test]
    fn test_greater_than_equals() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("1.2 gte 3.4\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; 1.2 gte 3.4",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "gte $i31 $r31 $r30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 31,
                reg: VmRegister::I(0)
            }]
        );
    }

    #[test]
    fn test_less_than() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("1.2 lt 3.4\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; 1.2 lt 3.4",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "lt $i31 $r31 $r30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 31,
                reg: VmRegister::I(0)
            }]
        );
    }

    #[test]
    fn test_less_than_equals() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("1.2 lte 3.4\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; 1.2 lte 3.4",
                "load $r31 #1.20",
                "load $r30 #3.40",
                "lte $i31 $r31 $r30",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 31,
                reg: VmRegister::I(0)
            }]
        );
    }

    #[test]
    fn test_assign() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("foo = 42.0\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![".code", "; foo = 42.0", "load $i31 #42", "halt\n"]
        );
        assert_eq!(compiler.free_int_reg.len(), 31);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 31,
                reg: VmRegister::I(0)
            }]
        );
        assert_eq!(
            compiler.variables,
            [("foo".to_string(), 0)].iter().cloned().collect()
        );

        // test error to reassign type of ident.
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("foo = 42.0\nfoo=[1,2]\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_err());
    }

    #[test]
    fn test_identifier() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("foo = 42.3\nbar = foo\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; foo = 42.3",
                "load $r31 #42.30",
                "; bar = foo",
                "copy $r30 $r31",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 30);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(
            compiler.used_reg,
            vec![
                Register {
                    idx: 31,
                    reg: VmRegister::R(0.0)
                },
                Register {
                    idx: 30,
                    reg: VmRegister::R(0.0)
                }
            ]
        );
        assert_eq!(
            compiler.variables,
            [("foo".to_string(), 0), ("bar".to_string(), 1)]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_collection() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("[0.1, 1.2]\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; [0.1, 1.2]",
                "alloc $i31 #16",
                "copy $i30 $i31",
                "load $r31 #0.10",
                "sw $i30 $r31",
                "load $i29 #8",
                "add $i30 $i30 $i29",
                "load $r31 #1.20",
                "sw $i30 $r31",
                "load $v31 $i31 #16",
                "halt\n"
            ],
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 31);
        assert_eq!(
            compiler.used_reg,
            vec![Register {
                idx: 31,
                reg: VmRegister::V(vec![])
            }]
        );
    }

    #[test]
    fn test_builtin() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program("do(write, 42.0)\n").unwrap();
        assert!(compiler.visit_token(&test_program).is_ok());
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; do(write, 42.0)",
                "load $i31 #42",
                "load $i30 #0",
                "syscall $i30 $i31",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![]);

        let res = generate_test_program("do(foo)\n");
        assert!(res.is_err());
    }

    #[test]
    fn test_function() {
        let mut compiler = Compiler::new();
        let (_, test_program) = generate_test_program(
            "func foobar(i: integer, r: real) {\nr = r + i * 2\ndo(write, r)\n}\n",
        )
        .unwrap();
        //assert!(compiler.visit_token(&test_program).is_ok());
        println!("{:#?}", compiler.visit_token(&test_program));
        assert_eq!(
            compiler.assembly,
            vec![
                ".code",
                "; [start func] foobar",
                "func_foobar:",
                "; r = r + i * 2",
                "copy $r30 $r31",
                "copy $i30 $i31",
                "load $i29 #2",
                "mul $i28 $i30 $i29",
                "add $r29 $r30 $i28",
                "; do(write, r)",
                "copy $r31 $r29",
                "load $i28 #0",
                "syscall $i28 $r31",
                "; [end func] foobar",
                "halt\n"
            ]
        );
        assert_eq!(compiler.free_int_reg.len(), 32);
        assert_eq!(compiler.free_real_reg.len(), 32);
        assert_eq!(compiler.free_vec_reg.len(), 32);
        assert_eq!(compiler.used_reg, vec![]);

        // TODO: test shadowing
    }
}
