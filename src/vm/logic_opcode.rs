use crate::vm::error::Error;
use crate::vm::register::*;
use crate::vm::VM;

use std::convert::TryInto;
use std::iter::zip;

trait Number: From<i32> + PartialEq {
    fn to_bool(self) -> bool;
}

impl<T> Number for T
where
    T: From<i32> + PartialEq,
{
    fn to_bool(self) -> bool {
        self != 0.into()
    }
}

impl VM {
    fn do_and<T: Number>(a: T, b: T) -> T {
        if a.to_bool() && b.to_bool() {
            1.into()
        } else {
            0.into()
        }
    }

    fn do_or<T: Number>(a: T, b: T) -> T {
        if a.to_bool() || b.to_bool() {
            1.into()
        } else {
            0.into()
        }
    }

    fn do_not<T: Number>(a: T) -> T {
        if a.to_bool() {
            0.into()
        } else {
            1.into()
        }
    }

    pub fn and(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();
        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(out_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                self.iregisters[out_idx as usize] = VM::do_and(a, b);
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                self.rregisters[idx_from_real_register(out_idx) as usize] = VM::do_and(a, b);
            }
            Register::V(_) => {
                if let Register::V(va) = a_reg {
                    if let Register::V(vb) = b_reg {
                        // pairwise and across vectors.
                        if va.len() != vb.len() {
                            return Err(Error::new("Cannot and vectors with unequal lengths"));
                        }
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            zip(va, vb).map(|a| VM::do_and(a.0, a.1)).collect();
                    } else {
                        // compare b to every element of a
                        let b: f64 = b_reg.try_into()?;
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            va.iter().map(|a| VM::do_and(*a, b)).collect();
                    }
                } else if let Register::V(vb) = b_reg {
                    // compare a to every element of b
                    let a: f64 = a_reg.try_into()?;
                    self.vregisters[idx_from_vector_register(out_idx) as usize] =
                        vb.iter().map(|b| VM::do_and(a, *b)).collect();
                } else {
                    return Err(Error::new(
                        "Cannot and two non-vector registers into a vector register",
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn or(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();
        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(out_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                self.iregisters[out_idx as usize] = VM::do_or(a, b);
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                self.rregisters[idx_from_real_register(out_idx) as usize] = VM::do_or(a, b);
            }
            Register::V(_) => {
                if let Register::V(va) = a_reg {
                    if let Register::V(vb) = b_reg {
                        // pairwise and across vectors.
                        if va.len() != vb.len() {
                            return Err(Error::new("Cannot and vectors with unequal lengths"));
                        }
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            zip(va, vb).map(|a| VM::do_or(a.0, a.1)).collect();
                    } else {
                        // compare b to every element of a
                        let b: f64 = b_reg.try_into()?;
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            va.iter().map(|a| VM::do_or(*a, b)).collect();
                    }
                } else if let Register::V(vb) = b_reg {
                    // compare a to every element of b
                    let a: f64 = a_reg.try_into()?;
                    self.vregisters[idx_from_vector_register(out_idx) as usize] =
                        vb.iter().map(|b| VM::do_or(a, *b)).collect();
                } else {
                    return Err(Error::new(
                        "Cannot and two non-vector registers into a vector register",
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn not(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();
        let in_idx = self.next_u8();

        let in_reg = self.get_register(in_idx)?;

        match self.get_register(out_idx)? {
            Register::I(_) => {
                let a: i32 = in_reg.try_into()?;

                self.iregisters[out_idx as usize] = VM::do_not(a);
            }
            Register::R(_) => {
                let a: f64 = in_reg.try_into()?;

                self.rregisters[idx_from_real_register(out_idx) as usize] = VM::do_not(a);
            }
            Register::V(_) => {
                if let Register::V(va) = in_reg {
                    self.vregisters[idx_from_vector_register(out_idx) as usize] =
                        va.iter().map(|a| VM::do_not(*a)).collect();
                } else {
                    return Err(Error::new(
                        "Cannot and two non-vector registers into a vector register",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::asm::opcode::Opcode;

    #[test]
    fn test_opcode_and_f64() {
        assert_eq!(VM::do_and(42.0, 64.0), 1.0);
        assert_eq!(VM::do_and(0.0, 42.0), 0.0);
        assert_eq!(VM::do_and(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_opcode_and_i32() {
        assert_eq!(VM::do_and(42, 64), 1);
        assert_eq!(VM::do_and(0, 42), 0);
        assert_eq!(VM::do_and(0, 0), 0);
    }

    #[test]
    fn test_opcode_or_f64() {
        assert_eq!(VM::do_or(42.0, 64.0), 1.0);
        assert_eq!(VM::do_or(0.0, 42.0), 1.0);
        assert_eq!(VM::do_or(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_opcode_or_i32() {
        assert_eq!(VM::do_or(42, 64), 1);
        assert_eq!(VM::do_or(0, 42), 1);
        assert_eq!(VM::do_or(0, 0), 0);
    }

    #[test]
    fn test_opcode_not_f64() {
        assert_eq!(VM::do_not(42.0), 0.0);
        assert_eq!(VM::do_not(0.0), 1.0);
    }

    #[test]
    fn test_opcode_not_i32() {
        assert_eq!(VM::do_not(42), 0);
        assert_eq!(VM::do_not(0), 1);
    }

    #[test]
    fn test_opcode_and() {
        // integer && integer
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::AND as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 0;
        vm.program = vec![Opcode::AND as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        // real && integer
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::AND as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        let mut vm = VM::new();
        vm.rregisters[0] = 2.0;
        vm.iregisters[1] = 0;
        vm.program = vec![Opcode::AND as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        // integer && real
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::AND as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 1);

        let mut vm = VM::new();
        vm.iregisters[0] = 2;
        vm.rregisters[1] = 0.0;
        vm.program = vec![Opcode::AND as u8, 0, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 0);

        // real && real
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.rregisters[1] = 0.0;
        vm.program = vec![
            Opcode::AND as u8,
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
            Opcode::AND as u8,
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
            Opcode::AND as u8,
            vector_register_to_idx(0),
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.vregisters[0], vec![1.0, 1.0, 1.0]);

        let mut vm = VM::new();
        vm.vregisters[0] = vec![2.0, 3.0, 0.0];
        vm.vregisters[1] = vec![2.0, 0.0, 4.0];
        vm.program = vec![
            Opcode::AND as u8,
            vector_register_to_idx(0),
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.vregisters[0], vec![1.0, 0.0, 0.0]);
    }
}
