#include "instructions.h"
#include <stddef.h>
#include <stdio.h>

/* ===== Dispatch table ===== */

InstrFn atlas_dispatch_a[A_OP_COUNT];

/* ===== Instruction implementations ===== */

void instr_illegal(CPU *cpu) {
    /* For now: simple trap */
    (void)cpu;
    fprintf(stderr, "Illegal instruction: 0x%04X\n", cpu->ir.value);
    /* Later: set a fault flag or halt */
}

/* ===== A-type instruction implementations ===== */

/// @brief Add the values of two registers and store the result in the destination register
/// @param cpu Pointer to the CPU structure
void instr_add(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] += cpu->registers.regs[rs];
}

/// @brief Add the values of two registers and store the result in the destination register, with carry
/// @param cpu Pointer to the CPU structure
void instr_addc(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    // TODO: Implement carry flag
    cpu->registers.regs[rd] = cpu->registers.regs[rd] + cpu->registers.regs[rs]; // + carry
}

void instr_sub(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] -= cpu->registers.regs[rs];
}

void instr_subc(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    // TODO: Implement carry flag
    cpu->registers.regs[rd] = cpu->registers.regs[rd] - cpu->registers.regs[rs]; // - carry
}

void instr_and(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] &= cpu->registers.regs[rs];
}

void instr_or(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] |= cpu->registers.regs[rs];
}

void instr_xor(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] ^= cpu->registers.regs[rs];
}

void instr_not(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] = ~cpu->registers.regs[rs];
}

void instr_shl(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] = cpu->registers.regs[rs] << 1;
}

void instr_shr(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] = cpu->registers.regs[rs] >> 1;
}

void instr_rol(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t val = cpu->registers.regs[rs];
    cpu->registers.regs[rd] = (val << 1) | (val >> 7);
}

void instr_ror(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t val = cpu->registers.regs[rs];
    cpu->registers.regs[rd] = (val >> 1) | (val << 7);
}

void instr_cmp(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    // TODO: Implement flags based on rd - rs
    (void)rd;
    (void)rs;
}

void instr_tst(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    // TODO: Implement flags based on rd & rs
    (void)rd;
    (void)rs;
}

void instr_mov(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] = cpu->registers.regs[rs];
}

void instr_neg(CPU *cpu) {
    uint8_t rd = (cpu->ir.bytes[1] >> 4) & 0x0F;
    // TODO: Determine proper negation semantics
    cpu->registers.regs[rd] = -cpu->registers.regs[rd];
}

/* ===== I-type instruction implementation ===== */

void instr_ldi(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t imm = cpu->ir.bytes[0];
    if (rd == 0) {
        // LDI to R0 is illegal
        instr_illegal(cpu);
        return;
    }
    cpu->registers.regs[rd] = imm;
}

/* ===== M-type instruction implementation ===== */

void instr_ld(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t base_code = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t off_code = cpu->ir.bytes[0] & 0x0F;

    uint16_t base = 0;
    int16_t offset = 0;

    // get base address from registers
    if (base_code <= 9) {
        // 8-bit gprs
        base = cpu->registers.regs[base_code];
    } else {
        // 16-bit sprs
        base_code &= ~1; // set lsb to 0
        base = cpu->registers.regs[base_code];
        base = cpu->registers.regs[base_code + 1] << 8;
    }

    // get the offset
    if (off_code <= 12) {
        offset = off_code;
    } else if (off_code == 13) {
        offset = cpu->registers.tr.value;
    } else if (off_code == 14) {
        offset = cpu->registers.sp.value;
    } else if (off_code == 15) {
        offset = cpu->registers.pc.value;
    }

    cpu->registers.regs[rd] = cpu->memory[(uint16_t)(base + offset)];
}

void instr_st(CPU *cpu) {
    uint8_t rs = cpu->ir.bytes[1] & 0x0F;
    uint8_t base_code = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t off_code = cpu->ir.bytes[0] & 0x0F;

    uint16_t base = 0;
    int16_t offset = 0;

    // get base address from registers
    if (base_code <= 9) {
        // 8-bit gprs
        base = cpu->registers.regs[base_code];
    } else {
        // 16-bit sprs
        base_code &= ~1; // set lsb to 0
        base = cpu->registers.regs[base_code];
        base = cpu->registers.regs[base_code + 1] << 8;
    }

    // get the offset
    if (off_code <= 12) {
        offset = off_code;
    } else if (off_code == 13) {
        offset = cpu->registers.tr.value;
    } else if (off_code == 14) {
        offset = cpu->registers.sp.value;
    } else if (off_code == 15) {
        offset = cpu->registers.pc.value;
    }

    cpu->memory[(uint16_t)(base + offset)] = cpu->registers.regs[rs];
}

/* ===== Initialization ===== */

void instructions_init(void) {
    // Initialize A-type dispatch table
    for (int i = 0; i < A_OP_COUNT; ++i)
        atlas_dispatch_a[i] = instr_illegal;

    atlas_dispatch_a[A_OP_ADD] = instr_add;
    atlas_dispatch_a[A_OP_ADDC] = instr_addc;
    atlas_dispatch_a[A_OP_SUB] = instr_sub;
    atlas_dispatch_a[A_OP_SUBC] = instr_subc;
    atlas_dispatch_a[A_OP_AND] = instr_and;
    atlas_dispatch_a[A_OP_OR] = instr_or;
    atlas_dispatch_a[A_OP_XOR] = instr_xor;
    atlas_dispatch_a[A_OP_NOT] = instr_not;
    atlas_dispatch_a[A_OP_SHL] = instr_shl;
    atlas_dispatch_a[A_OP_SHR] = instr_shr;
    atlas_dispatch_a[A_OP_ROL] = instr_rol;
    atlas_dispatch_a[A_OP_ROR] = instr_ror;
    atlas_dispatch_a[A_OP_CMP] = instr_cmp;
    atlas_dispatch_a[A_OP_TST] = instr_tst;
    atlas_dispatch_a[A_OP_MOV] = instr_mov;
    atlas_dispatch_a[A_OP_NEG] = instr_neg;
}