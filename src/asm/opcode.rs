use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Opcode {
    HLT,
    LOAD,
    COPY,
    // TODO: LW/SW already don't operate only on words so should be renamed. maybe find a vec version too?
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
    AND,
    OR,
    NOT,
    ALLOC,
    SYSCALL,
    IGL = 255,
}

impl<'a> From<&'a str> for Opcode {
    fn from(v: &'a str) -> Self {
        let lowercase_opcode = v.to_lowercase();
        match lowercase_opcode.as_str() {
            "halt" => Opcode::HLT,
            "load" => Opcode::LOAD,
            "copy" => Opcode::COPY,
            "lw" => Opcode::LW,
            "sw" => Opcode::SW,
            "add" => Opcode::ADD,
            "sub" => Opcode::SUB,
            "mul" => Opcode::MUL,
            "div" => Opcode::DIV,
            "jmp" => Opcode::JMP,
            "eq" => Opcode::EQ,
            "neq" => Opcode::NEQ,
            "gt" => Opcode::GT,
            "lt" => Opcode::LT,
            "gte" => Opcode::GTE,
            "lte" => Opcode::LTE,
            "jeq" => Opcode::JEQ,
            "and" => Opcode::AND,
            "or" => Opcode::OR,
            "not" => Opcode::NOT,
            "alloc" => Opcode::ALLOC,
            "syscall" => Opcode::SYSCALL,
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
        assert_eq!(Opcode::from("load"), Opcode::LOAD);
        assert_eq!(Opcode::from("lOaD"), Opcode::LOAD);
        assert_eq!(Opcode::from("daol"), Opcode::IGL);
    }
}
