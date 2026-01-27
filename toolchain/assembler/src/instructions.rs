
/// The instruction enum represents all possible instructions in the ISA.
/// Not by reflect mnemonics, but is organized by type and operands,
/// because the same mnemonic can have different operands.
#[derive(Debug, Clone)]
pub enum Instruction {
    A { // ALU
        operation: u8,
        rd: u8,
        rs: u8,
    },
    I { // Immediate
        operation: u8,
        rd: u8,
        immediate: u8,
    },
    M { // Memory
        operation: u8,  // load or store
        rd: u8,
        rb: u8,  // base register
        offset: i8,  // or spr_code
    },
    BI { // Branch Immediate
        absolute: bool,
        condition: u8,
        offset: BranchOffset,
    },
    BR { // Branch Register
        absolute: bool,
        condition: u8,
        rs: (u8, u8),
    },
    S {  // Stack
        operation: u8,
        operand: SOperand,
    },
    P {  // Peek/Poke
        operation: u8,
        reg: u8,
        offset: u8,
    },
    X {  // Extended opcode format
        operation: u8,
        operand: XOperand,
    },
}

#[derive(Debug, Clone)]
pub enum BranchOffset {
    Immediate(i8),
    Label(String),
}

#[derive(Debug, Clone)]
pub enum SOperand {
    RegisterPair(u8, u8),
    Immediate(u8),
}

#[derive(Debug, Clone)]
pub enum XOperand {
    Registers {
        source: u8,
        destination: u8,
    },
    Immediate(u8),
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::A {
            operation: 0,
            rd: 0,
            rs: 0,
        }
    }
}

/// MnemonicInfo provides mapping from mnemonic strings to their types and operation codes.
pub enum MnemonicInfo {
    A(u8),      // operation code is deterministic
    I(u8),      // operation code is deterministic
    M(u8),      // operation code is deterministic
    B(u8),      // condition code (absolute/relative determined by @ prefix in parser)
    S(u8),      // xop is deterministic
    P(u8),      // operation is deterministic
    X(u8),      // xop is deterministic
    Psuedo(u8),  // for virtual instructions that expand to real ones
    Unknown,
}

impl MnemonicInfo {
    pub fn from_str(mnemonic: &str) -> Self {
        let mnemonic = mnemonic.to_ascii_lowercase();

        match mnemonic.as_str() {
            // A-type
            "add" => MnemonicInfo::A(0),
            "addc" => MnemonicInfo::A(1),
            "sub" => MnemonicInfo::A(2),
            "subc" => MnemonicInfo::A(3),
            "and" => MnemonicInfo::A(4),
            "or" => MnemonicInfo::A(5),
            "xor" => MnemonicInfo::A(6),
            "not" => MnemonicInfo::A(7),
            "shl" => MnemonicInfo::A(8),
            "shr" => MnemonicInfo::A(9),
            "rol" => MnemonicInfo::A(10),
            "ror" => MnemonicInfo::A(11),
            "cmp" => MnemonicInfo::A(12),
            "tst" => MnemonicInfo::A(13),
            "mov" => MnemonicInfo::A(14),
            "neg" => MnemonicInfo::A(15),

            // I-type
            "ldi" => MnemonicInfo::I(0),
            "addi" => MnemonicInfo::I(1),
            "subi" => MnemonicInfo::I(2),
            "andi" => MnemonicInfo::I(3),
            "ori" => MnemonicInfo::I(4),

            // M-type
            "ld" => MnemonicInfo::M(0),
            "st" => MnemonicInfo::M(1),

            // B*-type (branches)
            "br" => MnemonicInfo::B(0),
            "beq" => MnemonicInfo::B(1),
            "bne" => MnemonicInfo::B(2),
            "bcs" => MnemonicInfo::B(3),
            "bcc" => MnemonicInfo::B(4),
            "bmi" => MnemonicInfo::B(5),
            "bpl" => MnemonicInfo::B(6),
            "bov" => MnemonicInfo::B(7),

            // S-type
            "push" => MnemonicInfo::S(0),
            "pop" => MnemonicInfo::S(1),
            "subsp" => MnemonicInfo::S(2), // subsp can be 2 or 3
            "addsp" => MnemonicInfo::S(4), // addsp can be 4 or 5

            // P-type
            "peek" => MnemonicInfo::P(0),
            "poke" => MnemonicInfo::P(1),

            // X-type
            "syscall" => MnemonicInfo::X(0),
            "eret" => MnemonicInfo::X(1),
            "halt" => MnemonicInfo::X(2),

            "mmu_on" => MnemonicInfo::X(3),
            "mmu_off" => MnemonicInfo::X(4),
            "mmuwr" => MnemonicInfo::X(5),
            "mmurd" => MnemonicInfo::X(6),
            "mmuflush" => MnemonicInfo::X(7),
            "mmupid" => MnemonicInfo::X(8),

            "icache_inv" => MnemonicInfo::X(9),
            "dcache_inv" => MnemonicInfo::X(10),
            "dcache_clean" => MnemonicInfo::X(11),
            "cache_flush" => MnemonicInfo::X(12),

            _ => MnemonicInfo::Unknown,
        }
    }
}