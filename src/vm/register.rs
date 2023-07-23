use crate::vm::error::Error;

use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    I(i32),
    R(f64),
    V(Vec<f64>),
}

impl TryInto<i32> for Register {
    type Error = Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Register::I(i) => Ok(i),
            Register::R(r) => {
                log::warn!("Possible loss of precision converting {} into integer", r);
                Ok(r as i32)
            }
            Register::V(_) => Err(Error::new("Cannot convert vector register into i32")),
        }
    }
}

impl TryInto<f64> for Register {
    type Error = Error;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Register::I(i) => Ok(i as f64),
            Register::R(r) => Ok(r),
            Register::V(_) => Err(Error::new("Cannot convert vector register into f64")),
        }
    }
}

impl TryInto<Vec<f64>> for Register {
    type Error = Error;

    fn try_into(self) -> Result<Vec<f64>, Self::Error> {
        match self {
            Register::I(_) => Err(Error::new("Cannot convert integer register into vector")),
            Register::R(_) => Err(Error::new("Cannot convert real register into vector")),
            Register::V(v) => Ok(v),
        }
    }
}

pub fn is_int_register(reg: u8) -> bool {
    !is_real_register(reg) && !is_vector_register(reg)
}

pub fn is_real_register(reg: u8) -> bool {
    (reg & 0b10000000) == 0b10000000
}

pub fn is_vector_register(reg: u8) -> bool {
    (reg & 0b01000000) == 0b01000000
}

pub fn idx_from_int_register(reg: u8) -> u8 {
    reg
}

pub fn idx_from_real_register(reg: u8) -> u8 {
    reg & 0b01111111
}

pub fn idx_from_vector_register(reg: u8) -> u8 {
    reg & 0b10111111
}

pub fn int_register_to_idx(reg: u8) -> u8 {
    reg
}

pub fn real_register_to_idx(reg: u8) -> u8 {
    reg | 0b10000000
}

pub fn vector_register_to_idx(reg: u8) -> u8 {
    reg | 0b01000000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_try_into() {
        let i: i32 = Register::I(42).try_into().unwrap();
        assert_eq!(i, 42);

        let r: f64 = Register::R(42.0).try_into().unwrap();
        assert_eq!(r, 42.0);

        let v: Vec<f64> = Register::V(vec![1.0f64, 2.0]).try_into().unwrap();
        assert_eq!(v, vec![1.0f64, 2.0]);

        // transcoding
        let i: i32 = Register::R(4.2).try_into().unwrap();
        assert_eq!(i, 4);

        let r: f64 = Register::I(42).try_into().unwrap();
        assert_eq!(r, 42.0);

        let v: Result<Vec<f64>, Error> = Register::R(42.0).try_into();
        assert!(v.is_err());

        let r: Result<f64, Error> = Register::V(vec![1.0f64, 2.0]).try_into();
        assert!(r.is_err());
    }

    #[test]
    fn test_is_int_register() {
        let reg = int_register_to_idx(24);
        assert!(is_int_register(reg));
        assert!(!is_real_register(reg));
        assert!(!is_vector_register(reg));
        assert_eq!(idx_from_int_register(reg), 24);
    }

    #[test]
    fn test_is_real_register() {
        let reg = real_register_to_idx(24);
        assert!(!is_int_register(reg));
        assert!(is_real_register(reg));
        assert!(!is_vector_register(reg));
        assert_eq!(idx_from_real_register(reg), 24);
    }

    #[test]
    fn test_is_vector_register() {
        let reg = vector_register_to_idx(24);
        assert!(!is_int_register(reg));
        assert!(!is_real_register(reg));
        assert!(is_vector_register(reg));
        assert_eq!(idx_from_vector_register(reg), 24);
    }
}
