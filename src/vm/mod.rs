use crate::asm::opcode::Opcode;
use crate::asm::{DO_HEADER_LEN, DO_HEADER_PREFIX};
use crate::vm::error::Error;

use std::convert::{TryFrom, TryInto};
use std::default::Default;

pub mod error;

pub struct VM {
    pub iregisters: [i32; 32],
    pub rregisters: [f64; 32],
    pub vregisters: [Vec<f64>; 32],
    pub program: Vec<u8>,
    heap: Vec<u8>,
    pc: usize,
    pub ro_data: Vec<u8>,
}

pub fn is_valid_bytecode(bytecode: &[u8]) -> bool {
    bytecode.len() > DO_HEADER_LEN && bytecode[0..4] == DO_HEADER_PREFIX
}

// TODO: use from/into to convert a register into an value.
enum Register<'a> {
    I(i32),
    R(f64),
    V(&'a Vec<f64>),
}

impl TryInto<i32> for Register<'_> {
    type Error = Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Register::I(i) => Ok(i),
            Register::R(r) => Ok(r as i32),
            Register::V(_) => Err(Error::new("Cannot convert vector register into i32")),
        }
    }
}

impl TryInto<f64> for Register<'_> {
    type Error = Error;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Register::I(i) => Ok(i as f64),
            Register::R(r) => Ok(r),
            Register::V(_) => Err(Error::new("Cannot convert vector register into f64")),
        }
    }
}

impl<'a> TryInto<&'a Vec<f64>> for Register<'a> {
    type Error = Error;

    fn try_into(self) -> Result<&'a Vec<f64>, Self::Error> {
        match self {
            Register::I(_) => Err(Error::new("Cannot convert integer register into vector")),
            Register::R(_) => Err(Error::new("Cannot convert real register into vector")),
            Register::V(v) => Ok(v),
        }
    }
}

// TODO: test these
fn is_int_register(reg: u8) -> bool {
    !is_real_register(reg) && !is_vector_register(reg)
}

fn is_real_register(reg: u8) -> bool {
    (reg & 0b10000000) == 0b10000000
}

fn is_vector_register(reg: u8) -> bool {
    (reg & 0b01000000) == 0b01000000
}

fn idx_from_real_register(reg: u8) -> u8 {
    reg & 0b01111111
}

fn idx_from_vector_register(reg: u8) -> u8 {
    reg & 0b10111111
}

pub fn real_register_to_idx(reg: u8) -> u8 {
    reg | 0b10000000
}

pub fn vector_register_to_idx(reg: u8) -> u8 {
    reg | 0b01000000
}

