use crate::{instructions::{Instruction, BranchOffset}, symbol_table::{Symbol, SymbolTable}};

pub struct Linker {
    symbol_tables: Vec<SymbolTable>,
    irs: Vec<Vec<(String, Instruction)>>,
    global_symbol_table: SymbolTable,
    ir: Vec<(u32, String, Instruction)>,
}

impl Default for Linker {
    fn default() -> Self {
        Self {
            symbol_tables: Vec::new(),
            irs: Vec::new(),
            global_symbol_table: SymbolTable::new(),
            ir: Vec::new(),
        }
    }
}

impl Linker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, ir: Vec<(String, Instruction)>, symbol_table: SymbolTable) {
        self.irs.push(ir);
        self.symbol_tables.push(symbol_table);
    }

    pub fn link(&mut self) -> Result<(), String> {
        // First pass: assign byte addresses and collect global symbols
        let mut current_address = 0u32; // in bytes
        let mut address_map = Vec::new(); // Maps file_idx to its starting byte address
        
        for (_file_idx, ir) in self.irs.iter().enumerate() {
            address_map.push(current_address);
            
            current_address += (ir.len() as u32) * 2; // Each instruction is 2 bytes
        }
        
        // Collect all labels from symbol tables and build global label map
        for symbol_table in self.symbol_tables.iter() {
            // The parser populates the symbol_table during parsing
            // We merge all symbol tables into the global one
            self.global_symbol_table = symbol_table.clone();
        }
        
        // Second pass: resolve symbols and create final IR
        for (file_idx, ir) in self.irs.iter().enumerate() {
            let base_address = address_map[file_idx];
            
            for (idx, (mnemonic, instruction)) in ir.iter().enumerate() {
                let current_byte_addr = base_address + (idx as u32) * 2;
                
                // Resolve symbol references in branches
                let resolved_instruction = self.resolve_instruction(
                    instruction.clone(),
                    current_byte_addr,
                )?;
                
                self.ir.push((current_byte_addr, mnemonic.clone(), resolved_instruction));
            }
        }
        
        Ok(())
    }

    fn resolve_instruction(
        &self,
        instruction: Instruction,
        current_byte_addr: u32,
    ) -> Result<Instruction, String> {
        match instruction {
            Instruction::BI { absolute, condition, ref offset } => {
                match offset {
                    BranchOffset::Label(label_name) => {
                        // Look up label address in global symbol table
                        match self.global_symbol_table.resolve(&label_name) {
                            Some(Symbol::Label(target_addr)) => {
                                // Calculate relative offset if needed
                                let offset_val = if absolute {
                                    // Absolute mode: use target address directly
                                    *target_addr as i8
                                } else {
                                    // Relative mode: calculate offset from next instruction
                                    let next_addr = current_byte_addr + 2;
                                    let diff = (*target_addr as i32) - (next_addr as i32);
                                    if diff >= -128 && diff <= 127 {
                                        diff as i8
                                    } else {
                                        return Err(format!("Branch target {} is too far from instruction at {}", target_addr, current_byte_addr));
                                    }
                                };
                                
                                Ok(Instruction::BI {
                                    absolute,
                                    condition,
                                    offset: BranchOffset::Immediate(offset_val),
                                })
                            }
                            _ => Err(format!("Unresolved label: {}", label_name)),
                        }
                    }
                    BranchOffset::Immediate(_) => {
                        // Already resolved
                        Ok(instruction)
                    }
                }
            }
            _ => {
                // Other instructions don't need resolution
                Ok(instruction)
            }
        }
    }

    pub fn get_ir(&self) -> &[(u32, String, Instruction)] {
        &self.ir
    }

    /// Returns the linked IR and final symbol table as a tuple
    pub fn get_linked_ir(&self) -> Option<(Vec<(u32, String, Instruction)>, SymbolTable)> {
        if self.ir.is_empty() {
            None
        } else {
            Some((self.ir.clone(), self.global_symbol_table.clone()))
        }
    }
}