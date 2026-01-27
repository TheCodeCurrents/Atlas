
pub mod error;
pub mod instructions;
pub mod lexer;
pub mod parser;
pub mod symbol_table;
pub mod linker;
pub mod encoder;

pub use error::{AssemblerError, AssemblyPhase};
pub use instructions::Instruction;
pub use symbol_table::SymbolTable;

use lexer::Lexer;
use parser::Parser;
use linker::Linker;
use encoder::Encoder;

/// Represents the complete output of the assembly process
#[derive(Debug, Clone)]
pub struct AssemblyResult {
    /// Final resolved symbol table
    pub symbol_table: SymbolTable,
    /// Linked intermediate representation (address, mnemonic, instruction)
    pub linked_ir: Vec<(u32, String, Instruction)>,
    /// Final machine code (address, encoded 16-bit instruction)
    pub machine_code: Vec<(u32, u16)>,
}

/// Top-level assembler orchestrating all phases
pub struct Assembler;

impl Assembler {
    /// Assembles source code through the complete pipeline: lexer → parser → linker → encoder
    /// 
    /// Returns structured results or detailed errors at each phase.
    pub fn assemble(source: String) -> Result<AssemblyResult, AssemblerError> {
        // Phase 1: Lexing
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()
            .map_err(|e| AssemblerError::lex(e))?;

        // Phase 2: Parsing
        let mut parser = Parser::default();
        let symbol_table = SymbolTable::new();
        let parsed_instructions = parser.parse(tokens, &symbol_table);
        let parser_symbol_table = parser.get_symbol_table().clone();

        // Phase 3: Linking (multi-file symbol resolution and address assignment)
        let mut linker = Linker::new();
        linker.add_file(parsed_instructions, parser_symbol_table);
        linker.link()
            .map_err(|e| AssemblerError::link(e))?;

        let (linked_ir, final_symbol_table) = linker.get_linked_ir()
            .ok_or_else(|| AssemblerError::link("Linking produced no output"))?;

        // Phase 4: Encoding (IR to machine code)
        let mut encoder = Encoder::new(linked_ir.clone());
        encoder.encode()
            .map_err(|e| AssemblerError::encode(e))?;
        
        let machine_code = encoder.output();

        Ok(AssemblyResult {
            symbol_table: final_symbol_table,
            linked_ir,
            machine_code,
        })
    }
}