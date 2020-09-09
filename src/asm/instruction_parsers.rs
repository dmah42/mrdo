use crate::asm::directive_parsers::*;
use crate::asm::label_parsers::*;
use crate::asm::opcode_parsers::*;
use crate::asm::operand_parsers::operand;
use crate::asm::symbols::*;
use crate::asm::Token;
use crate::vm::real_register_to_idx;

use nom::types::CompleteStr;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    // TODO: these are pub for directive_parsers. maybe private and
    // add a ctor.
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub opcode: Option<Token>,
    pub operand0: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
}

impl AssemblerInstruction {
    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    // TODO: return result instead of option
    pub fn label_name(&self) -> Option<String> {
        match &self.label {
            Some(l) => match l {
                Token::LabelDecl { name } => Some(name.clone()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn is_directive(&self) -> bool {
        self.directive.is_some()
    }

    pub fn directive_name(&self) -> Option<String> {
        match &self.directive {
            Some(d) => match d {
                Token::Directive { name } => Some(name.to_string()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn is_opcode(&self) -> bool {
        self.opcode.is_some()
    }

    pub fn has_operands(&self) -> bool {
        self.operand0.is_some() || self.operand1.is_some() || self.operand2.is_some()
    }

    pub fn string_constant(&self) -> Option<String> {
        match &self.operand0 {
            Some(d) => match d {
                Token::DoString { value } => Some(value.to_string()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn to_bytes(&self, symbols: &Table) -> Vec<u8> {
        let mut results = vec![];
        if let Some(ref token) = self.opcode {
            match token {
                Token::Op { code } => {
                    let b: u8 = (*code).into();
                    results.push(b);
                }
                _ => {
                    println!("Non-opcode found in opcode field!");
                    std::process::exit(1); // TODO: error returns
                }
            }
        };

        for operand in &[&self.operand0, &self.operand1, &self.operand2] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, symbols, &mut results)
            }
        }

        while results.len() < 4 {
            results.push(0);
        }

        results
    }

    fn extract_operand(t: &Token, symbols: &Table, results: &mut Vec<u8>) {
        match t {
            Token::IntRegister { idx } => {
                results.push(*idx);
            }
            Token::RealRegister { idx } => {
                let idx = real_register_to_idx(*idx);
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
                    println!("No value found for label {:?}", name);
                    std::process::exit(1); // TODO: error returns.
                }
            }
            _ => {
                println!("Opcode {:?} found in operand field", t);
            }
        };
    }
}

impl fmt::Display for AssemblerInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Label: {:?}, Opcode: {:?}\nOp 0: {:?}, Op 1: {:?}, Op 2: {:?})",
            self.label, self.opcode, self.operand0, self.operand1, self.operand2
        )
    }
}

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        instr: alt!(
            instruction_comb | directive
        ) >>
        (
            instr
        )
    )
);

named!(instruction_comb<CompleteStr, AssemblerInstruction>,
    do_parse!(
        l: opt!(label_decl) >>
        o: opcode >>
        o0: opt!(operand) >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        (
            AssemblerInstruction{
                label: l,
                directive: None,
                opcode: Some(o),
                operand0: o0,
                operand1: o1,
                operand2: o2,
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::Opcode;

    #[test]
    fn test_extract_operand_int_register() {
        let token = Token::IntRegister { idx: 4 };
        let symbols = Table::new();
        let mut results = vec![];

        AssemblerInstruction::extract_operand(&token, &symbols, &mut results);
        assert_eq!(results, vec![4]);
    }

    #[test]
    fn test_extract_operand_real_register() {
        let token = Token::RealRegister { idx: 3 };
        let symbols = Table::new();
        let mut results = vec![];

        AssemblerInstruction::extract_operand(&token, &symbols, &mut results);
        assert_eq!(results, vec![131]);
    }

    #[test]
    fn test_extract_operand_integer() {
        let token = Token::Integer { value: 42 };
        let symbols = Table::new();
        let mut results = vec![];

        AssemblerInstruction::extract_operand(&token, &symbols, &mut results);
        assert_eq!(results, vec![0, 0, 0, 42]);

        let token = Token::Integer { value: -42 };
        let mut results = vec![];

        AssemblerInstruction::extract_operand(&token, &symbols, &mut results);
        assert_eq!(results, vec![255, 255, 255, 214]);
    }

    #[test]
    fn test_extract_operand_real() {
        let token = Token::Real { value: 4.2 };
        let symbols = Table::new();
        let mut results = vec![];

        AssemblerInstruction::extract_operand(&token, &symbols, &mut results);
        assert_eq!(results, vec![64, 16, 204, 204, 204, 204, 204, 205]);

        let token = Token::Real { value: -4.2 };
        let mut results = vec![];

        AssemblerInstruction::extract_operand(&token, &symbols, &mut results);
        assert_eq!(results, vec![192, 16, 204, 204, 204, 204, 204, 205]);
    }

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_comb(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
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
        let result = instruction_comb(CompleteStr("load $0 @test1\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
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
        let result = instruction_comb(CompleteStr("halt"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
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
        let result = instruction_comb(CompleteStr("add %0 $1 $2\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
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
