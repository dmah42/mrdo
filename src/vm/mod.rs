use crate::asm::instruction::*;
use crate::asm::{DO_HEADER_LEN, DO_HEADER_PREFIX};
use crate::vm::error::Error;

use std::convert::TryFrom;

pub mod error;

pub struct VM {
    pub iregisters: [i32; 32],
    pub rregisters: [f64; 32],
    pub program: Vec<u8>,
    heap: Vec<u8>,
    pc: usize,
    pub ro_data: Vec<u8>,
}

pub fn is_valid_bytecode(bytecode: &[u8]) -> bool {
    bytecode.len() > DO_HEADER_LEN && bytecode[0..4] == DO_HEADER_PREFIX
}

fn is_int_register(reg: u8) -> bool {
    (reg & 0b10000000) == 0
}

fn idx_from_real_register(reg: u8) -> u8 {
    reg & 0b01111111
}

pub fn real_register_to_idx(reg: u8) -> u8 {
    reg | 0b10000000
}

impl VM {
    pub fn new() -> VM {
        VM {
            iregisters: [0; 32],
            rregisters: [0.0; 32],
            program: vec![],
            heap: vec![],
            pc: 0,
            ro_data: vec![],
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut exit = false;
        while !exit {
            exit = self.step()?;
        }
        Ok(())
    }

    pub fn set_bytecode(&mut self, bytecode: &[u8]) -> Result<(), Error> {
        if !is_valid_bytecode(bytecode) {
            return Err(Error::new("Invalid bytecode"));
        }

        self.program.clear();
        self.program.append(&mut bytecode.to_vec());

        let bytes = &self.program[4..8];

        let ro_len = ((bytes[0] as u32) << 24
            | (bytes[1] as u32) << 16
            | (bytes[2] as u32) << 8
            | (bytes[3] as u32)) as usize;

        let ro_end = DO_HEADER_LEN + ro_len;

        if ro_len != 0 {
            self.ro_data
                .append(&mut (self.program[DO_HEADER_LEN..ro_end].to_vec()));
        }

        self.pc = ro_end;
        Ok(())
    }

    // Step one instruction. Returns an error or a boolean indicating the program is complete.
    pub fn step(&mut self) -> Result<bool, Error> {
        if self.pc >= self.program.len() {
            return Err(Error::new("Ran out of program to run"));
        }
        let opcode = self.decode_opcode();
        match opcode {
            Opcode::HLT => {
                return Ok(true);
            }
            Opcode::LOAD => {
                // TODO: future proof this a bit. `LOAD %1 10` will fail, as
                // will `LOAD $1 3.14`.
                let register = self.next_u8();
                if is_int_register(register) {
                    self.iregisters[register as usize] = self.next_i32();
                } else {
                    self.rregisters[idx_from_real_register(register) as usize] = self.next_f64();
                }
            }
            Opcode::ADD => self.add(),
            Opcode::SUB => self.sub(),
            Opcode::MUL => self.mul(),
            Opcode::DIV => self.div(),
            Opcode::JMP => {
                let target = self.iregisters[self.next_u8() as usize];
                self.pc = target as usize;
            }
            Opcode::EQ => self.eq(),
            Opcode::NEQ => self.neq(),
            Opcode::GT => self.gt(),
            Opcode::LT => self.lt(),
            Opcode::GTE => self.gte(),
            Opcode::LTE => self.lte(),
            Opcode::JEQ => self.jeq()?,
            Opcode::ALLOC => {
                let register = self.next_u8();
                if !is_int_register(register) {
                    return Err(Error::new(
                        "Cannot write heap location to non-integer register",
                    ));
                }
                let bytes_reg = self.next_u8();
                if !is_int_register(register) {
                    return Err(Error::new("Cannot allocate non-integer number of bytes"));
                }
                let bytes = self.iregisters[bytes_reg as usize];
                self.iregisters[register as usize] = self.heap.len() as i32;
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            }
            Opcode::PRINT => {
                let offset = self.next_u16() as usize;
                let mut end = offset;
                let slice = self.ro_data.as_slice();

                while slice[end] != 0 {
                    end += 1;
                }

                let result = std::str::from_utf8(&slice[offset..end]);
                match result {
                    Ok(s) => {
                        println!("{}", s);
                    }
                    Err(e) => {
                        return Err(Error::new(&format!(
                            "Error decoding string to print: {:#?}",
                            e
                        )))
                    }
                };
            }
            _ => {
                return Err(Error::new(&format!("Unrecognized opcode '{:?}'", opcode)));
            }
        }
        Ok(false)
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::try_from(self.program[self.pc]).unwrap();
        self.pc += 1;
        opcode
    }

    fn next_u8(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_u16(&mut self) -> u16 {
        let bytes = [self.program[self.pc], self.program[self.pc + 1]];
        self.pc += 2;
        u16::from_be_bytes(bytes)
    }

    fn next_i32(&mut self) -> i32 {
        let bytes = [
            self.program[self.pc],
            self.program[self.pc + 1],
            self.program[self.pc + 2],
            self.program[self.pc + 3],
        ];
        self.pc += 4;
        i32::from_be_bytes(bytes)
    }

    fn next_f64(&mut self) -> f64 {
        let bytes = [
            self.program[self.pc],
            self.program[self.pc + 1],
            self.program[self.pc + 2],
            self.program[self.pc + 3],
            self.program[self.pc + 4],
            self.program[self.pc + 5],
            self.program[self.pc + 6],
            self.program[self.pc + 7],
        ];
        self.pc += 8;
        f64::from_be_bytes(bytes)
    }

    fn add(&mut self) {
        let register = self.next_u8();
        let a_reg = self.next_u8();
        let b_reg = self.next_u8();

        let mut ia: Option<i32> = None;
        let mut ra: Option<f64> = None;
        if is_int_register(a_reg) {
            ia = Some(self.iregisters[a_reg as usize]);
        } else {
            ra = Some(self.rregisters[idx_from_real_register(a_reg) as usize]);
        }

        let mut ib: Option<i32> = None;
        let mut rb: Option<f64> = None;
        if is_int_register(b_reg) {
            ib = Some(self.iregisters[b_reg as usize]);
        } else {
            rb = Some(self.rregisters[idx_from_real_register(b_reg) as usize]);
        }

        if is_int_register(register) {
            let a: i32 = match ia {
                Some(i) => i,
                None => ra.unwrap() as i32,
            };

            let b: i32 = match ib {
                Some(i) => i,
                None => rb.unwrap() as i32,
            };

            self.iregisters[register as usize] = a + b;
        } else {
            let a: f64 = match ra {
                Some(r) => r,
                None => ia.unwrap() as f64,
            };

            let b: f64 = match rb {
                Some(r) => r,
                None => ib.unwrap() as f64,
            };

            self.rregisters[idx_from_real_register(register) as usize] = a + b;
        }
    }

    fn sub(&mut self) {
        let register = self.next_u8();
        let a_reg = self.next_u8();
        let b_reg = self.next_u8();

        let mut ia: Option<i32> = None;
        let mut ra: Option<f64> = None;
        if is_int_register(a_reg) {
            ia = Some(self.iregisters[a_reg as usize]);
        } else {
            ra = Some(self.rregisters[idx_from_real_register(a_reg) as usize]);
        }

        let mut ib: Option<i32> = None;
        let mut rb: Option<f64> = None;
        if is_int_register(b_reg) {
            ib = Some(self.iregisters[b_reg as usize]);
        } else {
            rb = Some(self.rregisters[idx_from_real_register(b_reg) as usize]);
        }

        if is_int_register(register) {
            let a: i32 = match ia {
                Some(i) => i,
                None => ra.unwrap() as i32,
            };

            let b: i32 = match ib {
                Some(i) => i,
                None => rb.unwrap() as i32,
            };

            self.iregisters[register as usize] = a - b;
        } else {
            let a: f64 = match ra {
                Some(r) => r,
                None => ia.unwrap() as f64,
            };

            let b: f64 = match rb {
                Some(r) => r,
                None => ib.unwrap() as f64,
            };

            self.rregisters[idx_from_real_register(register) as usize] = a - b;
        }
    }

    fn mul(&mut self) {
        let register = self.next_u8();
        let a_reg = self.next_u8();
        let b_reg = self.next_u8();

        let mut ia: Option<i32> = None;
        let mut ra: Option<f64> = None;
        if is_int_register(a_reg) {
            ia = Some(self.iregisters[a_reg as usize]);
        } else {
            ra = Some(self.rregisters[idx_from_real_register(a_reg) as usize]);
        }

        let mut ib: Option<i32> = None;
        let mut rb: Option<f64> = None;
        if is_int_register(b_reg) {
            ib = Some(self.iregisters[b_reg as usize]);
        } else {
            rb = Some(self.rregisters[idx_from_real_register(b_reg) as usize]);
        }

        if is_int_register(register) {
            let a: i32 = match ia {
                Some(i) => i,
                None => ra.unwrap() as i32,
            };

            let b: i32 = match ib {
                Some(i) => i,
                None => rb.unwrap() as i32,
            };

            self.iregisters[register as usize] = a * b;
        } else {
            let a: f64 = match ra {
                Some(r) => r,
                None => ia.unwrap() as f64,
            };

            let b: f64 = match rb {
                Some(r) => r,
                None => ib.unwrap() as f64,
            };

            self.rregisters[idx_from_real_register(register) as usize] = a * b;
        }
    }

    fn div(&mut self) {
        let register = self.next_u8();
        let a_reg = self.next_u8();
        let b_reg = self.next_u8();

        let mut ia: Option<i32> = None;
        let mut ra: Option<f64> = None;
        if is_int_register(a_reg) {
            ia = Some(self.iregisters[a_reg as usize]);
        } else {
            ra = Some(self.rregisters[idx_from_real_register(a_reg) as usize]);
        }

        let mut ib: Option<i32> = None;
        let mut rb: Option<f64> = None;
        if is_int_register(b_reg) {
            ib = Some(self.iregisters[b_reg as usize]);
        } else {
            rb = Some(self.rregisters[idx_from_real_register(b_reg) as usize]);
        }

        if is_int_register(register) {
            let a: i32 = match ia {
                Some(i) => i,
                None => ra.unwrap() as i32,
            };

            let b: i32 = match ib {
                Some(i) => i,
                None => rb.unwrap() as i32,
            };

            self.iregisters[register as usize] = a / b;
        } else {
            let a: f64 = match ra {
                Some(r) => r,
                None => ia.unwrap() as f64,
            };

            let b: f64 = match rb {
                Some(r) => r,
                None => ib.unwrap() as f64,
            };

            self.rregisters[idx_from_real_register(register) as usize] = a / b;
        }
    }

    fn eq(&mut self) {
        let register = self.next_u8();
        let a_reg = self.next_u8();
        let b_reg = self.next_u8();

        let mut ia: Option<i32> = None;
        let mut ra: Option<f64> = None;
        if is_int_register(a_reg) {
            ia = Some(self.iregisters[a_reg as usize]);
        } else {
            ra = Some(self.rregisters[idx_from_real_register(a_reg) as usize]);
        }

        let mut ib: Option<i32> = None;
        let mut rb: Option<f64> = None;
        if is_int_register(b_reg) {
            ib = Some(self.iregisters[b_reg as usize]);
        } else {
            rb = Some(self.rregisters[idx_from_real_register(b_reg) as usize]);
        }

        let a: f64 = match ra {
            Some(f) => f,
            None => ia.unwrap() as f64,
        };

        let b: f64 = match rb {
            Some(f) => f,
            None => ib.unwrap() as f64,
        };

        if (a - b).abs() < f64::EPSILON {
            if is_int_register(register) {
                self.iregisters[register as usize] = 1;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 1.0;
            }
        } else {
            if is_int_register(register) {
                self.iregisters[register as usize] = 0;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 0.0;
            }
        }
    }

    fn neq(&mut self) {
        let register = self.next_u8();
        let a_reg = self.next_u8();
        let b_reg = self.next_u8();

        let mut ia: Option<i32> = None;
        let mut ra: Option<f64> = None;
        if is_int_register(a_reg) {
            ia = Some(self.iregisters[a_reg as usize]);
        } else {
            ra = Some(self.rregisters[idx_from_real_register(a_reg) as usize]);
        }

        let mut ib: Option<i32> = None;
        let mut rb: Option<f64> = None;
        if is_int_register(b_reg) {
            ib = Some(self.iregisters[b_reg as usize]);
        } else {
            rb = Some(self.rregisters[idx_from_real_register(b_reg) as usize]);
        }

        let a: f64 = match ra {
            Some(f) => f,
            None => ia.unwrap() as f64,
        };

        let b: f64 = match rb {
            Some(f) => f,
            None => ib.unwrap() as f64,
        };

        if (a - b).abs() > f64::EPSILON {
            if is_int_register(register) {
                self.iregisters[register as usize] = 1;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 1.0;
            }
        } else {
            if is_int_register(register) {
                self.iregisters[register as usize] = 0;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 0.0;
            }
        }
    }

    fn gt(&mut self) {
        let register = self.next_u8();
        let a_reg = self.next_u8();
        let b_reg = self.next_u8();

        let mut ia: Option<i32> = None;
        let mut ra: Option<f64> = None;
        if is_int_register(a_reg) {
            ia = Some(self.iregisters[a_reg as usize]);
        } else {
            ra = Some(self.rregisters[idx_from_real_register(a_reg) as usize]);
        }

        let mut ib: Option<i32> = None;
        let mut rb: Option<f64> = None;
        if is_int_register(b_reg) {
            ib = Some(self.iregisters[b_reg as usize]);
        } else {
            rb = Some(self.rregisters[idx_from_real_register(b_reg) as usize]);
        }

        let a: f64 = match ra {
            Some(f) => f,
            None => ia.unwrap() as f64,
        };

        let b: f64 = match rb {
            Some(f) => f,
            None => ib.unwrap() as f64,
        };

        if a > b {
            if is_int_register(register) {
                self.iregisters[register as usize] = 1;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 1.0;
            }
        } else {
            if is_int_register(register) {
                self.iregisters[register as usize] = 0;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 0.0;
            }
        }
    }

    fn lt(&mut self) {
        let register = self.next_u8();
        let a_reg = self.next_u8();
        let b_reg = self.next_u8();

        let mut ia: Option<i32> = None;
        let mut ra: Option<f64> = None;
        if is_int_register(a_reg) {
            ia = Some(self.iregisters[a_reg as usize]);
        } else {
            ra = Some(self.rregisters[idx_from_real_register(a_reg) as usize]);
        }

        let mut ib: Option<i32> = None;
        let mut rb: Option<f64> = None;
        if is_int_register(b_reg) {
            ib = Some(self.iregisters[b_reg as usize]);
        } else {
            rb = Some(self.rregisters[idx_from_real_register(b_reg) as usize]);
        }

        let a: f64 = match ra {
            Some(f) => f,
            None => ia.unwrap() as f64,
        };

        let b: f64 = match rb {
            Some(f) => f,
            None => ib.unwrap() as f64,
        };

        if a < b {
            if is_int_register(register) {
                self.iregisters[register as usize] = 1;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 1.0;
            }
        } else {
            if is_int_register(register) {
                self.iregisters[register as usize] = 0;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 0.0;
            }
        }
    }

    fn gte(&mut self) {
        let register = self.next_u8();
        let a_reg = self.next_u8();
        let b_reg = self.next_u8();

        let mut ia: Option<i32> = None;
        let mut ra: Option<f64> = None;
        if is_int_register(a_reg) {
            ia = Some(self.iregisters[a_reg as usize]);
        } else {
            ra = Some(self.rregisters[idx_from_real_register(a_reg) as usize]);
        }

        let mut ib: Option<i32> = None;
        let mut rb: Option<f64> = None;
        if is_int_register(b_reg) {
            ib = Some(self.iregisters[b_reg as usize]);
        } else {
            rb = Some(self.rregisters[idx_from_real_register(b_reg) as usize]);
        }

        let a: f64 = match ra {
            Some(f) => f,
            None => ia.unwrap() as f64,
        };

        let b: f64 = match rb {
            Some(f) => f,
            None => ib.unwrap() as f64,
        };

        if a >= b {
            if is_int_register(register) {
                self.iregisters[register as usize] = 1;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 1.0;
            }
        } else {
            if is_int_register(register) {
                self.iregisters[register as usize] = 0;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 0.0;
            }
        }
    }

    fn lte(&mut self) {
        let register = self.next_u8();
        let a_reg = self.next_u8();
        let b_reg = self.next_u8();

        let mut ia: Option<i32> = None;
        let mut ra: Option<f64> = None;
        if is_int_register(a_reg) {
            ia = Some(self.iregisters[a_reg as usize]);
        } else {
            ra = Some(self.rregisters[idx_from_real_register(a_reg) as usize]);
        }

        let mut ib: Option<i32> = None;
        let mut rb: Option<f64> = None;
        if is_int_register(b_reg) {
            ib = Some(self.iregisters[b_reg as usize]);
        } else {
            rb = Some(self.rregisters[idx_from_real_register(b_reg) as usize]);
        }

        let a: f64 = match ra {
            Some(f) => f,
            None => ia.unwrap() as f64,
        };

        let b: f64 = match rb {
            Some(f) => f,
            None => ib.unwrap() as f64,
        };

        if a <= b {
            if is_int_register(register) {
                self.iregisters[register as usize] = 1;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 1.0;
            }
        } else {
            if is_int_register(register) {
                self.iregisters[register as usize] = 0;
            } else {
                self.rregisters[idx_from_real_register(register) as usize] = 0.0;
            }
        }
    }

    fn jeq(&mut self) -> Result<(), Error> {
        let register = self.next_u8();
        if !is_int_register(register) {
            return Err(Error::new("Cannot jump to non-integer location"));
        }
        let target = self.iregisters[register as usize];

        let a_reg = self.next_u8();
        let b_reg = self.next_u8();

        let mut ia: Option<i32> = None;
        let mut ra: Option<f64> = None;
        if is_int_register(a_reg) {
            ia = Some(self.iregisters[a_reg as usize]);
        } else {
            ra = Some(self.rregisters[idx_from_real_register(a_reg) as usize]);
        }

        let mut ib: Option<i32> = None;
        let mut rb: Option<f64> = None;
        if is_int_register(b_reg) {
            ib = Some(self.iregisters[b_reg as usize]);
        } else {
            rb = Some(self.rregisters[idx_from_real_register(b_reg) as usize]);
        }

        let a: f64 = match ra {
            Some(f) => f,
            None => ia.unwrap() as f64,
        };

        let b: f64 = match rb {
            Some(f) => f,
            None => ib.unwrap() as f64,
        };

        if (a - b).abs() < f64::EPSILON {
            self.pc = target as usize;
        }
        Ok(())
    }
}

impl Default for VM {
    fn default() -> Self {
        VM::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let vm = VM::new();
        assert_eq!(vm.iregisters[0], 0);
        assert_eq!(vm.rregisters[0], 0.0);
    }

    #[test]
    fn test_decode_opcode() {
        let mut vm = VM::new();
        vm.program = vec![0, 0, 0, 0];
        assert_eq!(vm.decode_opcode(), Opcode::HLT);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut vm = VM::new();
        vm.program = vec![0, 0, 0, 0];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), true);
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 0, 1, 244];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 500);

        let mut vm = VM::new();
        vm.program = vec![1, 128, 64, 16, 204, 204, 204, 204, 204, 205];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.rregisters[0], 4.2);
    }

    #[test]
    fn test_opcode_add() {
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![2, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 5);
    }

    #[test]
    fn test_opcode_sub() {
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.iregisters[1] = 2;
        vm.program = vec![3, 128, 128, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.rregisters[0], 1.0);
    }

    #[test]
    fn test_opcode_mul() {
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.rregisters[1] = 2.0;
        vm.program = vec![4, 0, 0, 129];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 6);
    }

    #[test]
    fn test_opcode_div() {
        let mut vm = VM::new();
        vm.rregisters[0] = 3.0;
        vm.rregisters[1] = 2.0;
        vm.program = vec![5, 128, 128, 129];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.rregisters[0], 1.5);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut vm = VM::new();
        vm.program = vec![6, 0, 0, 0];
        vm.iregisters[0] = 100;
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.pc, 100);
    }

    #[test]
    fn test_opcode_eq() {
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![7, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 0);
    }

    #[test]
    fn test_opcode_neq() {
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![8, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 1);
    }

    #[test]
    fn test_opcode_gt() {
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![9, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 1);
    }

    #[test]
    fn test_opcode_lt() {
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![10, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 0);
    }

    #[test]
    fn test_opcode_gte() {
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![11, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 1);
    }

    #[test]
    fn test_opcode_lte() {
        let mut vm = VM::new();
        vm.iregisters[0] = 3;
        vm.iregisters[1] = 2;
        vm.program = vec![12, 0, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 0);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut vm = VM::new();
        vm.iregisters[0] = 100;
        vm.iregisters[1] = 3;
        vm.iregisters[2] = 3;
        vm.program = vec![13, 0, 1, 2];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.pc, 100);
    }

    #[test]
    fn test_opcode_alloc() {
        let mut vm = VM::new();
        vm.iregisters[0] = 1024;
        vm.program = vec![14, 0, 0, 0];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.heap.len(), 1024);
    }

    #[test]
    fn test_print_opcode() {
        let mut vm = VM::new();
        vm.ro_data.append(&mut vec![72, 101, 108, 108, 111, 0]);
        vm.program = vec![15, 0, 0, 0];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
    }

    #[test]
    fn test_opcode_igl() {
        let mut vm = VM::new();
        vm.program = vec![255, 0, 0, 0];
        let exit = vm.step();
        assert!(exit.is_err());
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_set_bytecode() {
        let mut vm = VM::new();

        let result = vm.set_bytecode(&[1, 2, 3, 4, 0, 0, 0, 0]);
        assert!(result.is_err());

        let mut bytecode = vec![];
        bytecode.append(&mut DO_HEADER_PREFIX.to_vec());
        while bytecode.len() < DO_HEADER_LEN {
            bytecode.push(0 as u8);
        }
        bytecode.append(&mut vec![1, 2, 3, 4]);
        let result = vm.set_bytecode(&bytecode);
        assert!(result.is_ok());
        assert_eq!(vm.ro_data, vec![]);
        assert_eq!(vm.pc, DO_HEADER_LEN);
    }
}
