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

    #[structopt(short("a"), long)]
    list_asm: bool,

    #[structopt(short("b"), long)]
    list_bc: bool,

    #[structopt(short("r"), long)]
    list_reg: bool,
    // TODO: implement this.
    //#[structopt(short, long)]
    //threads: Option<u32>,
}

fn main() {
    let args = CLI::from_args();

    match args.program {
        Some(p) => {
            let bytecode = read_bytecode(&p);
            let bc = match bytecode {
                Some(bc) => bc,
                None => compile(&p, args.output, args.list_asm),
            };
            run_bytecode(&bc, args.list_bc, args.list_reg);
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

fn run_bytecode(bytecode: &[u8], list_bc: bool, list_reg: bool) {
    let mut vm = VM::new();
    if let Err(e) = vm.set_bytecode(&bytecode) {
        println!("vmerror: {}", e);
    }

    if list_bc {
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
    }

    let result = vm.run();
    match result {
        Ok(_) => {
            if list_reg {
                // TODO: move this to repl and reuse it here.
                println!("Listing integer registers:");
                for (i, reg) in vm.iregisters.chunks(8).enumerate() {
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
                for (i, reg) in vm.rregisters.chunks(8).enumerate() {
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
                for (i, reg) in vm.vregisters.iter().enumerate() {
                    if !reg.is_empty() {
                        println!("  [{}]\t{:?}", i, reg);
                    }
                }
                println!("EOF");
            }
            std::process::exit(0);
        }
        Err(e) => {
            println!("vmerror: {}", e);
            std::process::exit(1);
        }
    }
}

fn compile(
    assembly: &std::path::PathBuf,
    output: Option<std::path::PathBuf>,
    list_asm: bool,
) -> Vec<u8> {
    let mut compiler = Compiler::new();

    let source = read_assembly(&assembly);
    let assembly = compiler.compile(&source);
    if let Err(e) = assembly {
        println!("compiler error: {}", e);
        std::process::exit(1);
    }

    let assembly = assembly.unwrap();

    if list_asm {
        println!("assembly\n{}\nEOF", assembly);
    }
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
