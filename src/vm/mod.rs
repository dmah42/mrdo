use crate::asm::opcode::Opcode;
use crate::asm::syscalls::Syscall;
use crate::asm::{DO_HEADER_LEN, DO_HEADER_PREFIX};
use crate::vm::error::Error;
use crate::vm::register::*;

use std::convert::{TryFrom, TryInto};
use std::default::Default;

mod arith_opcode;
mod compare_opcode;
mod error;
mod logic_opcode;
pub mod register;

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
            return Err(Error::new(
                format!("Invalid bytecode '{:#?}'", bytecode).as_str(),
            ));
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
        log::debug!("[vm] opcode {:?}", opcode);
        match opcode {
            Opcode::HLT => {
                return Ok(true);
            }
            Opcode::LOAD => {
                let register = self.next_u8();
                match self.get_register(register)? {
                    Register::I(_) => self.iregisters[register as usize] = self.next_i32(),
                    Register::R(_) => {
                        self.rregisters[idx_from_real_register(register) as usize] = self.next_f64()
                    }
                    Register::V(_) => {
                        let base_addr_reg = self.next_u8();
                        if !is_int_register(base_addr_reg) {
                            return Err(Error::new(
                                "Cannot load vector from non-integer base address",
                            ));
                        }
                        let base_addr = self.iregisters[base_addr_reg as usize];
                        if base_addr < 0 {
                            return Err(Error::new(
                                "Cannot load vector from negative base address",
                            ));
                        }

                        let base_addr = base_addr as usize;

                        let len = self.next_i32() as usize;
                        // println!("Loading {} bytes from {}", len, base_addr);

                        let mut v = vec![];
                        let mut addr = base_addr;
                        while addr < base_addr + len {
                            // println!(".. loading 8 bytes from {}", addr);
                            let bytes: [u8; 8] = self.heap[addr..(addr + 8)].try_into().unwrap();
                            v.push(f64::from_be_bytes(bytes));
                            addr += 8;
                        }

                        // println!("storing {:?} to vreg {}", v, register);

                        self.vregisters[idx_from_vector_register(register) as usize] = v;
                    }
                }
            }
            Opcode::COPY => {
                let dest_reg = self.next_u8();
                let src_reg = self.next_u8();

                match self.get_register(src_reg)? {
                    Register::I(si) => {
                        match self.get_register(dest_reg)? {
                            Register::I(_) => self.iregisters[dest_reg as usize] = si,
                            Register::R(_) => {
                                self.rregisters[idx_from_real_register(dest_reg) as usize] =
                                    si as f64
                            }
                            Register::V(_) => {
                                return Err(Error::new(
                                    "Cannot copy from int register to vector register",
                                ))
                            }
                        };
                    }
                    Register::R(sr) => {
                        match self.get_register(dest_reg)? {
                            Register::I(_) => {
                                if ((sr as i32) as f64) != sr {
                                    log::warn!("loss of precision copying {} from real register to integer register", sr)
                                }
                                self.iregisters[dest_reg as usize] = sr as i32;
                            }
                            Register::R(_) => {
                                self.rregisters[idx_from_real_register(dest_reg) as usize] = sr
                            }
                            Register::V(_) => {
                                return Err(Error::new(
                                    "Cannot copy from real register to vector register",
                                ))
                            }
                        };
                    }
                    Register::V(sv) => {
                        match self.get_register(dest_reg)? {
                            Register::I(_) => {
                                return Err(Error::new(
                                    "Cannot copy from vector register to int register",
                                ))
                            }
                            Register::R(_) => {
                                return Err(Error::new(
                                    "Cannot copy from vector register to real register",
                                ))
                            }
                            Register::V(_) => {
                                self.vregisters[idx_from_vector_register(dest_reg) as usize] = sv
                            }
                        };
                    }
                }

                // Throw away the last byte.
                self.next_u8();
            }
            Opcode::LW => self.lw()?,
            Opcode::SW => self.sw()?,
            Opcode::ADD => self.add()?,
            Opcode::SUB => self.sub()?,
            Opcode::MUL => self.mul()?,
            Opcode::DIV => self.div()?,
            Opcode::JMP => {
                let target = self.iregisters[self.next_u8() as usize];
                self.pc = target as usize;
            }
            Opcode::EQ => self.eq()?,
            Opcode::NEQ => self.neq()?,
            Opcode::GT => self.gt()?,
            Opcode::LT => self.lt()?,
            Opcode::GTE => self.gte()?,
            Opcode::LTE => self.lte()?,
            Opcode::JEQ => self.jeq()?,
            Opcode::AND => self.and()?,
            Opcode::OR => self.or()?,
            Opcode::NOT => self.not()?,
            Opcode::ALLOC => {
                let register = self.next_u8();
                if !is_int_register(register) {
                    return Err(Error::new(
                        "Cannot write heap location to non-integer register",
                    ));
                }
                let bytes = self.next_i32();
                if bytes < 0 {
                    return Err(Error::new("Cannot allocate negative number of bytes"));
                }
                self.iregisters[register as usize] = self.heap.len() as i32;
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            }
            Opcode::SYSCALL => {
                let call_idx = self.next_u8();
                if !is_int_register(call_idx) {
                    return Err(Error::new("Expected integer register for syscall"));
                }

                let call_num = self.iregisters[call_idx as usize];
                match Syscall::try_from(call_num) {
                    Ok(call) => {
                        match call {
                            Syscall::PrintReg => {
                                let reg_idx = self.next_u8();
                                let reg = self.get_register(reg_idx)?;
                                match reg {
                                    Register::I(i) => println!("{}", i),
                                    Register::R(r) => println!("{}", r),
                                    Register::V(v) => println!("{:?}", v),
                                };
                                // Swallow the remaining u8.
                                self.next_u8();
                            }
                            Syscall::PrintMem => return Err(Error::new("Unimplemented")),
                            Syscall::PrintStr => {
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
                        }
                    }
                    Err(_) => {
                        return Err(Error::new(format!("Unknown syscall {}", call_num).as_str()));
                    }
                }
            }
            Opcode::IGL => return Err(Error::new("Illegal opcode")),
        }
        Ok(false)
    }

    pub fn print_registers(&self) {
        println!("Listing integer registers:");
        for (i, reg) in self.iregisters.chunks(8).enumerate() {
            println!(
                "  [{}]\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i * 8,
                reg[0],
                reg[1],
                reg[2],
                reg[3],
                reg[4],
                reg[5],
                reg[6],
                reg[7]
            );
        }
        println!("EOF");

        println!("Listing real registers:");
        for (i, reg) in self.rregisters.chunks(8).enumerate() {
            println!(
                "  [{}]\t{:.03}\t{:.03}\t{:.03}\t{:.03}\t{:.03}\t{:.03}\t{:.03}\t{:.03}",
                i * 8,
                reg[0],
                reg[1],
                reg[2],
                reg[3],
                reg[4],
                reg[5],
                reg[6],
                reg[7],
            );
        }
        println!("EOF");

        println!("Listing (non-empty) vector registers:");
        for (i, reg) in self.vregisters.iter().enumerate() {
            if !reg.is_empty() {
                println!("  [{}]\t{:?}", i, reg);
            }
        }
        println!("EOF");
    }

    fn decode_opcode(&mut self) -> Opcode {
        let raw_opcode = self.program[self.pc];
        let opcode = Opcode::try_from(raw_opcode).unwrap();
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
                self.vregisters[idx_from_vector_register(reg) as usize].clone(),
            ));
        }

        Err(Error::new(
            format!("Unknown register type {}", reg).as_str(),
        ))
    }

    fn next_u8(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_u16(&mut self) -> u16 {
        let bytes: [u8; 2] = [self.program[self.pc], self.program[self.pc + 1]];
        self.pc += 2;
        u16::from_be_bytes(bytes)
    }

    fn next_i32(&mut self) -> i32 {
        let bytes: [u8; 4] = self.program[self.pc..self.pc + 4].try_into().unwrap();
        self.pc += 4;
        i32::from_be_bytes(bytes)
    }

    fn next_f64(&mut self) -> f64 {
        let bytes: [u8; 8] = self.program[self.pc..self.pc + 8].try_into().unwrap();
        self.pc += 8;
        f64::from_be_bytes(bytes)
    }

    fn lw(&mut self) -> Result<(), Error> {
        let register = self.next_u8();

        let address_reg = self.next_u8();
        if !is_int_register(address_reg) {
            return Err(Error::new("Cannot load word from non-integer address"));
        }

        let address = self.iregisters[address_reg as usize];
        if address < 0 {
            return Err(Error::new("Cannot load word from negative address offset"));
        }

        let address = address as usize;

        match self.get_register(register)? {
            Register::I(_) => {
                let bytes: [u8; 4] = self.heap[address..address + 4].try_into().unwrap();

                self.iregisters[register as usize] = i32::from_be_bytes(bytes);
            }
            Register::R(_) => {
                let bytes: [u8; 8] = self.heap[address..address + 8].try_into().unwrap();
                self.rregisters[idx_from_real_register(register) as usize] =
                    f64::from_be_bytes(bytes);
            }
            Register::V(_) => return Err(Error::new("Cannot load word into vector register")),
        }

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

        let reg_idx = self.next_u8();
        let reg = self.get_register(reg_idx)?;
        let bytes = match reg {
            Register::I(i) => i32::to_be_bytes(i).to_vec(),
            Register::R(r) => f64::to_be_bytes(r).to_vec(),
            Register::V(_) => {
                return Err(Error::new("Cannot store word from vector register"));
            }
        };

        for (i, b) in bytes.iter().enumerate() {
            self.heap[address + i] = *b;
        }

        // swallow the next byte.
        self.next_u8();

        Ok(())
    }

    fn jeq(&mut self) -> Result<(), Error> {
        let register = self.next_u8();
        if !is_int_register(register) {
            return Err(Error::new("Cannot jump to non-integer location"));
        }
        let target = self.iregisters[register as usize];

        let a_idx = self.next_u8();
        let b_idx = self.next_u8();

        if self.are_register_contents_equal(a_idx, b_idx)? {
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

    use assert_approx_eq::assert_approx_eq;

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
        assert!(exit.unwrap());
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        // integer load
        let mut vm = VM::new();
        vm.program = vec![Opcode::LOAD as u8, 0, 0, 0, 1, 244];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
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
        assert!(!exit.unwrap());
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
            16,
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.vregisters[0], vec![4.2, 5.2]);
    }

    #[test]
    fn test_opcode_copy() {
        // int to int
        let mut vm = VM::new();
        vm.iregisters[1] = 42;
        vm.program = vec![Opcode::COPY as u8, 0, 1, 0];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 42);

        // int to real
        let mut vm = VM::new();
        vm.iregisters[1] = 42;
        vm.program = vec![Opcode::COPY as u8, real_register_to_idx(0), 1, 0];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.rregisters[0], 42.0);

        // int to vector
        let mut vm = VM::new();
        vm.iregisters[1] = 42;
        vm.program = vec![Opcode::COPY as u8, vector_register_to_idx(0), 1, 0];
        let exit = vm.step();
        assert!(exit.is_err());

        // real to int
        let mut vm = VM::new();
        vm.rregisters[1] = 42.0;
        vm.program = vec![Opcode::COPY as u8, 0, real_register_to_idx(1), 0];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 42);

        // real to real
        let mut vm = VM::new();
        vm.rregisters[1] = 42.0;
        vm.program = vec![
            Opcode::COPY as u8,
            real_register_to_idx(0),
            real_register_to_idx(1),
            0,
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.rregisters[0], 42.0);

        // real to vector
        let mut vm = VM::new();
        vm.rregisters[1] = 42.0;
        vm.program = vec![
            Opcode::COPY as u8,
            vector_register_to_idx(0),
            real_register_to_idx(1),
            0,
        ];
        let exit = vm.step();
        assert!(exit.is_err());

        // vector to int
        let mut vm = VM::new();
        vm.vregisters[1] = vec![42.0];
        vm.program = vec![Opcode::COPY as u8, 0, vector_register_to_idx(1), 0];
        let exit = vm.step();
        assert!(exit.is_err());

        // vector to real
        let mut vm = VM::new();
        vm.vregisters[1] = vec![42.0];
        vm.program = vec![
            Opcode::COPY as u8,
            real_register_to_idx(0),
            vector_register_to_idx(1),
            0,
        ];
        let exit = vm.step();
        assert!(exit.is_err());

        // vector to vector
        let mut vm = VM::new();
        vm.vregisters[1] = vec![42.0];
        vm.program = vec![
            Opcode::COPY as u8,
            vector_register_to_idx(0),
            vector_register_to_idx(1),
            0,
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.vregisters[0], vec![42.0]);
    }

    #[test]
    fn test_opcode_lw() {
        let mut vm = VM::new();
        vm.heap = vec![0, 0, 0, 0, 0, 0, 0, 42];
        vm.iregisters[1] = 4;
        vm.program = vec![Opcode::LW as u8, 0, 1];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.iregisters[0], 42);

        let mut vm = VM::new();
        vm.heap = vec![64, 16, 204, 204, 204, 204, 204, 255];
        vm.iregisters[0] = 0;
        vm.program = vec![Opcode::LW as u8, real_register_to_idx(0), 0];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_approx_eq!(vm.rregisters[0], 4.2);

        let mut vm = VM::new();
        vm.heap = vec![0, 0, 0, 0, 0, 0, 0, 42];
        vm.rregisters[1] = 4.0;
        vm.program = vec![Opcode::LW as u8, 0, real_register_to_idx(1)];
        let exit = vm.step();
        assert!(exit.is_err());

        let mut vm = VM::new();
        vm.heap = vec![0, 0, 0, 42];
        vm.iregisters[1] = 0;
        vm.program = vec![Opcode::LW as u8, vector_register_to_idx(0), 1];
        let exit = vm.step();
        assert!(exit.is_err());
    }

    #[test]
    fn test_opcode_sw() {
        let mut vm = VM::new();
        vm.iregisters[0] = 0;
        vm.iregisters[1] = 42;
        vm.heap = vec![0, 0, 0, 0];
        vm.program = vec![Opcode::SW as u8, 0, 1, 0];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.heap, vec![0, 0, 0, 42]);

        let mut vm = VM::new();
        vm.iregisters[1] = 42;
        vm.heap = vec![0, 0, 0, 0];
        vm.program = vec![Opcode::SW as u8, real_register_to_idx(0), 1, 0];
        assert!(vm.step().is_err());

        let mut vm = VM::new();
        vm.iregisters[1] = -42;
        vm.heap = vec![0, 0, 0, 0];
        vm.program = vec![Opcode::SW as u8, 1, 0, 0];
        assert!(vm.step().is_err());
    }

    #[test]
    fn test_opcode_jmp() {
        let mut vm = VM::new();
        vm.program = vec![Opcode::JMP as u8, 0, 0, 0];
        vm.iregisters[0] = 100;
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.pc, 100);
    }

    #[test]
    fn test_opcode_jeq_int_registers() {
        let mut vm = VM::new();
        vm.iregisters[0] = 100;
        vm.iregisters[1] = 3;
        vm.iregisters[2] = 3;
        vm.program = vec![Opcode::JEQ as u8, 0, 1, 2];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.pc, 100);

        vm = VM::new();
        vm.iregisters[0] = 200;
        vm.iregisters[1] = 3;
        vm.iregisters[2] = 5;
        vm.program = vec![Opcode::JEQ as u8, 0, 1, 2];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_opcode_jeq_real_registers() {
        let mut vm = VM::new();
        vm.iregisters[0] = 100;
        vm.rregisters[1] = 3.4;
        vm.rregisters[2] = 3.4;
        vm.program = vec![
            Opcode::JEQ as u8,
            0,
            real_register_to_idx(1),
            real_register_to_idx(2),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.pc, 100);

        vm = VM::new();
        vm.iregisters[0] = 200;
        vm.rregisters[1] = 3.4;
        vm.rregisters[2] = 3.2;
        vm.program = vec![
            Opcode::JEQ as u8,
            0,
            real_register_to_idx(1),
            real_register_to_idx(2),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_opcode_jeq_vector_registers() {
        let mut vm = VM::new();
        vm.iregisters[0] = 100;
        vm.vregisters[1] = vec![3.4, 5.2];
        vm.vregisters[2] = vec![3.4, 5.2];
        vm.program = vec![
            Opcode::JEQ as u8,
            0,
            vector_register_to_idx(1),
            vector_register_to_idx(2),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.pc, 100);

        vm = VM::new();
        vm.iregisters[0] = 200;
        vm.vregisters[1] = vec![3.4, 5.2];
        vm.vregisters[2] = vec![3.4, 5.1];
        vm.program = vec![
            Opcode::JEQ as u8,
            0,
            vector_register_to_idx(1),
            vector_register_to_idx(2),
        ];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_opcode_alloc() {
        let mut vm = VM::new();
        vm.program = vec![Opcode::ALLOC as u8, 0, 0, 0, 0, 120];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
        assert_eq!(vm.heap.len(), 120);

        let mut vm = VM::new();
        vm.program = vec![Opcode::ALLOC as u8, 0, 220, 0, 0, 0];
        let exit = vm.step();
        assert!(exit.is_err());
    }

    #[test]
    fn test_opcode_syscall_printstr() {
        let mut vm = VM::new();
        vm.ro_data.append(&mut vec![72, 101, 108, 108, 111, 0]);
        vm.program = vec![Opcode::SYSCALL as u8, Syscall::PrintStr as u8, 0, 0];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
    }

    #[test]
    fn test_opcode_syscall_printreg() {
        let mut vm = VM::new();
        vm.rregisters[0] = 42.0;
        vm.program = vec![Opcode::SYSCALL as u8, Syscall::PrintReg as u8, 0, 0];
        let exit = vm.step();
        assert!(exit.is_ok());
        assert!(!exit.unwrap());
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
            bytecode.push(0);
        }
        bytecode.append(&mut vec![1, 2, 3, 4]);
        let result = vm.set_bytecode(&bytecode);
        assert!(result.is_ok());
        assert_eq!(vm.ro_data, vec![]);
        assert_eq!(vm.pc, DO_HEADER_LEN);
    }
}
