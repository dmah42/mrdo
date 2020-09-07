use crate::asm::program_parsers::program;
use crate::asm::Assembler;
use crate::repl::command_parser::CommandParser;
use crate::vm::VM;

use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

pub mod command_parser;

const INFO_TAG: char = '\u{29FB}';
const WARN_TAG: char = '\u{26A0}';
const ERROR_TAG: char = '\u{2620}';

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
    asm: Assembler,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
            asm: Assembler::new(),
        }
    }

    pub fn run(&mut self) {
        println!("{} What will you DO today?", INFO_TAG);

        loop {
            let mut buffer = String::new();

            print!("\u{21AA} ");
            io::stdout().flush().expect("Unable to flush stdout");

            io::stdin()
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            if buffer.starts_with(':') {
                self.execute_command(&buffer);
            } else {
                let program = match program(buffer.into()) {
                    Ok((_, program)) => program,
                    Err(e) => {
                        println!("{} Unable to parse input: {}", WARN_TAG, e);
                        continue;
                    }
                };
                self.vm
                    .program
                    .append(&mut program.to_bytes(&self.asm.symbols));
                if let Err(e) = self.vm.step() {
                    println!("{} {}", ERROR_TAG, e);
                    continue;
                }
            }
        }
    }

    fn execute_command(&mut self, input: &str) {
        let args = CommandParser::tokenize(input);
        match args[0] {
            ":c" => self.vm = VM::new(),
            ":history" => {
                for command in &self.command_buffer {
                    println!("  {}", command);
                }
            }
            ":list" => self.list_program(),
            ":q" => {
                println!("{} Buh-bye!", INFO_TAG);
                std::process::exit(0);
            }
            ":r" => self.list_registers(),
            ":s" => {
                println!("{} Listing symbols:", INFO_TAG);
                println!("{:#?}", self.asm.symbols);
                println!("{} EOF", INFO_TAG);
            }
            ":load" => self.load_file(&args[1..]),
            _ => println!("Invalid command: '{}'", input),
        }
    }

    fn list_program(&self) {
        println!("{} Listing instructions:", INFO_TAG);
        for instr in self.vm.program.chunks(4) {
            println!("  {:?}", instr);
        }
        println!("{} EOF", INFO_TAG);
    }

    fn list_registers(&self) {
        println!("{} Listing integer registers:", INFO_TAG);
        for reg in self.vm.iregisters.chunks(4) {
            println!("  {}\t{}\t{}\t{}", reg[0], reg[1], reg[2], reg[3]);
        }
        println!("{} EOF", INFO_TAG);

        println!("{} Listing real registers:", INFO_TAG);
        for reg in self.vm.rregisters.chunks(4) {
            println!("  {}\t{}\t{}\t{}", reg[0], reg[1], reg[2], reg[3]);
        }
        println!("{} EOF", INFO_TAG);
    }

    fn load_file(&mut self, args: &[&str]) {
        // TODO: merge with `main.rs` file reading.
        let mut tmp = String::new();
        if args.is_empty() {
            print!("{} Please enter the path to the file: ", INFO_TAG);
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut tmp)
                .expect("Failed to read line from user");
            tmp = tmp.trim().to_string();
        } else {
            tmp = args[1].to_string();
        }
        let path = Path::new(&tmp);
        println!(
            "{} Loading program from '{}'",
            INFO_TAG,
            path.to_str().unwrap()
        );
        let result = fs::read_to_string(path);
        let contents = match result {
            Ok(contents) => contents,
            Err(e) => {
                println!("{} Error loading that file: {:?}", ERROR_TAG, e);
                return;
            }
        };

        match self.asm.assemble(&contents) {
            Ok(assembled) => {
                println!("{} Sending program to VM", INFO_TAG);
                self.vm = VM::new();
                self.vm.set_bytecode(&assembled).unwrap();
                self.list_program();
                let result = self.vm.run();
                if result.is_err() {
                    println!("{} Runtime error: {}", ERROR_TAG, result.unwrap_err());
                }
            }
            Err(errors) => {
                println!("{} Errors encountered while assembling:", ERROR_TAG);
                for error in errors {
                    println!("  {}", error);
                }
            }
        }
    }
}

impl Default for REPL {
    fn default() -> Self {
        REPL::new()
    }
}
