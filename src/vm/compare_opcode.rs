use crate::vm::error::Error;
use crate::vm::register::*;
use crate::vm::VM;

use std::convert::TryInto;

impl VM {
    pub fn eq(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();

        if !is_int_register(out_idx) {
            return Err(Error::new(
                "Comparison operators require integer output registers",
            ));
        }

        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        if self.are_register_contents_equal(a_idx, b_idx)? {
            self.iregisters[out_idx as usize] = 1;
        } else {
            self.iregisters[out_idx as usize] = 0;
        }

        Ok(())
    }

    pub fn neq(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();

        if !is_int_register(out_idx) {
            return Err(Error::new(
                "Comparison operators require integer output registers",
            ));
        }

        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(a_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                if a != b {
                    self.iregisters[out_idx as usize] = 1;
                } else {
                    self.iregisters[out_idx as usize] = 0;
                }
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                if (a - b).abs() > f64::EPSILON {
                    self.iregisters[out_idx as usize] = 1;
                } else {
                    self.iregisters[out_idx as usize] = 0;
                }
            }
            Register::V(va) => {
                if let Register::V(vb) = b_reg {
                    if va != vb {
                        self.iregisters[out_idx as usize] = 1;
                    } else {
                        self.iregisters[out_idx as usize] = 0;
                    }
                } else {
                    return Err(Error::new("Cannot compare vectors with integers or reals"));
                }
            }
        }
        Ok(())
    }

    pub fn gt(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();

        if !is_int_register(out_idx) {
            return Err(Error::new(
                "Comparison operators require integer output registers",
            ));
        }

        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(a_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                if a > b {
                    self.iregisters[out_idx as usize] = 1;
                } else {
                    self.iregisters[out_idx as usize] = 0;
                }
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                if a > b {
                    self.iregisters[out_idx as usize] = 1;
                } else {
                    self.iregisters[out_idx as usize] = 0;
                }
            }
            Register::V(va) => {
                if let Register::V(vb) = b_reg {
                    if va > vb {
                        self.iregisters[out_idx as usize] = 1;
                    } else {
                        self.iregisters[out_idx as usize] = 0;
                    }
                } else {
                    return Err(Error::new("Cannot compare vectors with integers or reals"));
                }
            }
        }
        Ok(())
    }

    pub fn lt(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();

        if !is_int_register(out_idx) {
            return Err(Error::new(
                "Comparison operators require integer output registers",
            ));
        }

        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(a_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                if a < b {
                    self.iregisters[out_idx as usize] = 1;
                } else {
                    self.iregisters[out_idx as usize] = 0;
                }
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                if a < b {
                    self.iregisters[out_idx as usize] = 1;
                } else {
                    self.iregisters[out_idx as usize] = 0;
                }
            }
            Register::V(va) => {
                if let Register::V(vb) = b_reg {
                    if va < vb {
                        self.iregisters[out_idx as usize] = 1;
                    } else {
                        self.iregisters[out_idx as usize] = 0;
                    }
                } else {
                    return Err(Error::new("Cannot compare vectors with integers or reals"));
                }
            }
        }
        Ok(())
    }

    pub fn gte(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();

        if !is_int_register(out_idx) {
            return Err(Error::new(
                "Comparison operators require integer output registers",
            ));
        }

        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(a_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                if a >= b {
                    self.iregisters[out_idx as usize] = 1;
                } else {
                    self.iregisters[out_idx as usize] = 0;
                }
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                if a >= b {
                    self.iregisters[out_idx as usize] = 1;
                } else {
                    self.iregisters[out_idx as usize] = 0;
                }
            }
            Register::V(va) => {
                if let Register::V(vb) = b_reg {
                    if va >= vb {
                        self.iregisters[out_idx as usize] = 1;
                    } else {
                        self.iregisters[out_idx as usize] = 0;
                    }
                } else {
                    return Err(Error::new("Cannot compare vectors with integers or reals"));
                }
            }
        }
        Ok(())
    }

    pub fn lte(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();

        if !is_int_register(out_idx) {
            return Err(Error::new(
                "Comparison operators require integer output registers",
            ));
        }

        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(a_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                if a <= b {
                    self.iregisters[out_idx as usize] = 1;
                } else {
                    self.iregisters[out_idx as usize] = 0;
                }
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                if a <= b {
                    self.iregisters[out_idx as usize] = 1;
                } else {
                    self.iregisters[out_idx as usize] = 0;
                }
            }
            Register::V(va) => {
                if let Register::V(vb) = b_reg {
                    if va <= vb {
                        self.iregisters[out_idx as usize] = 1;
                    } else {
                        self.iregisters[out_idx as usize] = 0;
                    }
                } else {
                    return Err(Error::new("Cannot compare vectors with integers or reals"));
                }
            }
        }
        Ok(())
    }

    pub(super) fn are_register_contents_equal(&self, a_idx: u8, b_idx: u8) -> Result<bool, Error> {
        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(a_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                Ok(a == b)
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                Ok((a - b).abs() < f64::EPSILON)
            }
            Register::V(va) => {
                if let Register::V(vb) = b_reg {
                    Ok(va == vb)
                } else {
                    Err(Error::new("Cannot compare vectors with integers or reals"))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::asm::opcode::Opcode;

    #[test]
    fn test_opcode_eq() {
        // integer == integer
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // real == integer
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // vector == vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.vregisters[0] = vec![2.0, 3.0, 4.0];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);
    }

    #[test]
    fn test_opcode_neq() {
        // integer == integer
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // real == integer
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // vector == vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.vregisters[0] = vec![2.0, 3.0, 4.0];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);
    }

    #[test]
    fn test_opcode_gt() {
        // integer == integer
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // real == integer
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // vector == vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.vregisters[0] = vec![2.0, 3.0, 4.0];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);
    }

    #[test]
    fn test_opcode_gte() {
        // integer == integer
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // real == integer
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // vector == vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.vregisters[0] = vec![2.0, 3.0, 4.0];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);
    }
    #[test]
    fn test_opcode_lt() {
        // integer == integer
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // real == integer
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // vector == vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.vregisters[0] = vec![2.0, 3.0, 4.0];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);
    }
    #[test]
    fn test_opcode_lte() {
        // integer == integer
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // real == integer
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::EQ as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::EQ as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // integer == real
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            real_register_to_idx(0),
            real_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        // vector == vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        let mut vm = VM::new();
        vm.vregisters[0] = vec![2.0, 3.0, 4.0];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::EQ as u8,
            0,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);
    }
}
