use crate::vm::error::Error;
use crate::vm::register::*;
use crate::vm::VM;

use std::convert::TryInto;

impl VM {
    pub fn add(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();
        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(out_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                self.iregisters[out_idx as usize] = a + b;
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                self.rregisters[idx_from_real_register(out_idx) as usize] = a + b;
            }
            Register::V(_) => {
                if let Register::V(va) = a_reg {
                    if let Register::V(vb) = b_reg {
                        // pairwise add across vectors.
                        if va.len() != vb.len() {
                            return Err(Error::new("Cannot add vectors with unequal lengths"));
                        }
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            va.iter().zip(vb).map(|(a, b)| a + b).collect();
                    } else {
                        // add b to every element of a
                        let b: f64 = b_reg.try_into()?;
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            va.iter().map(|a| a + b).collect();
                    }
                } else if let Register::V(vb) = b_reg {
                    // add a to every element of b
                    let a: f64 = a_reg.try_into()?;
                    self.vregisters[idx_from_vector_register(out_idx) as usize] =
                        vb.iter().map(|b| a + b).collect();
                } else {
                    return Err(Error::new(
                        "Cannot add two non-vector registers into a vector register",
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn sub(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();
        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(out_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                self.iregisters[out_idx as usize] = a - b;
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                self.rregisters[idx_from_real_register(out_idx) as usize] = a - b;
            }
            Register::V(_) => {
                if let Register::V(va) = a_reg {
                    if let Register::V(vb) = b_reg {
                        // pairwise sub across vectors.
                        if va.len() != vb.len() {
                            return Err(Error::new("Cannot sub vectors with unequal lengths"));
                        }
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            va.iter().zip(vb).map(|(a, b)| a - b).collect();
                    } else {
                        // sub b from every element of a
                        let b: f64 = b_reg.try_into()?;
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            va.iter().map(|a| a - b).collect();
                    }
                } else if let Register::V(_) = b_reg {
                    return Err(Error::new("Cannot sub vector from real"));
                } else {
                    return Err(Error::new(
                        "Cannot sub two non-vector registers into a vector register",
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn mul(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();
        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(out_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                self.iregisters[out_idx as usize] = a * b;
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                self.rregisters[idx_from_real_register(out_idx) as usize] = a * b;
            }
            Register::V(_) => {
                if let Register::V(va) = a_reg {
                    if let Register::V(vb) = b_reg {
                        // pairwise mul across vectors.
                        if va.len() != vb.len() {
                            return Err(Error::new("Cannot mul vectors with unequal lengths"));
                        }
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            va.iter().zip(vb).map(|(a, b)| a * b).collect();
                    } else {
                        // mul every element of a by b
                        let b: f64 = b_reg.try_into()?;
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            va.iter().map(|a| a * b).collect();
                    }
                } else if let Register::V(vb) = b_reg {
                    // mul every element of b by a
                    let a: f64 = a_reg.try_into()?;
                    self.vregisters[idx_from_vector_register(out_idx) as usize] =
                        vb.iter().map(|b| a * b).collect();
                } else {
                    return Err(Error::new(
                        "Cannot mul two non-vector registers into a vector register",
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn div(&mut self) -> Result<(), Error> {
        let out_idx = self.next_u8();
        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        let a_reg = self.get_register(a_idx)?;
        let b_reg = self.get_register(b_idx)?;

        match self.get_register(out_idx)? {
            Register::I(_) => {
                let a: i32 = a_reg.try_into()?;
                let b: i32 = b_reg.try_into()?;

                self.iregisters[out_idx as usize] = a / b;
            }
            Register::R(_) => {
                let a: f64 = a_reg.try_into()?;
                let b: f64 = b_reg.try_into()?;

                self.rregisters[idx_from_real_register(out_idx) as usize] = a / b;
            }
            Register::V(_) => {
                if let Register::V(va) = a_reg {
                    if let Register::V(vb) = b_reg {
                        // pairwise divide across vectors.
                        if va.len() != vb.len() {
                            return Err(Error::new("Cannot divide vectors with unequal lengths"));
                        }
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            va.iter().zip(vb).map(|(a, b)| a / b).collect();
                    } else {
                        // divide every element of a by b
                        let b: f64 = b_reg.try_into()?;
                        self.vregisters[idx_from_vector_register(out_idx) as usize] =
                            va.iter().map(|a| a / b).collect();
                    }
                } else if let Register::V(_) = b_reg {
                    return Err(Error::new("Cannot divide a real by a vector"));
                } else {
                    return Err(Error::new(
                        "Cannot divide two non-vector registers into a vector register",
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

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_opcode_add() {
        // integer
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::ADD as u8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 5);

        // real to integer
        let mut vm = VM::new();
        vm.rregisters[0] = 3.2;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::ADD as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 5);

        // integer to real
        let mut vm = VM::new();
        vm.rregisters[0] = 3.2;
        vm.iregisters[1] = 2;
        vm.program = vec![
            Opcode::ADD as u8,
            real_register_to_idx(0),
            real_register_to_idx(0),
            1,
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.rregisters[0], 5.2);

        // vector to vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::ADD as u8,
            vector_register_to_idx(0),
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.vregisters[0], vec![3.0, 5.0, 7.1]);

        // real to vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.rregisters[0] = 1.2;
        vm.program = vec![
            Opcode::ADD as u8,
            vector_register_to_idx(0),
            vector_register_to_idx(0),
            real_register_to_idx(0),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.vregisters[0], vec![2.2, 3.2, 4.3]);

        // vector to real
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.rregisters[0] = 1.2;
        vm.program = vec![
            Opcode::ADD as u8,
            vector_register_to_idx(0),
            real_register_to_idx(0),
            vector_register_to_idx(0),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.vregisters[0], vec![2.2, 3.2, 4.3]);

        // TODO: test error cases
    }

    #[test]
    fn test_opcode_sub() {
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::SUB as u8, 128, 128, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.rregisters[0], 1.0);

        // real to integer
        let mut vm = VM::new();
        vm.rregisters[0] = 3.2;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::SUB as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 1);

        // integer to real
        let mut vm = VM::new();
        vm.rregisters[0] = 3.2;
        vm.iregisters[1] = 2;
        vm.program = vec![
            Opcode::SUB as u8,
            real_register_to_idx(0),
            real_register_to_idx(0),
            1,
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_approx_eq!(vm.rregisters[0], 1.2);

        // vector to vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::SUB as u8,
            vector_register_to_idx(0),
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        for (i, vreg) in vec![-1.0, -1.0, -0.9].iter().enumerate() {
            assert_approx_eq!(vm.vregisters[0][i], vreg);
        }

        // real to vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.rregisters[0] = 1.2;
        vm.program = vec![
            Opcode::SUB as u8,
            vector_register_to_idx(0),
            vector_register_to_idx(0),
            real_register_to_idx(0),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        for (i, vreg) in vec![-0.2, 0.8, 1.9].iter().enumerate() {
            assert_approx_eq!(vm.vregisters[0][i], vreg);
        }

        // vector to real
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.rregisters[0] = 1.2;
        vm.program = vec![
            Opcode::SUB as u8,
            vector_register_to_idx(0),
            real_register_to_idx(0),
            vector_register_to_idx(0),
        ];
        let exit = vm.step();
        assert!(exit.is_err());
    }

    #[test]
    fn test_opcode_mul() {
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::MUL as u8, 0, 0, 129];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 6);

        // real to integer
        let mut vm = VM::new();
        vm.rregisters[0] = 3.2;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::MUL as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 6);

        // integer to real
        let mut vm = VM::new();
        vm.rregisters[0] = 3.2;
        vm.iregisters[1] = 2;
        vm.program = vec![
            Opcode::MUL as u8,
            real_register_to_idx(0),
            real_register_to_idx(0),
            1,
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_approx_eq!(vm.rregisters[0], 6.4);

        // vector to vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::MUL as u8,
            vector_register_to_idx(0),
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.vregisters[0], vec![2.0, 6.0, 12.4]);

        // real to vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.rregisters[0] = 1.2;
        vm.program = vec![
            Opcode::MUL as u8,
            vector_register_to_idx(0),
            vector_register_to_idx(0),
            real_register_to_idx(0),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        for (i, vreg) in vec![1.2, 2.4, 3.72].iter().enumerate() {
            assert_approx_eq!(vm.vregisters[0][i], vreg);
        }

        // vector to real
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.rregisters[0] = 1.2;
        vm.program = vec![
            Opcode::MUL as u8,
            vector_register_to_idx(0),
            real_register_to_idx(0),
            vector_register_to_idx(0),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        for (i, vreg) in vec![1.2, 2.4, 3.72].iter().enumerate() {
            assert_approx_eq!(vm.vregisters[0][i], vreg);
        }

        // TODO: test error cases
    }

    #[test]
    fn test_opcode_div() {
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![Opcode::DIV as u8, 128, 128, 129];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.rregisters[0], 1.5);

        // real to integer
        let mut vm = VM::new();
        vm.rregisters[0] = 3.2;
        vm.iregisters[1] = 2;
        vm.program = vec![Opcode::DIV as u8, 0, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 1);

        // integer to real
        let mut vm = VM::new();
        vm.rregisters[0] = 3.2;
        vm.iregisters[1] = 2;
        vm.program = vec![
            Opcode::DIV as u8,
            real_register_to_idx(0),
            real_register_to_idx(0),
            1,
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_approx_eq!(vm.rregisters[0], 1.6);

        // vector to vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.vregisters[1] = vec![2.0, 3.0, 4.0];
        vm.program = vec![
            Opcode::DIV as u8,
            vector_register_to_idx(0),
            vector_register_to_idx(0),
            vector_register_to_idx(1),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        for (i, vreg) in vec![0.5, 0.6666666, 0.775].iter().enumerate() {
            assert_approx_eq!(vm.vregisters[0][i], vreg);
        }

        // real to vector
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.rregisters[0] = 1.2;
        vm.program = vec![
            Opcode::DIV as u8,
            vector_register_to_idx(0),
            vector_register_to_idx(0),
            real_register_to_idx(0),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        for (i, vreg) in vec![0.8333333, 1.6666666, 2.5833333].iter().enumerate() {
            assert_approx_eq!(vm.vregisters[0][i], vreg);
        }

        // vector to real
        let mut vm = VM::new();
        vm.vregisters[0] = vec![1.0, 2.0, 3.1];
        vm.rregisters[0] = 1.2;
        vm.program = vec![
            Opcode::DIV as u8,
            vector_register_to_idx(0),
            real_register_to_idx(0),
            vector_register_to_idx(0),
        ];
        let exit = vm.step();
        assert!(exit.is_err());
    }
}
