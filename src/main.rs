use crate::asm::Assembler;
use crate::compiler::Compiler;
use crate::repl::REPL;
use crate::vm::{is_valid_bytecode, VM};

use clap::Parser;
use log::LevelFilter;
use std::fs;
use std::fs::File;
use std::io::Write;

pub mod asm;
pub mod compiler;
pub mod repl;
pub mod vm;

#[derive(Parser)]
#[command(name = "mrdo")]
#[command(bin_name = "mrdo")]
enum Cli {
    Args(Args),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_hint = clap::ValueHint::FilePath, value_name = "INPUT_FILE")]
    program: Option<std::path::PathBuf>,

    #[arg(short, long, value_hint = clap::ValueHint::FilePath, value_name = "OUTPUT_FILE")]
    output: Option<std::path::PathBuf>,

    #[arg(short = 'd', long)]
    debug: bool,

    #[arg(short = 'a', long)]
    list_asm: bool,

    #[arg(short = 'b', long)]
    list_bc: bool,

    #[arg(short = 'r', long)]
    list_reg: bool,
    // TODO: implement this.
    //#[structopt(short, long)]
    //threads: Option<u32>,
}

fn main() {
    let Cli::Args(args) = Cli::parse();

    if args.debug {
        pretty_env_logger::formatted_timed_builder()
            .filter_level(LevelFilter::Debug)
            .init();
    } else {
        pretty_env_logger::formatted_timed_builder()
            .parse_default_env()
            .init();
    }

    match args.program {
        Some(p) => {
            let bytecode = read_bytecode(&p);
            let bc = match bytecode {
                Some(bc) => bc,
                None => compile(&p, args.output, args.list_asm),
            };
            log::info!("Running...");
            run_bytecode(&bc, args.list_bc, args.list_reg);
        }
        None => run_repl(),
    }
}

fn read_bytecode(tmp: &std::path::PathBuf) -> Option<Vec<u8>> {
    let bytecode = fs::read(tmp).unwrap();

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
    if let Err(e) = vm.set_bytecode(bytecode) {
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
                vm.print_registers();
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
    log::info!("Compiling...");
    let mut compiler = Compiler::new();

    let source = read_assembly(assembly);
    let assembly = compiler.compile(&source);
    if let Err(e) = assembly {
        log::error!("compiler error: {}", e);
        std::process::exit(1);
    }

    let assembly = assembly.unwrap();

    if list_asm {
        println!("assembly\n{}\nEOF", assembly);
    }
    log::info!("Assembling...");
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
            log::error!("assembler error: {:?}", e);
            std::process::exit(1);
        }
    }
}
