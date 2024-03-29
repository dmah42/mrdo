use crate::asm::error::Error;
use crate::asm::instruction_parsers::Instruction;
use crate::asm::opcode::Opcode;
use crate::asm::program_parsers::{program, Program};
use crate::asm::symbols::{Symbol, Table, Type};

mod directive_parsers;
mod error;
mod instruction_parsers;
mod label_parsers;
mod opcode_parsers;
mod operand_parsers;
mod register_parsers;
mod symbols;

pub mod opcode;
pub mod program_parsers;
pub mod syscalls;

pub const DO_HEADER_PREFIX: [u8; 4] = [68, 79, 86, 77]; // "DOVM"
pub const DO_HEADER_LEN: usize = 32;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op { code: Opcode },
    IntRegister { idx: u8 },
    RealRegister { idx: u8 },
    VectorRegister { idx: u8 },
    Integer { value: i32 },
    Real { value: f64 },
    LabelDecl { name: String },
    LabelRef { name: String },
    Directive { name: String },
    DoString { value: String },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Phase {
    First,
    Second,
}

#[derive(Debug)]
pub struct Assembler {
    pub symbols: Table,
    readonly: Vec<u8>,
    phase: Phase,
    sections: Vec<Section>,
    current_section: Option<Section>,
    errors: Vec<Error>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            symbols: Table::new(),
            readonly: vec![],
            phase: Phase::First,
            sections: vec![],
            current_section: None,
            errors: vec![],
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<Error>> {
        match program(raw) {
            Ok((_remainder, program)) => {
                self.process_first(&program);

                if !self.errors.is_empty() {
                    return Err(self.errors.clone());
                }

                if self.sections.len() != 2 {
                    self.errors.push(Error::MissingSection);
                    return Err(self.errors.clone());
                }

                let body = self.process_second(&program);
                if let Err(e) = body {
                    self.errors.push(e);
                    return Err(self.errors.clone());
                }

                let mut assembled = self.write_header();
                assembled.append(&mut self.readonly);
                assembled.append(&mut body.unwrap());
                Ok(assembled)
            }
            Err(e) => {
                self.errors.push(Error::ParseError {
                    error: e.to_string(),
                });
                Err(self.errors.clone())
            }
        }
    }

    fn process_first(&mut self, p: &Program) {
        for i in &p.instructions {
            if i.is_comment() {
                continue;
            }

            if i.is_label() {
                if self.current_section.is_some() {
                    self.process_label_decl(i);
                } else {
                    self.errors.push(Error::NoSectionDecl);
                }
            }

            if i.is_directive() {
                self.process_directive(i);
            }
        }
        self.phase = Phase::Second;
    }

    fn process_label_decl(&mut self, i: &Instruction) {
        let name = match i.label_name() {
            Some(name) => name,
            None => {
                self.errors.push(Error::StringConstantWithoutLabel {
                    instr: i.to_string(),
                });
                return;
            }
        };

        if self.symbols.has(&name) {
            self.errors.push(Error::SymbolAlreadyDeclared { name });
            return;
        }

        self.symbols.add(Symbol::new(name, Type::Label));
    }

    // NOTE: public so the repl can do the right thing.
    pub fn process_second(&mut self, p: &Program) -> Result<Vec<u8>, Error> {
        let mut program = vec![];
        for i in &p.instructions {
            if i.is_comment() {
                continue;
            }
            if i.is_opcode() {
                program.append(&mut i.to_bytes(&self.symbols)?);
            }
            if i.is_directive() {
                self.process_directive(i);
            }
        }
        Ok(program)
    }

    fn process_directive(&mut self, i: &Instruction) {
        let name = match i.directive_name() {
            Some(name) => name,
            None => {
                self.errors.push(Error::InvalidDirectiveName {
                    instr: i.to_string(),
                });
                return;
            }
        };

        if i.has_operands() {
            match name.as_ref() {
                "str" => {
                    self.handle_str(i);
                }
                _ => {
                    self.errors.push(Error::UnknownDirective { name });
                }
            }
        } else {
            self.process_section_header(&name);
        }
    }

