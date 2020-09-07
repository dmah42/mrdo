use crate::vm::error::Error;
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt)]
struct CLI {
    #[structopt(parse(from_os_str))]
    program: Option<std::path::PathBuf>,
    #[structopt(short, long)]
    threads: Option<u32>,
}

fn main() {
    let args = CLI::from_args();

    match args.program {
        Some(p) => {
            let bytecode = fs::read(&p).unwrap();
            run(&bytecode).unwrap();
        }
        None => {
            let mut repl = repl::REPL::new();
            repl.run();
        }
    }
}

pub fn run(bytecode: &[u8]) -> Result<(), Error> {
    let mut vm = vm::VM::new();
    vm.set_bytecode(&bytecode)?;

    println!("Listing instructions:");
    for instr in vm.program.chunks(4) {
        println!("  {:?}", instr);
    }
    println!("EOF");

    let result = vm.run();
    match result {
        Ok(_) => {
            println!("Listing integer registers:");
            for reg in vm.iregisters.chunks(4) {
                println!("  {}\t{}\t{}\t{}", reg[0], reg[1], reg[2], reg[3]);
            }
            println!("EOF");

            println!("Listing real registers:");
            for reg in vm.rregisters.chunks(4) {
                println!("  {}\t{}\t{}\t{}", reg[0], reg[1], reg[2], reg[3]);
            }
            println!("EOF");
            std::process::exit(0);
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
