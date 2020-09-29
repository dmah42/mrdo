use nom::types::CompleteStr;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Opcode {
    HLT,
    LOAD,
    LW,
    SW,
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
    SYSCALL,
    IGL = 255,
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        let lowercase_opcode = v.to_lowercase();
        match CompleteStr(&lowercase_opcode) {
            CompleteStr("halt") => Opcode::HLT,
            CompleteStr("load") => Opcode::LOAD,
            CompleteStr("lw") => Opcode::LW,
            CompleteStr("sw") => Opcode::SW,
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
            CompleteStr("syscall") => Opcode::SYSCALL,
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
