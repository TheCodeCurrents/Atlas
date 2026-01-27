use crate::instructions::{Instruction, MnemonicInfo};
use crate::lexer::{Token, SpecialRegister};
use crate::symbol_table::{SymbolTable, Symbol};


#[derive(Debug, Default)]
pub struct Parser {
    tokens: Vec<Token>,
    ir: Vec<(String, Instruction)>,
    index: u32,
    symbol_table: SymbolTable,
    current_address: u32,  // Track byte offset for label positions
}

impl Parser {
    pub fn parse(&mut self, tokens: Vec<Token>, _symbol_table: &SymbolTable) -> Vec<(String, Instruction)> {
        self.tokens = tokens;
        self.symbol_table = _symbol_table.clone();
        self.current_address = 0;

        while self.index < self.tokens.len() as u32 {
            match self.get_next_token() {
                Some(Token::Label(name)) => {
                    // Store label position in symbol table
                    let label_name = name.clone();
                    self.symbol_table.insert(label_name, Symbol::Label(self.current_address));
                    // Don't add label to IR, just continue
                }
                Some(Token::Mnemonic(mnemonic_s)) => {
                    let mnemonic = MnemonicInfo::from_str(&mnemonic_s);
                    let mnemonic_s = mnemonic_s.clone();

                    // process instruction based on its type
                    let instruction = self.process_instruction(mnemonic).unwrap();
                    self.ir.push((mnemonic_s, instruction));
                    // Each instruction is 2 bytes
                    self.current_address += 2;
                }
                _ => {}
            }
        }

        self.ir.clone()
    }

