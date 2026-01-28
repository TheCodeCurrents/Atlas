/// ALU operation codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluOp {
    ADD = 0,
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
}

/// Immediate operation codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImmOp {
    LDI = 0,
    ADDI,
    SUBI,
    ANDI,
    ORI,
}

/// Memory operation codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemOp {
    LD = 0,
    ST,
}

/// Branch condition codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchCond {
    Unconditional = 0,
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
}

/// Stack operation codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackOp {
    PUSH = 0,
    POP,
    SUBSP,
    ADDSP,
}

/// Port operation codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortOp {
    POKE = 0,
    PEEK,
}

/// Extended operation codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XTypeOp {
    SYSC = 0,
    ERET,
    HALT,
    ICINV,
    DCINV,
    DCCLEAN,
    FLUSH,
}
