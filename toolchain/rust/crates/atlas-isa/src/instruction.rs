
/// Instruction by mnemonic
enum Instruction {
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


/// Resolved instruction with all operands specified, format is optimized for encoding and simulation
enum ResolvedInstruction {
    A { op: AluOp, dest: RegisterIdentifier, source: RegisterIdentifier },
    I { op: ImmOp, dest: RegisterIdentifier, immediate: u8 },
    M { op: MemOp, dest: RegisterIdentifier, base: RegisterIdentifier, offset: MOffset },
    BI { absolute: bool, cond: BranchCond, address: u8 },
    BR { absolute: bool, cond: BranchCond, source: RegisterPairIdentifier },
    S { op: StackOp, register: RegisterIdentifier },
    P { op: PortOp, register: RegisterIdentifier, offset: u8 },
    X { op: XTypeOp, operand: XOperand },
}

#[repr(u8)]
enum AluOp {
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

#[repr(u8)]
enum ImmOp {
    LDI = 0,
    ADDI,
    SUBI,
    ANDI,
    ORI,
}

#[repr(u8)]
enum MemOp {
    LD = 0,
    ST,
}

#[repr(u8)]
enum BranchCond {
    Unconditional = 0,
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
}

#[repr(u8)]
enum StackOp {
    PUSH = 0,
    POP,
    SUBSP,
    ADDSP,
}

#[repr(u8)]
enum PortOp {
    POKE = 0,
    PEEK,
}

#[repr(u8)]
enum XTypeOp {
    SYSC = 0,
    ERET,
    HALT,
    ICINV,
    DCINV,
    DCCLEAN,
    FLUSH,
}

enum XOperand {
    None,
    Immediate(u8),
    Register(RegisterIdentifier),
    Registers(RegisterIdentifier, RegisterIdentifier),
}

type RegisterIdentifier = u8;


struct RegisterPairIdentifier {
    high: RegisterIdentifier,
    low: RegisterIdentifier,
}

enum MOffset {
    Offset8(u8),
    SR(RegisterIdentifier),
}