    pub fn get_symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    /// Processes an instruction based on its MnemonicInfo and adds it to the IR
    fn process_instruction(&mut self, mnemonic: MnemonicInfo) -> Result<Instruction, String> {
        // reference MnemonicInfo::* as *
        use MnemonicInfo::*;

        match mnemonic {
            A(operation) => {
                let rd = self.expect_register()?;
                self.expect(Token::Comma)?;
                let rs = self.expect_register()?;
                
                Ok(Instruction::A {
                    operation,
                    rd,
                    rs,
                })
            },
            I(operation) => {
                let rd = self.expect_register()?;
                self.expect(Token::Comma)?;
                let imm = self.expect_immediate()?;

                // validate immediate fits in u8
                let immediate = if imm >= 0 && imm <= 255 {
                    imm as u8
                } else {
                    return Err("Immediate value out of range for I-type instruction".to_string());
                };

                Ok(Instruction::I {
                    operation,
                    rd,
                    immediate,
                })
            },
            M(operation) => {
                let rd = self.expect_register()?;
                self.expect(Token::Comma)?;
                
                self.expect(Token::LeftBracket)?;
                let base = self.expect_register()?;
                self.expect(Token::Plus)?;
                let offset_imm = self.expect_immediate()?;
                self.expect(Token::RightBracket)?;

                // M-type uses 4-bit signed offset with special codes:
                // -8 (0b1000): TR, -7 (0b1001): SP, -6 (0b1010): PC
                // Valid range: -5 to +7 for regular offsets
                let offset = if offset_imm >= -5 && offset_imm <= 7 {
                    offset_imm as i8
                } else if offset_imm == -8 || offset_imm == -7 || offset_imm == -6 {
                    offset_imm as i8 // These are valid SPR codes
                } else {
                    return Err(format!("Offset {} out of range for M-type instruction (must be -8 to 7, with -8/-7/-6 reserved for SPR)", offset_imm));
                };

                Ok(Instruction::M {
                    operation,
                    rd,
                    rb: base,
                    offset,
                })
            },
            B(condition) => {
                // Check for @ prefix to determine absolute vs relative
                let absolute = self.peek_token() == Some(&Token::At);
                if absolute {
                    self.get_next_token(); // consume @
                }

                // Determine if this is BI (immediate/label) or BR (register) based on next token
                match self.peek_token() {
                    Some(Token::Immediate(_)) => {
                        let offset_imm = self.expect_immediate()?;
                        
                        // Validate offset fits in i8
                        let offset = if offset_imm >= -128 && offset_imm <= 127 {
                            offset_imm as i8
                        } else {
                            return Err("Branch offset out of range for BI-type instruction (must fit in i8)".to_string());
                        };

                        Ok(Instruction::BI {
                            absolute,
                            condition,
                            offset: crate::instructions::BranchOffset::Immediate(offset),
                        })
                    },
                    Some(Token::Identifier(_)) | Some(Token::Mnemonic(_)) => {
                        // This is a label reference - could be tokenized as Identifier or Mnemonic
                        let label_name = match self.get_next_token() {
                            Some(Token::Identifier(id)) => id.clone(),
                            Some(Token::Mnemonic(m)) => m.clone(),
                            _ => return Err("Expected identifier or mnemonic for label".to_string()),
                        };
                        
                        Ok(Instruction::BI {
                            absolute,
                            condition,
                            offset: crate::instructions::BranchOffset::Label(label_name),
                        })
                    },
                    Some(Token::Register(_)) => {
                        let rs = self.expect_register_pair()?;
                        
                        Ok(Instruction::BR {
                            absolute,
                            condition,
                            rs,
                        })
                    },
                    Some(Token::RegisterPair(_, _)) => {
                        let rs = self.expect_register_pair()?;
                        
                        Ok(Instruction::BR {
                            absolute,
                            condition,
                            rs,
                        })
                    },
                    Some(Token::SpecialRegister(_)) => {
                        // Handle special registers (TR, SP, PC) as register pairs
                        let spr: SpecialRegister = self.expect_special_register()?;
                        let rs = match spr {
                            crate::lexer::SpecialRegister::TR => (10, 11),
                            crate::lexer::SpecialRegister::SP => (12, 13),
                            crate::lexer::SpecialRegister::PC => (14, 15),
                        };
                        
                        Ok(Instruction::BR {
                            absolute,
                            condition,
                            rs,
                        })
                    },
                    _ => Err("Expected immediate offset, label, or register (pair) for branch instruction".to_string()),
                }
            },
            S(operation) => {
                // S-type can take either a register pair or an immediate
                // For subsp and addsp, adjust opcode based on operand type:
                // - subsp imm8: opcode 2, subsp rs: opcode 3
                // - addsp imm8: opcode 4, addsp rs: opcode 5
                match self.peek_token() {
                    Some(Token::Immediate(_)) => {
                        let imm = self.expect_immediate()?;
                        
                        // Validate immediate fits in u8
                        let immediate = if imm >= 0 && imm <= 255 {
                            imm as u8
                        } else {
                            return Err("Immediate value out of range for S-type instruction".to_string());
                        };
                        
                        // Use base operation code for immediate operands (already correct: 2 for subsp, 4 for addsp)
                        Ok(Instruction::S {
                            operation,
                            operand: crate::instructions::SOperand::Immediate(immediate),
                        })
                    },
                    Some(Token::Register(_)) => {
                        // Single register - treat as r0:register pair
                        let reg = self.expect_register()?;
                        
                        // Adjust opcode for register pair operands: +1 for subsp/addsp
                        let adjusted_operation = if operation == 2 || operation == 4 {
                            operation + 1
                        } else {
                            operation
                        };
                        
                        Ok(Instruction::S {
                            operation: adjusted_operation,
                            operand: crate::instructions::SOperand::RegisterPair(0, reg),
                        })
                    },
                    Some(Token::RegisterPair(_, _)) => {
                        let (r1, r2) = self.expect_register_pair()?;
                        
                        // Adjust opcode for register pair operands: +1 for subsp/addsp
                        // subsp: 2 -> 3, addsp: 4 -> 5, push/pop: remain 0/1
                        let adjusted_operation = if operation == 2 || operation == 4 {
                            operation + 1
                        } else {
                            operation
                        };
                        
                        Ok(Instruction::S {
                            operation: adjusted_operation,
                            operand: crate::instructions::SOperand::RegisterPair(r1, r2),
                        })
                    },
                    Some(Token::SpecialRegister(_)) => {
                        // Handle special registers (TR, SP, PC) as register pairs
                        let spr: SpecialRegister = self.expect_special_register()?;
                        let (r1, r2) = match spr {
                            crate::lexer::SpecialRegister::TR => (10, 11),
                            crate::lexer::SpecialRegister::SP => (12, 13),
                            crate::lexer::SpecialRegister::PC => (14, 15),
                        };
                        
                        // Adjust opcode for register pair operands: +1 for subsp/addsp
                        let adjusted_operation = if operation == 2 || operation == 4 {
                            operation + 1
                        } else {
                            operation
                        };
                        
                        Ok(Instruction::S {
                            operation: adjusted_operation,
                            operand: crate::instructions::SOperand::RegisterPair(r1, r2),
                        })
                    },
                    _ => Err("Expected immediate or register (pair) for S-type instruction".to_string()),
                }
            },
            P(operation) => {
                let reg = self.expect_register()?;
                self.expect(Token::Comma)?;
                let offset_imm = self.expect_immediate()?;
                
                // Validate offset fits in u8
                let offset = if offset_imm >= 0 && offset_imm <= 255 {
                    offset_imm as u8
                } else {
                    return Err("Offset value out of range for P-type instruction".to_string());
                };
                
                Ok(Instruction::P {
                    operation,
                    reg,
                    offset,
                })
            },
            X(operation) => {
                // X-type can take different operands based on the operation
                // Some operations have no operands (syscall, eret, halt, etc.)
                // Some take register operands (mmuwr, mmurd, mmupid)
                // Some take immediate operands
                
                match self.peek_token() {
                    Some(Token::Register(_)) => {
                        // Check if it's a two-register operation
                        let src = self.expect_register()?;
                        
                        // Check if there's a second register
                        if self.peek_token() == Some(&Token::Comma) {
                            self.expect(Token::Comma)?;
                            let dst = self.expect_register()?;
                            
                            Ok(Instruction::X {
                                operation,
                                operand: crate::instructions::XOperand::Registers {
                                    source: src,
                                    destination: dst,
                                },
                            })
                        } else {
                            // Single register, treat as source
                            Ok(Instruction::X {
                                operation,
                                operand: crate::instructions::XOperand::Registers {
                                    source: src,
                                    destination: 0,
                                },
                            })
                        }
                    },
                    Some(Token::Immediate(_)) => {
                        let imm = self.expect_immediate()?;
                        
                        // Validate immediate fits in u8
                        let immediate = if imm >= 0 && imm <= 255 {
                            imm as u8
                        } else {
                            return Err("Immediate value out of range for X-type instruction".to_string());
                        };
                        
                        Ok(Instruction::X {
                            operation,
                            operand: crate::instructions::XOperand::Immediate(immediate),
                        })
                    },
                    _ => {
                        // No operands - use immediate 0 as default
                        Ok(Instruction::X {
                            operation,
                            operand: crate::instructions::XOperand::Immediate(0),
                        })
                    }
                }
            },
            _ => Ok(Instruction::default()),
        }
    }

