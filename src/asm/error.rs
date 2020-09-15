use crate::asm::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AsmError {
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

impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AsmError::ParseError { ref error } => f.write_str(&format!("Parse error: {}", error)),
            AsmError::NoSectionDecl => f.write_str("No section declared"),
            AsmError::MissingSection => f.write_str("Missing section"),
            AsmError::StringConstantWithoutLabel { ref instr } => f.write_str(&format!(
                "String constant declared without label: {}",
                instr
            )),
            AsmError::SymbolAlreadyDeclared { ref name } => {
                f.write_str(&format!("Symbol {:?} declared multiple times", name))
            }
            AsmError::InvalidDirectiveName { ref instr } => {
                f.write_str(&format!("Invalid directive name: {}", instr))
            }
            AsmError::UnknownDirective { ref name } => {
                f.write_str(&format!("Unknown directive: {}", name))
            }
            AsmError::UnknownSection { ref name } => {
                f.write_str(&format!("Unknown section: {}", name))
            }
            AsmError::UnknownLabel { ref name } => f.write_str(&format!("Unknown label: {}", name)),
            AsmError::UnexpectedToken { ref token } => {
                f.write_str(&format!("Unexpected token {:?} in the bagging area", token))
            }
            AsmError::NotAnOpcode => f.write_str("Non-opcode found in opcode field"),
            AsmError::EmptyString => f.write_str("Empty string provided"),
            AsmError::UnlabeledString => f.write_str("Unlabeled string cannot be referenced"),
        }
    }
}

impl std::error::Error for AsmError {
    fn description(&self) -> &str {
        match self {
            AsmError::ParseError { .. } => "There was an error parsing the code",
            AsmError::NoSectionDecl => "No section declared",
            AsmError::MissingSection => "Missing section",
            AsmError::StringConstantWithoutLabel { .. } => "String constant declared without label",
            AsmError::SymbolAlreadyDeclared { .. } => "Symbol declared multiple times",
            AsmError::InvalidDirectiveName { .. } => "Invalid directive name",
            AsmError::UnknownDirective { .. } => "Unknown directive",
            AsmError::UnknownSection { .. } => "Unknown section",
            AsmError::UnknownLabel { .. } => "Unknown label",
            AsmError::UnexpectedToken { .. } => "Unexpected token",
            AsmError::NotAnOpcode { .. } => "Not an opcode",
            AsmError::EmptyString { .. } => "Empty string",
            AsmError::UnlabeledString { .. } => "Unlabeled string",
        }
    }
}
