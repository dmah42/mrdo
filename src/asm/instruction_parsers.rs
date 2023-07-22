use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::{map_res, opt};
use nom::sequence::{preceded, tuple};
use nom::IResult;

use crate::asm::directive_parsers::*;
use crate::asm::error::Error;
use crate::asm::label_parsers::*;
use crate::asm::opcode_parsers::*;
use crate::asm::operand_parsers::operand;
use crate::asm::symbols::*;
use crate::asm::Token;
use crate::vm::register::{real_register_to_idx, vector_register_to_idx};

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    label: Option<Token>,
    directive: Option<Token>,
    opcode: Option<Token>,
    operand0: Option<Token>,
    operand1: Option<Token>,
    operand2: Option<Token>,
}

impl Instruction {
    pub fn new_directive(
        directive: Token,
        label: Option<Token>,
        operand: Option<Token>,
    ) -> Instruction {
        Instruction {
            label,
            directive: Some(directive),
            opcode: None,
            operand0: operand,
            operand1: None,
            operand2: None,
        }
    }

    pub fn new_label(label: Token) -> Instruction {
        Instruction {
            label: Some(label),
            directive: None,
            opcode: None,
            operand0: None,
            operand1: None,
            operand2: None,
        }
    }

    pub fn new_opcode(
        opcode: Token,
        operand0: Option<Token>,
        operand1: Option<Token>,
        operand2: Option<Token>,
    ) -> Instruction {
        Instruction {
            label: None,
            directive: None,
            opcode: Some(opcode),
            operand0,
            operand1,
            operand2,
        }
    }

    pub fn new_comment() -> Instruction {
        Instruction {
            label: None,
            directive: None,
            opcode: None,
            operand0: None,
            operand1: None,
            operand2: None,
        }
    }

    pub fn is_comment(&self) -> bool {
        self.label.is_none() && self.directive.is_none() && self.opcode.is_none()
    }

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn label_name(&self) -> Option<String> {
        if let Some(Token::LabelDecl { name }) = &self.label {
            Some(name.clone())
        } else {
            None
        }
    }

    pub fn is_directive(&self) -> bool {
        self.directive.is_some()
    }

    pub fn directive_name(&self) -> Option<String> {
        if let Some(Token::Directive { name }) = &self.directive {
            Some(name.clone())
        } else {
            None
        }
    }

    pub fn is_opcode(&self) -> bool {
        self.opcode.is_some()
    }

    pub fn has_operands(&self) -> bool {
        self.operand0.is_some() || self.operand1.is_some() || self.operand2.is_some()
    }

    pub fn string_constant(&self) -> Option<String> {
        if let Some(Token::DoString { value }) = &self.operand0 {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn to_bytes(&self, symbols: &Table) -> Result<Vec<u8>, Error> {
        let mut results = vec![];
        // println!(".. writing {}", self);
        if let Some(ref token) = self.opcode {
            match token {
                Token::Op { code } => {
                    let b: u8 = (*code).into();
                    results.push(b);
                }
                _ => return Err(Error::NotAnOpcode),
            }
        };

        [&self.operand0, &self.operand1, &self.operand2]
            .iter()
            .copied()
            .flatten()
            .try_for_each(|token| -> Result<(), Error> {
                Instruction::extract_operand(token, symbols, &mut results)?;
                Ok(())
            })?;

        while results.len() < 4 {
            results.push(0);
        }

        Ok(results)
    }

    fn extract_operand(t: &Token, symbols: &Table, results: &mut Vec<u8>) -> Result<(), Error> {
        match t {
            Token::IntRegister { idx } => {
                results.push(*idx);
            }
            Token::RealRegister { idx } => {
                let idx = real_register_to_idx(*idx);
                results.push(idx);
            }
            Token::VectorRegister { idx } => {
                let idx = vector_register_to_idx(*idx);
                results.push(idx);
            }
            Token::Integer { value } => {
                for b in value.to_be_bytes().iter() {
                    results.push(*b);
                }
            }
            Token::Real { value } => {
                for b in value.to_be_bytes().iter() {
                    results.push(*b);
                }
            }
            Token::LabelRef { name } => {
                if let Some(value) = symbols.value(name) {
                    let lb = value;
                    let hb = value >> 8;
                    results.push(hb as u8);
                    results.push(lb as u8);
                } else {
                    return Err(Error::UnknownLabel {
                        name: name.to_string(),
                    });
                }
            }
            _ => {
                return Err(Error::UnexpectedToken { token: t.clone() });
            }
        };
        Ok(())
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\tLabel: {:?}, Opcode: {:?}\tOp 0: {:?}, Op 1: {:?}, Op 2: {:?}",
            self.label, self.opcode, self.operand0, self.operand1, self.operand2
        )
    }
}

pub fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((instruction_comb, directive))(i)
}

