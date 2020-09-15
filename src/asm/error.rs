use crate::asm::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    ParseError { error: String },
    NoSectionDecl,
    MissingSection,
    StringConstantWithoutLabel { instr: String },
    SymbolAlreadyDeclared { name: String },
    InvalidDirectiveName { instr: String },
    UnknownDirective { name: String },
    UnknownSection { name: String },
    UnknownLabel { name: String },
    UnexpectedToken { token: Token },
    NotAnOpcode,
    EmptyString,
    UnlabeledString,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseError { ref error } => f.write_str(&format!("Parse error: {}", error)),
            Error::NoSectionDecl => f.write_str("No section declared"),
            Error::MissingSection => f.write_str("Missing section"),
            Error::StringConstantWithoutLabel { ref instr } => f.write_str(&format!(
                "String constant declared without label: {}",
                instr
            )),
            Error::SymbolAlreadyDeclared { ref name } => {
                f.write_str(&format!("Symbol {:?} declared multiple times", name))
            }
            Error::InvalidDirectiveName { ref instr } => {
                f.write_str(&format!("Invalid directive name: {}", instr))
            }
            Error::UnknownDirective { ref name } => {
                f.write_str(&format!("Unknown directive: {}", name))
            }
            Error::UnknownSection { ref name } => {
                f.write_str(&format!("Unknown section: {}", name))
            }
            Error::UnknownLabel { ref name } => f.write_str(&format!("Unknown label: {}", name)),
            Error::UnexpectedToken { ref token } => {
                f.write_str(&format!("Unexpected token {:?} in the bagging area", token))
            }
            Error::NotAnOpcode => f.write_str("Non-opcode found in opcode field"),
            Error::EmptyString => f.write_str("Empty string provided"),
            Error::UnlabeledString => f.write_str("Unlabeled string cannot be referenced"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::ParseError { .. } => "There was an error parsing the code",
            Error::NoSectionDecl => "No section declared",
            Error::MissingSection => "Missing section",
            Error::StringConstantWithoutLabel { .. } => "String constant declared without label",
            Error::SymbolAlreadyDeclared { .. } => "Symbol declared multiple times",
            Error::InvalidDirectiveName { .. } => "Invalid directive name",
            Error::UnknownDirective { .. } => "Unknown directive",
            Error::UnknownSection { .. } => "Unknown section",
            Error::UnknownLabel { .. } => "Unknown label",
            Error::UnexpectedToken { .. } => "Unexpected token",
            Error::NotAnOpcode { .. } => "Not an opcode",
            Error::EmptyString { .. } => "Empty string",
            Error::UnlabeledString { .. } => "Unlabeled string",
        }
    }
}