    fn process_section_header(&mut self, name: &str) {
        let section: Section = name.into();
        if section == Section::Unknown {
            self.errors.push(Error::UnknownSection {
                name: name.to_string(),
            });
            return;
        }
        self.sections.push(section.clone());
        self.current_section = Some(section);
    }

    fn handle_str(&mut self, i: &Instruction) {
        if self.phase != Phase::First {
            return;
        }

        match i.string_constant() {
            Some(s) => {
                match i.label_name() {
                    Some(name) => {
                        self.symbols.set_offset(&name, self.readonly.len() as u32);
                    }
                    None => {
                        self.errors.push(Error::UnlabeledString);
                        return;
                    }
                };
                for byte in s.as_bytes() {
                    self.readonly.push(*byte);
                }

                self.readonly.push(0);
            }
            None => {
                self.errors.push(Error::EmptyString);
            }
        }
    }

    fn write_header(&self) -> Vec<u8> {
        let mut header = vec![];
        for byte in &DO_HEADER_PREFIX {
            header.push(*byte);
        }
        let ro_len = self.readonly.len() as u32;
        header.push((ro_len >> 24) as u8);
        header.push((ro_len >> 16) as u8);
        header.push((ro_len >> 8) as u8);
        header.push(ro_len as u8);
        while header.len() < DO_HEADER_LEN {
            header.push(0);
        }
        header
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Assembler::new()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Section {
    Data { offset: Option<u32> },
    Code { offset: Option<u32> },
    Unknown,
}

impl Default for Section {
    fn default() -> Self {
        Section::Unknown
    }
}

impl From<&str> for Section {
    fn from(name: &str) -> Section {
        match name {
            "data" => Section::Data { offset: None },
            "code" => Section::Code { offset: None },
            _ => Section::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string =
            ".data\nhello: .str 'Hello'\n.code\nload $i0 #100\nload $r1 #1.4\ntest: add $i0 $i0 $r1\n";

        let result = asm.assemble(test_string);
        assert!(result.is_ok());

        let program = result.unwrap();
        assert_eq!(program.len(), 58);
    }

    #[test]
    fn test_symbol_table() {
        let mut sym = Table::new();
        let new_sym = Symbol::new("test".to_string(), Type::Label);
        sym.add(new_sym);
        sym.set_offset("test", 42);
        assert_eq!(sym.symbols.len(), 1);
        assert!(sym.has("test"));

        let v = sym.value("test");
        assert!(v.is_some());

        let v = v.unwrap();
        assert_eq!(v, 42);

        let v = sym.value("does not exist");
        assert!(v.is_none());
    }

    #[test]
    fn test_ro_data() {
        let mut asm = Assembler::new();
        let test = ".data\ntest: .str 'This is test'\n.code\n";
        let program = asm.assemble(test);
        assert!(program.is_ok());
    }

    #[test]
    fn test_bad_ro_data() {
        let mut asm = Assembler::new();
        let test = ".code\ntest: .str 'This is test'\n.wrong\n";
        let program = asm.assemble(test);
        assert!(program.is_err());
    }

    #[test]
    fn test_first_phase_no_segment() {
        let mut asm = Assembler::new();
        let test = "hello: .str 'fail'\n";
        let result = program(test);
        assert!(result.is_ok());
        let (_, p) = result.unwrap();
        asm.process_first(&p);
        assert_eq!(asm.errors.len(), 1);
    }

    #[test]
    fn test_first_phase_inside_segment() {
        let mut asm = Assembler::new();
        let test = ".data\ntest: .str 'Hello'\n";
        let result = program(test);
        assert!(result.is_ok());

        let (_, p) = result.unwrap();
        asm.process_first(&p);
        assert_eq!(asm.errors.len(), 0);
    }

    #[test]
    fn test_start_offset_written() {
        let mut asm = Assembler::new();
        let test = ".data\ntest: .str 'Hello'\n.code\nload $0 #100\nhlt\n";
        let program = asm.assemble(test);
        assert!(program.is_ok());
        assert_eq!(program.unwrap()[4..8], [0, 0, 0, 6]);
    }
}
