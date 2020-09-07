use nom::types::CompleteStr;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Opcode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    EQ,
    NEQ,
    GT,
    LT,
    GTE,
    LTE,
    JEQ,
    ALLOC,
    PRINT,
    IGL = 255,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        let lowercase_opcode = v.to_lowercase();
        match CompleteStr(&lowercase_opcode) {
            CompleteStr("halt") => Opcode::HLT,
            CompleteStr("load") => Opcode::LOAD,
            CompleteStr("add") => Opcode::ADD,
            CompleteStr("sub") => Opcode::SUB,
            CompleteStr("mul") => Opcode::MUL,
            CompleteStr("div") => Opcode::DIV,
            CompleteStr("jmp") => Opcode::JMP,
            CompleteStr("eq") => Opcode::EQ,
            CompleteStr("neq") => Opcode::NEQ,
            CompleteStr("gt") => Opcode::GT,
            CompleteStr("lt") => Opcode::LT,
            CompleteStr("gte") => Opcode::GTE,
            CompleteStr("lte") => Opcode::LTE,
            CompleteStr("jeq") => Opcode::JEQ,
            CompleteStr("alloc") => Opcode::ALLOC,
            CompleteStr("print") => Opcode::PRINT,
            _ => Opcode::IGL,
        }
    }
}

impl Copy for Opcode {}

impl Clone for Opcode {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::{Into, TryFrom};

    #[test]
    fn test_create_hlt_instruction() {
        let inst = Instruction::new(Opcode::HLT);
        assert_eq!(inst.opcode, Opcode::HLT);
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(Opcode::try_from(0), Ok(Opcode::HLT));
        assert!(Opcode::try_from(200).is_err());
    }

    #[test]
    fn test_to_u8() {
        let loadi: u8 = Opcode::LOAD.into();
        assert_eq!(loadi, 1);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Opcode::from(CompleteStr("load")), Opcode::LOAD);
        assert_eq!(Opcode::from(CompleteStr("lOaD")), Opcode::LOAD);
        assert_eq!(Opcode::from(CompleteStr("daol")), Opcode::IGL);
    }
}
