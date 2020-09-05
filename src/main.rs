use crate::compiler::Compiler;
use mrdovm::assemble_and_run;
use std::fs;
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
    repl_mode: Option<String>,
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

            assemble_and_run(&assembly);
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
