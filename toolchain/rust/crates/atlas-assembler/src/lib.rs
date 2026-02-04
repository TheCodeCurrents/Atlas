pub mod lexer;
pub mod parser;
pub mod error;

pub use lexer::Lexer;
pub use parser::Parser;
pub use error::AssemblerError;

use std::fs::{self, File};
use std::io::Write;

pub fn assemble(src: &str, output: &str) -> Result<(), AssemblerError> {
    let source = fs::read_to_string(src).map_err(|e| AssemblerError::IoError {
        operation: format!("Failed to read input file '{}'", src),
        source: e,
    })?;
    
    // Create parser (which includes lexer)
    let parser = Parser::new(&source);
    
    // Collect all parsed instructions
    let mut instructions: Vec<_> = Vec::new();
    for result in parser {
        instructions.push(result?);
    }
    
    // Encode each instruction to bytes
    let mut bytes: Vec<u8> = Vec::new();
    for instr in instructions {
        let encoded = instr.encode()?;
        // Convert u16 to bytes (big-endian)
        bytes.extend_from_slice(&encoded.to_be_bytes());
    }
    
    // Write to output file
    let mut file = File::create(output).map_err(|e| AssemblerError::IoError {
        operation: format!("Failed to create output file '{}'", output),
        source: e,
    })?;
    file.write_all(&bytes).map_err(|e| AssemblerError::IoError {
        operation: format!("Failed to write to output file '{}'", output),
        source: e,
    })?;
    
    Ok(())
}