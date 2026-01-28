//! Mnemonic to opcode and family mappings
//!
//! This module provides conversions between instruction mnemonics and their
//! corresponding opcodes and instruction families.

use crate::instruction::Instruction;

impl Instruction {
    /// Get the mnemonic string for the instruction
    pub fn mnemonic(&self) -> &'static str {
        match self {
            // A-type
            Instruction::ADD => "ADD",
            Instruction::ADDC => "ADDC",
            Instruction::SUB => "SUB",
            Instruction::SUBC => "SUBC",
            Instruction::AND => "AND",
            Instruction::OR => "OR",
            Instruction::XOR => "XOR",
            Instruction::NOT => "NOT",
            Instruction::SHL => "SHL",
            Instruction::SHR => "SHR",
            Instruction::ROL => "ROL",
            Instruction::ROR => "ROR",
            Instruction::CMP => "CMP",
            Instruction::TST => "TST",
            Instruction::MOV => "MOV",
            Instruction::NEG => "NEG",

            // I-type
            Instruction::LDI => "LDI",
            Instruction::ADDI => "ADDI",
            Instruction::SUBI => "SUBI",
            Instruction::ANDI => "ANDI",
            Instruction::ORI => "ORI",

            // M-type
            Instruction::LD => "LD",
            Instruction::ST => "ST",

            // B*-types
            Instruction::BR => "BR",
            Instruction::BEQ => "BEQ",
            Instruction::BNE => "BNE",
            Instruction::BCS => "BCS",
            Instruction::BCC => "BCC",
            Instruction::BMI => "BMI",
            Instruction::BPL => "BPL",

            // S-type
            Instruction::PUSH => "PUSH",
            Instruction::POP => "POP",
            Instruction::SUBSP => "SUBSP",
            Instruction::ADDSP => "ADDSP",

            // P-type
            Instruction::POKE => "POKE",
            Instruction::PEEK => "PEEK",

            // X-type
            Instruction::SYSC => "SYSC",
            Instruction::ERET => "ERET",
            Instruction::HALT => "HALT",
            Instruction::ICINV => "ICINV",
            Instruction::DCINV => "DCINV",
            Instruction::DCCLEAN => "DCCLEAN",
            Instruction::FLUSH => "FLUSH",

            // Virtual instructions
            Instruction::NOP => "NOP",
        }
    }

    fn from_str(mnemonic: &str) -> Option<Self> {
        match mnemonic {
            // A-type
            "ADD" => Some(Instruction::ADD),
            "ADDC" => Some(Instruction::ADDC),
            "SUB" => Some(Instruction::SUB),
            "SUBC" => Some(Instruction::SUBC),
            "AND" => Some(Instruction::AND),
            "OR" => Some(Instruction::OR),
            "XOR" => Some(Instruction::XOR),
            "NOT" => Some(Instruction::NOT),
            "SHL" => Some(Instruction::SHL),
            "SHR" => Some(Instruction::SHR),
            "ROL" => Some(Instruction::ROL),
            "ROR" => Some(Instruction::ROR),
            "CMP" => Some(Instruction::CMP),
            "TST" => Some(Instruction::TST),
            "MOV" => Some(Instruction::MOV),
            "NEG" => Some(Instruction::NEG),

            // I-type
            "LDI" => Some(Instruction::LDI),
            "ADDI" => Some(Instruction::ADDI),
            "SUBI" => Some(Instruction::SUBI),
            "ANDI" => Some(Instruction::ANDI),
            "ORI" => Some(Instruction::ORI),

            // M-type
            "LD" => Some(Instruction::LD),
            "ST" => Some(Instruction::ST),

            // B*-types
            "BR" => Some(Instruction::BR),
            "BEQ" => Some(Instruction::BEQ),
            "BNE" => Some(Instruction::BNE),
            "BCS" => Some(Instruction::BCS),
            "BCC" => Some(Instruction::BCC),
            "BMI" => Some(Instruction::BMI),
            "BPL" => Some(Instruction::BPL),

            // S-type
            "PUSH" => Some(Instruction::PUSH),
            "POP" => Some(Instruction::POP),
            "SUBSP" => Some(Instruction::SUBSP),
            "ADDSP" => Some(Instruction::ADDSP),

            // P-type
            "POKE" => Some(Instruction::POKE),
            "PEEK" => Some(Instruction::PEEK),

            // X-type
            "SYSC" => Some(Instruction::SYSC),
            "ERET" => Some(Instruction::ERET),
            "HALT" => Some(Instruction::HALT),
            "ICINV" => Some(Instruction::ICINV),
            "DCINV" => Some(Instruction::DCINV),
            "DCCLEAN" => Some(Instruction::DCCLEAN),
            "FLUSH" => Some(Instruction::FLUSH),

            // Virtual instructions
            "NOP" => Some(Instruction::NOP),
            _ => None,
        }
    }
}