    pub fn get_next_token(&mut self) -> Option<&Token> {
        if self.index < self.tokens.len() as u32 {
            let token = &self.tokens[self.index as usize];
            self.index += 1;
            Some(token)
        } else {
            None
        }
    }

    pub fn peek_token(&self) -> Option<&Token> {
        if self.index < self.tokens.len() as u32 {
            Some(&self.tokens[self.index as usize])
        } else {
            None
        }
    }

    pub fn expect_register(&mut self) -> Result<u8, String> {
        match self.get_next_token() {
            Some(Token::Register(reg)) => Ok(*reg),
            _ => Err("Expected register".to_string()),
        }
    }

    pub fn expect_register_pair(&mut self) -> Result<(u8, u8), String> {
        match self.get_next_token() {
            Some(Token::RegisterPair(reg1, reg2)) => Ok((*reg1, *reg2)),
            _ => Err("Expected register pair".to_string()),
        }
    }

    pub fn expect_immediate(&mut self) -> Result<i32, String> {
        match self.get_next_token() {
            Some(Token::Immediate(imm)) => Ok(*imm),
            _ => Err("Expected immediate".to_string()),
        }
    }

    pub fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.get_next_token() {
            Some(token) if std::mem::discriminant(token) == std::mem::discriminant(&expected) => Ok(()),
            _ => Err(format!("Expected {:?}", expected)),
        }
    }

    pub fn expect_special_register(&mut self) -> Result<SpecialRegister, String> {
        match self.get_next_token() {
            Some(Token::SpecialRegister(spr)) => Ok(*spr),
            Some(Token::RegisterPair(r1, r2)) => {
                // Convert register pairs to special registers
                match (*r1, *r2) {
                    (10, 11) => Ok(SpecialRegister::TR),
                    (12, 13) => Ok(SpecialRegister::SP),
                    (14, 15) => Ok(SpecialRegister::PC),
                    _ => Err(format!("Register pair ({}, {}) is not a valid special register", r1, r2)),
                }
            }
            Some(Token::Register(r)) => {
                // Handle single registers that might map to special registers
                match *r {
                    10 | 11 => Ok(SpecialRegister::TR),
                    12 | 13 => Ok(SpecialRegister::SP),
                    14 | 15 => Ok(SpecialRegister::PC),
                    _ => Err(format!("Register r{} is not a special register", r)),
                }
            }
            _ => Err("Expected special register or register pair".to_string()),
        }
    }

    pub fn expect_identifier(&mut self) -> Result<String, String> {
        match self.get_next_token() {
            Some(Token::Identifier(id)) => Ok(id.clone()),
            _ => Err("Expected identifier".to_string()),
        }
    }
}