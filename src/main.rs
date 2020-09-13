#[macro_use]
extern crate nom;

use crate::asm::Assembler;
use crate::compiler::Compiler;
use crate::repl::REPL;
use crate::vm::{is_valid_bytecode, VM};

use std::fs;
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

pub mod asm;
pub mod compiler;
pub mod repl;
pub mod vm;

#[derive(StructOpt)]
struct CLI {
    #[structopt(parse(from_os_str))]
    program: Option<std::path::PathBuf>,
    #[structopt(short, long, parse(from_os_str))]
    output: Option<std::path::PathBuf>,
    #[structopt(short, long)]
    threads: Option<u32>,
}

fn main() {
    let args = CLI::from_args();

    match args.program {
        Some(p) => {
            let bytecode = read_bytecode(&p);
            let bc = match bytecode {
                Some(bc) => bc,
                None => compile(&p, args.output),
            };
            run_bytecode(&bc);
        }
        None => run_repl(),
    }
}

fn read_bytecode(tmp: &std::path::PathBuf) -> Option<Vec<u8>> {
    let bytecode = fs::read(&tmp).unwrap();

    match is_valid_bytecode(&bytecode) {
        true => Some(bytecode),
        false => None,
    }
}

fn read_assembly(tmp: &std::path::PathBuf) -> String {
    let contents = fs::read_to_string(tmp);
    match contents {
        Ok(_) => contents.unwrap(),
        Err(e) => {
            println!("Error reading file {:?}: {:?}", tmp.to_str(), e);
            std::process::exit(1);
        }
    }
}

fn run_repl() {
    let mut repl = REPL::new();
    repl.run();
}

fn run_bytecode(bytecode: &[u8]) {
    let mut vm = VM::new();
    if let Err(e) = vm.set_bytecode(&bytecode) {
        println!("vmerror: {}", e);
    }

    println!("Listing readonly:");
    for data in vm.ro_data.chunks(4) {
        println!("  {:?}", data);
    }
    println!("EOF");

    println!("Listing instructions:");
    for instr in vm.program.chunks(4) {
        println!("  {:?}", instr);
    }
    println!("EOF");

    let result = vm.run();
    match result {
        Ok(_) => {
            println!("Listing integer registers:");
            for (i, reg) in vm.iregisters.chunks(4).enumerate() {
                println!(
                    "  [{}]\t{}\t{}\t{}\t{}",
                    i * 4,
                    reg[0],
                    reg[1],
                    reg[2],
                    reg[3]
                );
            }
            println!("EOF");

            println!("Listing real registers:");
            for (i, reg) in vm.rregisters.chunks(4).enumerate() {
                println!(
                    "  [{}]\t{:.03}\t{:.03}\t{:.03}\t{:.03}",
                    i * 4,
                    reg[0],
                    reg[1],
                    reg[2],
                    reg[3]
                );
            }
            println!("EOF");
            std::process::exit(0);
        }
        Err(e) => {
            println!("vmerror: {}", e);
            std::process::exit(1);
        }
    }
}

fn compile(assembly: &std::path::PathBuf, output: Option<std::path::PathBuf>) -> Vec<u8> {
    let mut compiler = Compiler::new();

    let source = read_assembly(&assembly);
    let assembly = compiler.compile(&source);

    println!("assembly\n{}\nEOF", assembly);
    let mut asm = Assembler::new();
    let bytecode = asm.assemble(&assembly);
    match bytecode {
        Ok(bc) => {
            if let Some(o) = output {
                let mut f = File::create(o).expect("Unable to create file");
                f.write_all(&bc).expect("Unable to write data");
            };
            bc
        }
        Err(e) => {
            println!("assembler error: {:?}", e);
            std::process::exit(1);
        }
    }
}
