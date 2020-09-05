use crate::compiler::Compiler;
use mrdoasm::Assembler;
use mrdovm::run;
use std::fs;
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

pub mod compiler;
pub mod expression_parsers;
pub mod factor_parsers;
pub mod operand_parsers;
pub mod operator_parsers;
pub mod program_parser;
pub mod term_parsers;
pub mod tokens;
pub mod visitor;

#[derive(StructOpt)]
struct CLI {
    #[structopt(parse(from_os_str))]
    program: Option<std::path::PathBuf>,
    #[structopt(short)]
    repl_mode: Option<String>,
    #[structopt(short, parse(from_os_str))]
    output: Option<std::path::PathBuf>,
}

enum ReplMode {
    Assembler,
    HighLevel,
}

fn main() {
    let args = CLI::from_args();

    let mut compiler = Compiler::new();

    match args.program {
        Some(p) => {
            let source = read_file(&p);
            let assembly = compiler.compile(&source);

            println!("assembly\n{}\nEOF", assembly);
            let mut asm = Assembler::new();
            let bytecode = asm.assemble(&assembly);
            match bytecode {
                Ok(bc) => match args.output {
                    Some(o) => {
                        let mut f = File::create(o).expect("Unable to create file");
                        f.write_all(&bc).expect("Unable to write data");
                    }
                    None => {
                        if let Err(e) = run(&bc) {
                            println!("vm error: {:?}", e);
                            std::process::exit(1);
                        }
                    }
                },
                Err(e) => {
                    println!("assembler error: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
        None => {
            let repl_mode = match args.repl_mode {
                Some(mode) => match mode.as_str() {
                    "assembly" => ReplMode::Assembler,
                    "highlevel" => ReplMode::HighLevel,
                    _ => ReplMode::HighLevel,
                },
                None => ReplMode::HighLevel,
            };
            run_repl(repl_mode);
        }
    }
}

// TODO: add high-level REPL to mrdo.

fn read_file(tmp: &std::path::PathBuf) -> String {
    let contents = fs::read_to_string(tmp);
    match contents {
        Ok(_) => contents.unwrap(),
        Err(e) => {
            println!("Error reading file {:?}: {:?}", tmp.to_str(), e);
            std::process::exit(1);
        }
    }
}

// TODO:
fn run_repl(_mode: ReplMode) {}
