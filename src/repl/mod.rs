use crate::asm::program_parsers::program;
use crate::asm::Assembler;
use crate::compiler::Compiler;
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

#[derive(Debug)]
enum Mode {
    Assembly,
    Highlevel,
}

pub struct REPL {
    assembly_buffer: Vec<String>,
    command_buffer: Vec<String>,
    compiler: Compiler,
    asm: Assembler,
    vm: VM,
    mode: Mode,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            assembly_buffer: vec![],
            command_buffer: vec![],
            compiler: Compiler::new(),
            asm: Assembler::new(),
            vm: VM::new(),
            mode: Mode::Highlevel,
        }
    }

    pub fn run(&mut self) {
        println!("{} What will you DO today?", INFO_TAG);
        println!("{} (':h' for help)", INFO_TAG);

        loop {
            let mut buffer = String::new();

            print!("\u{21AA} ");
            io::stdout().flush().expect("Unable to flush stdout");

            io::stdin()
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            //let buffer = buffer.trim();

            self.command_buffer.push(buffer.clone());

            if buffer.starts_with(':') {
                self.execute_command(buffer.as_str());
            } else {
                let assembly = match self.mode {
                    Mode::Assembly => vec![buffer],
                    Mode::Highlevel => match self.compiler.compile_expr(buffer.as_str()) {
                        Ok(compiled) => compiled.to_vec(),
                        Err(e) => {
                            println!("{} Unable to compile input: {}", ERROR_TAG, e);
                            continue;
                        }
                    },
                };
                self.assembly_buffer.append(&mut assembly.clone());
                let assembled = assembly.join("\n");
                println!("assembled: '{}'", assembled);
                let bytecode = match program(&assembled) {
                    Ok((_, prog)) => self.asm.process_second(&prog),
                    Err(e) => {
                        println!("{} Unable to parse input: {}", WARN_TAG, e);
                        continue;
                    }
                };
                if let Err(e) = bytecode {
                    println!("{} {}", ERROR_TAG, e);
                    continue;
                }
                self.vm.program.append(&mut bytecode.unwrap());
                println!("vm program: {:#?}", self.vm.program);
                for _i in 1..=assembly.len() {
                    if let Err(e) = self.vm.step() {
                        println!("{} {}", ERROR_TAG, e);
                        continue;
                    }
                }
            }
        }
    }

    fn execute_command(&mut self, input: &str) {
        let args = CommandParser::tokenize(input);
        match args[0] {
            ":c" => self.vm = VM::new(),
            ":h" => self.print_help(),
            ":history" => {
                for command in &self.command_buffer {
                    println!("  {}", command);
                }
            }
            ":list" => self.list_program(&args[1..]),
            ":load" => self.load_file(&args[1..]),
            ":m" => self.toggle_mode(),
            ":q" => {
                println!("{} Buh-bye!", INFO_TAG);
                std::process::exit(0);
            }
            ":r" => self.vm.print_registers(),
            ":s" => {
                println!("{} Listing symbols:", INFO_TAG);
                println!("{:#?}", self.asm.symbols);
                println!("{} EOF", INFO_TAG);
            }
            _ => println!("{} Invalid command: '{}'", ERROR_TAG, input),
        }
    }

    fn print_help(&self) {
        println!(":c clears the VM");
        println!(":h prints this help");
        println!(":history shows the command buffer");
        println!(":list [asm|bc] lists the current program");
        println!(":load loads a program from the given path");
        println!(":m toggles the mode between assembly and high-level language");
        println!(":q quits");
        println!(":r prints the registers");
        println!(":s lists the known symbols");
    }

    fn list_program(&self, args: &[&str]) {
        if args.is_empty() || args[0] == "asm" {
            println!("{} Listing assembly:", INFO_TAG);
            for line in &self.assembly_buffer {
                println!("  {}", line);
            }
            println!("{} EOF", INFO_TAG);
        } else if args[0] == "bc" {
            println!("{} Listing bytecode:", INFO_TAG);
            for instr in self.vm.program.chunks(4) {
                println!("  {:?}", instr);
            }
            println!("{} EOF", INFO_TAG);
        }
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
                self.list_program(&[]);
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

    fn toggle_mode(&mut self) {
        match self.mode {
            Mode::Highlevel => self.mode = Mode::Assembly,
            Mode::Assembly => self.mode = Mode::Highlevel,
        };
        println!("{} Mode is now {:?}", INFO_TAG, self.mode);
    }
}

impl Default for REPL {
    fn default() -> Self {
        REPL::new()
    }
}
