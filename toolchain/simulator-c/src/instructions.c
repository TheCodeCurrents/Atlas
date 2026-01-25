#include "instructions.h"
#include <stddef.h>
#include <stdio.h>

#include "status.h"

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
    uint8_t a = cpu->registers.regs[rd];
    uint8_t b = cpu->registers.regs[rs];
    uint8_t r = (uint8_t)(a + b);
    cpu->registers.regs[rd] = r;
    sr_add8(cpu, a, b, 0, r);
}

/// @brief Add the values of two registers and store the result in the destination register, with carry
/// @param cpu Pointer to the CPU structure
void instr_addc(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t a = cpu->registers.regs[rd];
    uint8_t b = cpu->registers.regs[rs];
    uint8_t c = cpu_flag_get(cpu, SR_C);
    uint8_t r = (uint8_t)(a + b + c);
    cpu->registers.regs[rd] = r;
    sr_add8(cpu, a, b, c, r);
}

void instr_sub(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t a = cpu->registers.regs[rd];
    uint8_t b = cpu->registers.regs[rs];
    uint8_t r = (uint8_t)(a - b);
    cpu->registers.regs[rd] = r;
    sr_sub8(cpu, a, b, 0, r);
}

void instr_subc(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t a = cpu->registers.regs[rd];
    uint8_t b = cpu->registers.regs[rs];
    uint8_t borrow = (uint8_t)!cpu_flag_get(cpu, SR_C);
    uint8_t r = (uint8_t)(a - b - borrow);
    cpu->registers.regs[rd] = r;
    sr_sub8(cpu, a, b, borrow, r);
}

void instr_and(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] &= cpu->registers.regs[rs];
    sr_logic(cpu, cpu->registers.regs[rd]);
}

void instr_or(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] |= cpu->registers.regs[rs];
    sr_logic(cpu, cpu->registers.regs[rd]);
}

void instr_xor(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] ^= cpu->registers.regs[rs];
    sr_logic(cpu, cpu->registers.regs[rd]);
}

void instr_not(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] = (uint8_t)~cpu->registers.regs[rs];
    sr_logic(cpu, cpu->registers.regs[rd]);
}

void instr_shl(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t v = cpu->registers.regs[rs];
    cpu_flag_set(cpu, SR_C, (v & 0x80u) != 0);
    cpu->registers.regs[rd] = (uint8_t)(v << 1);
    sr_logic(cpu, cpu->registers.regs[rd]);
}

void instr_shr(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t v = cpu->registers.regs[rs];
    cpu_flag_set(cpu, SR_C, (v & 0x01u) != 0);
    cpu->registers.regs[rd] = (uint8_t)(v >> 1);
    sr_logic(cpu, cpu->registers.regs[rd]);
}

void instr_rol(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t val = cpu->registers.regs[rs];
    cpu_flag_set(cpu, SR_C, (val & 0x80u) != 0);
    cpu->registers.regs[rd] = (uint8_t)((val << 1) | (val >> 7));
    sr_logic(cpu, cpu->registers.regs[rd]);
}

void instr_ror(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t val = cpu->registers.regs[rs];
    cpu_flag_set(cpu, SR_C, (val & 0x01u) != 0);
    cpu->registers.regs[rd] = (uint8_t)((val >> 1) | (val << 7));
    sr_logic(cpu, cpu->registers.regs[rd]);
}

void instr_cmp(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t a = cpu->registers.regs[rd];
    uint8_t b = cpu->registers.regs[rs];
    uint8_t r = (uint8_t)(a - b);
    sr_sub8(cpu, a, b, 0, r);
}

void instr_tst(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    uint8_t r = (uint8_t)(cpu->registers.regs[rd] & cpu->registers.regs[rs]);
    sr_logic(cpu, r);
}

void instr_mov(CPU *cpu) {
    uint8_t rd = cpu->ir.bytes[1] & 0x0F;
    uint8_t rs = (cpu->ir.bytes[0] >> 4) & 0x0F;
    cpu->registers.regs[rd] = cpu->registers.regs[rs];
    sr_set_zn(cpu, cpu->registers.regs[rd]);
}

void instr_neg(CPU *cpu) {
    uint8_t rd = (cpu->ir.bytes[1] >> 4) & 0x0F;
    uint8_t a = cpu->registers.regs[rd];
    uint8_t r = (uint8_t)(0u - a);
    cpu->registers.regs[rd] = r;
    cpu_flag_set(cpu, SR_C, a == 0);
    cpu_flag_set(cpu, SR_V, a == 0x80u);
    sr_set_zn(cpu, r);
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
    sr_set_zn(cpu, imm);
    cpu_flag_set(cpu, SR_V, 0);
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

/* ===== BR-type instructions ===== */

void instr_br_r(CPU *cpu) {

    DoubleRegister ir = cpu->ir;

    // extract the information
    uint8_t absolute = (ir.high & 0x08) >> 3; // extract bit 11
    uint8_t condition = ir.high & 0x07; // extract bits 10:8
    DoubleRegister source = {
        .low = cpu->registers.regs[(ir.low >> 4) & 0x0F],
        .high = cpu->registers.regs[ir.low & 0x0F]
    };
    
    // evaluate conditions
    uint8_t take = 0;
    switch (condition) {
        case 0: take = 1; break;
        case 1: take = cpu_flag_get(cpu, SR_Z); break;
        case 2: take = (uint8_t)!cpu_flag_get(cpu, SR_Z); break;
        case 3: take = cpu_flag_get(cpu, SR_C); break;
        case 4: take = (uint8_t)!cpu_flag_get(cpu, SR_C); break;
        case 5: take = cpu_flag_get(cpu, SR_N); break;
        case 6: take = cpu_flag_get(cpu, SR_V); break;
        default: take = 0; break;
    }

    if (!take)
        return;

    if (absolute)
        cpu->registers.pc.value = source.value;
    else
        cpu->registers.pc.value = (uint16_t)(cpu->registers.pc.value + (int16_t)source.value);
}

/* ===== BI-type instructions ===== */

void instr_br_i(CPU *cpu) {

    DoubleRegister ir = cpu->ir;

    // extract the information
    uint8_t absolute = (ir.high & 0x08) >> 3; // extract bit 11
    uint8_t condition = ir.high & 0x07; // extract bits 10:8

    // evaluate conditions
    uint8_t take = 0;
    switch (condition) {
        case 0: take = 1; break;
        case 1: take = cpu_flag_get(cpu, SR_Z); break;
        case 2: take = (uint8_t)!cpu_flag_get(cpu, SR_Z); break;
        case 3: take = cpu_flag_get(cpu, SR_C); break;
        case 4: take = (uint8_t)!cpu_flag_get(cpu, SR_C); break;
        case 5: take = cpu_flag_get(cpu, SR_N); break;
        case 6: take = cpu_flag_get(cpu, SR_V); break;
        default: take = 0; break;
    }

    if (!take)
        return;

    // set the pc to the calculated address
    if (absolute)
        cpu->registers.pc.value = (uint16_t)ir.low;
    else
        cpu->registers.pc.value = (uint16_t)(cpu->registers.pc.value + (int16_t)(int8_t)ir.low);
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