impl VM {
    pub fn new() -> VM {
        VM {
            iregisters: Default::default(),
            rregisters: Default::default(),
            vregisters: Default::default(),
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
                // NOTE: `LOAD %1 10` will fail, as will `LOAD $1 3.14`, but in odd ways.
                let register = self.next_u8();
                match self.get_register(register)? {
                    Register::I(_) => self.iregisters[register as usize] = self.next_i32(),
                    Register::R(_) => {
                        self.rregisters[idx_from_real_register(register) as usize] = self.next_f64()
                    }
                    Register::V(_) => {
                        let base_addr = self.next_i32() as usize;
                        let len = self.next_i32() as usize;

                        let mut v = vec![];
                        let mut addr = base_addr;
                        while addr < base_addr + (len * 8) {
                            let bytes: [u8; 8] = self.heap[addr..(addr + 8)].try_into().unwrap();
                            v.push(f64::from_be_bytes(bytes));
                            addr += 8;
                        }

                        self.vregisters[idx_from_vector_register(register) as usize] = v;
                    }
                }
            }
            Opcode::LW => self.lw()?,
            Opcode::SW => self.sw()?,
            Opcode::ADD => self.add()?,
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
            Opcode::IGL => return Err(Error::new("Illegal opcode")),
        }
        Ok(false)
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::try_from(self.program[self.pc]).unwrap();
        self.pc += 1;
        opcode
    }

    fn get_register(&self, reg: u8) -> Result<Register, Error> {
        if is_int_register(reg) {
            return Ok(Register::I(self.iregisters[reg as usize]));
        }
        if is_real_register(reg) {
            return Ok(Register::R(
                self.rregisters[idx_from_real_register(reg) as usize],
            ));
        }
        if is_vector_register(reg) {
            return Ok(Register::V(
                &self.vregisters[idx_from_vector_register(reg) as usize],
            ));
        }

        return Err(Error::new(
            format!("Unknown register type {}", reg).as_str(),
        ));
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

    fn lw(&mut self) -> Result<(), Error> {
        let register = self.next_u8();
        if !is_int_register(register) {
            return Err(Error::new("Cannot load word into non-integer register"));
        }

        let address_reg = self.next_u8();
        if !is_int_register(address_reg) {
            return Err(Error::new("Cannot load word from non-integer address"));
        }

        let address = self.iregisters[address_reg as usize];
        if address < 0 {
            return Err(Error::new("Cannot load word from negative address offset"));
        }

        let address = address as usize;

        // TODO: convert from slice of heap.
        let bytes = [
            self.heap[address],
            self.heap[address + 1],
            self.heap[address + 2],
            self.heap[address + 3],
        ];

        self.iregisters[register as usize] = i32::from_be_bytes(bytes);
        Ok(())
    }

    fn sw(&mut self) -> Result<(), Error> {
        let address_reg = self.next_u8();
        if !is_int_register(address_reg) {
            return Err(Error::new("Cannot store word into non-integer address"));
        }
        let address = self.iregisters[address_reg as usize];
        if address < 0 {
            return Err(Error::new("Cannot store word to negative address offset"));
        }

        let address = address as usize;

        let register = self.next_u8();
        if !is_int_register(register) {
            return Err(Error::new("Cannot store word from non-integer register"));
        }

        let bytes = i32::to_be_bytes(self.iregisters[register as usize]);
        for i in 0..4 {
            self.heap[address + i] = bytes[0 + i];
        }
        Ok(())
    }

    fn add(&mut self) -> Result<(), Error> {
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
        // integer load
        let mut vm = VM::new();
        vm.program = vec![Opcode::LOAD as u8, 0, 0, 0, 1, 244];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 500);

        // real load
        let mut vm = VM::new();
        vm.program = vec![
            Opcode::LOAD as u8,
            real_register_to_idx(0),
            64,
            16,
            204,
            204,
            204,
            204,
            204,
            205,
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.rregisters[0], 4.2);

        // vector load
        let mut vm = VM::new();
        vm.heap = vec![
            64, 16, 204, 204, 204, 204, 204, 205, 64, 20, 204, 204, 204, 204, 204, 205,
        ];
        vm.program = vec![
            Opcode::LOAD as u8,
            vector_register_to_idx(0),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            2,
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.vregisters[0], vec![4.2, 5.2]);
    }

    #[test]
    fn test_opcode_lw() {
        let mut vm = VM::new();
        vm.heap = vec![0, 0, 0, 0, 0, 0, 0, 42];
        vm.iregisters[1] = 4;
        vm.program = vec![Opcode::LW as u8, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.iregisters[0], 42);

        let mut vm = VM::new();
        vm.heap = vec![0, 0, 0, 0, 0, 0, 0, 42];
        vm.rregisters[1] = 4.0;
        vm.program = vec![Opcode::LW as u8, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(!exit.is_ok());

        let mut vm = VM::new();
        vm.heap = vec![0, 0, 0, 0, 0, 0, 0, 42];
        vm.iregisters[1] = 4;
        vm.program = vec![Opcode::LW as u8, real_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(!exit.is_ok());
    }

    #[test]
    fn test_opcode_sw() {
        let mut vm = VM::new();
        vm.iregisters[0] = 0;
        vm.iregisters[1] = 42;
        vm.heap = vec![0, 0, 0, 0];
        vm.program = vec![Opcode::SW as u8, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.heap, vec![0, 0, 0, 42]);

        let mut vm = VM::new();
        vm.iregisters[1] = 42;
        vm.heap = vec![0, 0, 0, 0];
        vm.program = vec![Opcode::SW as u8, real_register_to_idx(0), 1];
        assert!(!vm.step().is_ok());

        let mut vm = VM::new();
        vm.iregisters[1] = -42;
        vm.heap = vec![0, 0, 0, 0];
        vm.program = vec![Opcode::SW as u8, 1, 0];
        assert!(!vm.step().is_ok());
    }

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
    }

    #[test]
    fn test_opcode_jmp() {
        let mut vm = VM::new();
        vm.program = vec![Opcode::JMP as u8, 0, 0, 0];
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
        vm.program = vec![Opcode::EQ as u8, 0, 0, 1];
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
        vm.program = vec![Opcode::NEQ as u8, 0, 0, 1];
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
        vm.program = vec![Opcode::GT as u8, 0, 0, 1];
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
        vm.program = vec![Opcode::LT as u8, 0, 0, 1];
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
        vm.program = vec![Opcode::GTE as u8, 0, 0, 1];
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
        vm.program = vec![Opcode::LTE as u8, 0, 0, 1];
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
        vm.program = vec![Opcode::JEQ as u8, 0, 1, 2];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.pc, 100);
    }

    #[test]
    fn test_opcode_alloc() {
        let mut vm = VM::new();
        vm.iregisters[0] = 1024;
        vm.program = vec![Opcode::ALLOC as u8, 0, 0, 0];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert_eq!(exit.unwrap(), false);
        assert_eq!(vm.heap.len(), 1024);
    }

    #[test]
    fn test_print_opcode() {
        let mut vm = VM::new();
        vm.ro_data.append(&mut vec![72, 101, 108, 108, 111, 0]);
        vm.program = vec![Opcode::PRINT as u8, 0, 0, 0];
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
