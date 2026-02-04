use crate::opcode::{AluOp, BranchCond, ImmOp, MemOp, PortOp, StackOp, XTypeOp};
use crate::operands::{BranchOperand, MOffset, RegisterIdentifier, RegisterPairIdentifier, XOperand};

/// Instruction by mnemonic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    // A-type
    ADD,
    ADDC,
    SUB,
    SUBC,
    AND,
    OR,
    XOR,
    NOT,
    SHL,
    SHR,
    ROL,
    ROR,
    CMP,
    TST,
    MOV,
    NEG,

    // I-type
    LDI,
    ADDI,
    SUBI,
    ANDI,
    ORI,

    // M-type
    LD,
    ST,

    // B*-types
    BR,
    BEQ,
    BNE,
    BCS,
    BCC,
    BMI,
    BPL,

    // S-type
    PUSH,
    POP,
    SUBSP,
    ADDSP,

    // P-type
    POKE,
    PEEK,

    // X-type
    SYSC,
    ERET,
    HALT,
    ICINV,
    DCINV,
    DCCLEAN,
    FLUSH,

    // Virtual instructions
    NOP,
}

pub enum InstructionFormat {
    A,
    I,
    M,
    B,
    S,
    P,
    X,
    Virtual
}

/// Resolved instruction with all operands specified, format is optimized for encoding and simulation
/// ! Note: Not every possible combination of fields is valid for a given instruction.
#[derive(Debug, Clone)]
pub enum ResolvedInstruction {
    A {
        op: AluOp,
        dest: RegisterIdentifier,
        source: RegisterIdentifier,
        line: usize,
    },
    I {
        op: ImmOp,
        dest: RegisterIdentifier,
        immediate: u8,
        line: usize,
    },
    M {
        op: MemOp,
        dest: RegisterIdentifier,
        base: RegisterIdentifier,
        offset: MOffset,
        line: usize,
    },
    BI {
        absolute: bool,
        cond: BranchCond,
        operand: BranchOperand,
        line: usize,
    },
    BR {
        absolute: bool,
        cond: BranchCond,
        source: RegisterPairIdentifier,
        line: usize,
    },
    S {
        op: StackOp,
        register: RegisterIdentifier,
        line: usize,
    },
    P {
        op: PortOp,
        register: RegisterIdentifier,
        offset: u8,
        line: usize,
    },
    X {
        op: XTypeOp,
        operand: XOperand,
        line: usize,
    },
}

impl ResolvedInstruction {
    pub fn line(&self) -> usize {
        match self {
            ResolvedInstruction::A { line, .. } => *line,
            ResolvedInstruction::I { line, .. } => *line,
            ResolvedInstruction::M { line, .. } => *line,
            ResolvedInstruction::BI { line, .. } => *line,
            ResolvedInstruction::BR { line, .. } => *line,
            ResolvedInstruction::S { line, .. } => *line,
            ResolvedInstruction::P { line, .. } => *line,
            ResolvedInstruction::X { line, .. } => *line,
        }
    }
}
