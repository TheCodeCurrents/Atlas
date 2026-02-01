use crate::instructions::{Instruction, BranchOffset};

pub struct Encoder {
    ir: Vec<(u32, String, Instruction)>,
    output: Vec<(u32, u16)>,
}

impl Encoder {
    pub fn new(ir: Vec<(u32, String, Instruction)>) -> Self {
        Self {
            ir,
            output: Vec::new(),
        }
    }

    pub fn encode(&mut self) -> Result<(), String> {
        for (addr, _mnemonic, instruction) in &self.ir {
            let encoded = Self::encode_instruction(instruction)?;
            self.output.push((*addr, encoded));
        }
        Ok(())
    }

    /// Returns the generated machine code as (address, encoded_instruction) pairs
    pub fn output(&self) -> Vec<(u32, u16)> {
        self.output.clone()
    }

    fn encode_instruction(instruction: &Instruction) -> Result<u16, String> {

        match instruction {
            Instruction::A { operation, rd, rs } => {
                let encoded = ((*rd as u16) << 12) 
                    | (*rs as u16)
                    | ((*operation as u16));
                Ok(encoded)
            },
            Instruction::I { operation, rd, immediate } => {
                let type_field = 1 + *operation;

                let encoded: u16 = ((type_field as u16) << 12)
                    | ((*rd as u16) << 8)
                    | (*immediate as u16);

                Ok(encoded)
            },
            Instruction::M { operation, rd, rb, offset } => {
                let type_field = 6 + *operation;

                let encoded: u16 = ((type_field as u16) << 12)
                    | ((*rd as u16) << 8)
                    | ((*rb as u16) << 4)
                    | ((*offset as u16) & 0xF); // Mask to 4 bits to preserve sign bit in MSB

                Ok(encoded)
            },
            Instruction::BI { absolute, condition, offset } => {
                // Get the actual offset value - should be resolved by linker already
                let offset_val = match offset {
                    BranchOffset::Immediate(val) => *val,
                    BranchOffset::Label(_) => {
                        return Err("Unresolved label in branch instruction at encoding stage".to_string());
                    }
                };

                let encoded = (8 << 12)
                    | ((*absolute as u16) << 11)
                    | ((*condition as u16) << 8)
                    | ((offset_val as u16) & 0xFF); // Mask to 8 bits to preserve sign bit in MSB

                Ok(encoded)
            },
            Instruction::BR { absolute, condition, rs } => {
                // BR-type: bits [15:12]=1001, [11]=absolute, [10:8]=condition, [7:4]=high, [3:0]=low
                let (high, low) = rs;
                let encoded = (9 << 12)
                    | ((*absolute as u16) << 11)
                    | ((*condition as u16) << 8)
                    | ((*high as u16) << 4)
                    | (*low as u16);

                Ok(encoded)
            },
            Instruction::S { operation, operand } => {
                let encoded = (10 << 12)
                    | ((*operation as u16) << 8)
                    | match operand {
                        crate::instructions::SOperand::RegisterPair(r1, r2) => {
                            ((*r1 as u16) << 4) | (*r2 as u16)
                        },
                        crate::instructions::SOperand::Immediate(imm) => {
                            *imm as u16
                        },
                    };

                Ok(encoded)
            },
            Instruction::P { operation, reg, offset } => {
                // P-type: bits [15:12]=1011, [11]=operation (peek=0/poke=1), [10:8]=upper 3 bits of offset, [7:4]=reg, [3:0]=lower 4 bits of offset
                // Simpler layout: bits [15:12]=1011, [11:8]=operation+upper offset bits, [7:4]=reg, [3:0]=lower offset
                // Most likely: bits [15:12]=1011, [11]=operation, [10:8]=reg[2:0], [7:0]=offset
                let encoded = (11 << 12)
                    | ((*operation as u16) << 11)
                    | ((*reg as u16) << 8)
                    | (*offset as u16);

                Ok(encoded)
            },
            Instruction::X { operation, operand } => {
                // X-type: bits [15:12]=1100, [11:8]=xop (operation), then operand in [7:0]
                let encoded = (12 << 12)
                    | ((*operation as u16) << 8)
                    | match operand {
                        crate::instructions::XOperand::Registers { source, destination } => {
                            ((*source as u16) << 4) | (*destination as u16)
                        },
                        crate::instructions::XOperand::Immediate(imm) => {
                            *imm as u16
                        },
                    };

                Ok(encoded)
            },
        }
    }
}