fn instruction_comb(i: &str) -> IResult<&str, Instruction> {
    log::debug!("[asm::instruction] parsing '{}'", i);
    map_res(
        tuple((
            opt(label_decl),
            opcode,
            opt(preceded(multispace1, operand)),
            opt(preceded(multispace1, operand)),
            opt(preceded(multispace1, operand)),
        )),
        |(_l, op, o0, o1, o2)| -> Result<Instruction, nom::error::Error<&str>> {
            log::debug!("[asm::instruction] success ({:?})'", op);
            Ok(Instruction::new_opcode(op, o0, o1, o2))
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::opcode::Opcode;

    #[test]
    fn test_extract_operand_int_register() {
        let token = Token::IntRegister { idx: 4 };
        let symbols = Table::new();
        let mut results = vec![];

        assert!(Instruction::extract_operand(&token, &symbols, &mut results).is_ok());
        assert_eq!(results, vec![4]);
    }

    #[test]
    fn test_extract_operand_real_register() {
        let token = Token::RealRegister { idx: 3 };
        let symbols = Table::new();
        let mut results = vec![];

        assert!(Instruction::extract_operand(&token, &symbols, &mut results).is_ok());
        assert_eq!(results, vec![131]);
    }

    #[test]
    fn test_extract_operand_integer() {
        let token = Token::Integer { value: 42 };
        let symbols = Table::new();
        let mut results = vec![];

        assert!(Instruction::extract_operand(&token, &symbols, &mut results).is_ok());
        assert_eq!(results, vec![0, 0, 0, 42]);

        let token = Token::Integer { value: -42 };
        let mut results = vec![];

        assert!(Instruction::extract_operand(&token, &symbols, &mut results).is_ok());
        assert_eq!(results, vec![255, 255, 255, 214]);
    }

    #[test]
    fn test_extract_operand_real() {
        let token = Token::Real { value: 4.2 };
        let symbols = Table::new();
        let mut results = vec![];

        assert!(Instruction::extract_operand(&token, &symbols, &mut results).is_ok());
        assert_eq!(results, vec![64, 16, 204, 204, 204, 204, 204, 205]);

        let token = Token::Real { value: -4.2 };
        let mut results = vec![];

        assert!(Instruction::extract_operand(&token, &symbols, &mut results).is_ok());
        assert_eq!(results, vec![192, 16, 204, 204, 204, 204, 204, 205]);
    }

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_comb("load $i0 #100");
        assert_eq!(
            result,
            Ok((
                "",
                Instruction {
                    label: None,
                    directive: None,
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    operand0: Some(Token::IntRegister { idx: 0 }),
                    operand1: Some(Token::Integer { value: 100 }),
                    operand2: None,
                }
            ))
        )
    }

    #[test]
    fn test_parse_instruction_form_one_with_label() {
        let result = instruction_comb("load $i0 @test1");
        assert_eq!(
            result,
            Ok((
                "",
                Instruction {
                    label: None,
                    directive: None,
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    operand0: Some(Token::IntRegister { idx: 0 }),
                    operand1: Some(Token::LabelRef {
                        name: "test1".to_string()
                    }),
                    operand2: None,
                }
            ))
        )
    }

    #[test]
    fn test_parse_instruction_form_two() {
        let result = instruction_comb("halt");
        assert_eq!(
            result,
            Ok((
                "",
                Instruction {
                    label: None,
                    directive: None,
                    opcode: Some(Token::Op { code: Opcode::HLT }),
                    operand0: None,
                    operand1: None,
                    operand2: None,
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_three() {
        let result = instruction_comb("add $r0 $i1 $i2");
        assert_eq!(
            result,
            Ok((
                "",
                Instruction {
                    label: None,
                    directive: None,
                    opcode: Some(Token::Op { code: Opcode::ADD }),
                    operand0: Some(Token::RealRegister { idx: 0 }),
                    operand1: Some(Token::IntRegister { idx: 1 }),
                    operand2: Some(Token::IntRegister { idx: 2 }),
                }
            ))
        );
    }
}
