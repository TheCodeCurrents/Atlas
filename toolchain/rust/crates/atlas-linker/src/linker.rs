use std::collections::HashMap;
use atlas_isa::{BranchOperand, ResolvedInstruction};

/// Represents a label and its address in the output binary
#[derive(Debug, Clone)]
pub struct LabelMap {
    labels: HashMap<String, u8>,
}

impl LabelMap {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
        }
    }

    /// Insert a label and its resolved address
    pub fn insert(&mut self, label: String, address: u8) {
        self.labels.insert(label, address);
    }

    /// Look up a label's address
    pub fn get(&self, label: &str) -> Option<u8> {
        self.labels.get(label).copied()
    }
}

pub struct Linker {
    label_map: LabelMap,
}

impl Linker {
    pub fn new() -> Self {
        Self {
            label_map: LabelMap::new(),
        }
    }

    /// Register a label with its resolved address
    pub fn register_label(&mut self, label: String, address: u8) {
        self.label_map.insert(label, address);
    }

    /// Resolve all label references in instructions to actual addresses
    /// This converts BranchOperand::Label to BranchOperand::Immediate with the resolved address
    pub fn resolve_labels(&self, instructions: Vec<ResolvedInstruction>) -> Result<Vec<ResolvedInstruction>, String> {
        instructions
            .into_iter()
            .map(|instr| self.resolve_instruction(instr))
            .collect()
    }

    /// Resolve labels in a single instruction
    fn resolve_instruction(&self, instr: ResolvedInstruction) -> Result<ResolvedInstruction, String> {
        match instr {
            ResolvedInstruction::BI { absolute, cond, operand, line } => {
                let resolved_operand = match operand {
                    BranchOperand::Immediate(addr) => BranchOperand::Immediate(addr),
                    BranchOperand::Label(label) => {
                        let addr = self.label_map.get(&label)
                            .ok_or_else(|| format!("Unresolved label: '{}'", label))?;
                        BranchOperand::Immediate(addr)
                    }
                };
                Ok(ResolvedInstruction::BI {
                    absolute,
                    cond,
                    operand: resolved_operand,
                    line,
                })
            }
            other => Ok(other),
        }
    }
}