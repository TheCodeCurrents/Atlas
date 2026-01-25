#pragma once
#include <stdint.h>
#include "cpu.h"

/// @brief Instruction type field values (bits 15:12)
/// Note: Some instruction types occupy multiple type field values
typedef enum {
    INSTR_TYPE_A = 0x0,      // Arithmetic and Logical (type field = 0)
    INSTR_TYPE_I = 0x1,      // Immediate (type field = 1)
    INSTR_TYPE_M_0 = 0x2,    // Memory (type field = 2 or 3)
    INSTR_TYPE_M_1 = 0x3,
    INSTR_TYPE_BI = 0x4,     // Branch Immediate (type field = 4)
    INSTR_TYPE_BR = 0x5,     // Branch Register (type field = 5)
    INSTR_TYPE_S_0 = 0x6,    // Stack (type field = 6 or 7)
    INSTR_TYPE_S_1 = 0x7
} InstrType;

/// @brief A-type instruction opcodes (bits 3:0)
typedef enum {
    A_OP_ADD = 0,
    A_OP_ADDC = 1,
    A_OP_SUB = 2,
    A_OP_SUBC = 3,
    A_OP_AND = 4,
    A_OP_OR = 5,
    A_OP_XOR = 6,
    A_OP_NOT = 7,
    A_OP_SHL = 8,
    A_OP_SHR = 9,
    A_OP_ROL = 10,
    A_OP_ROR = 11,
    A_OP_CMP = 12,
    A_OP_TST = 13,
    A_OP_NEG = 14,
    A_OP_MOV = 15,
    A_OP_COUNT = 16
} ATypeOpcode;

/// @brief Instruction function type
typedef void (*InstrFn)(CPU *cpu);

/// @brief Global instruction dispatch table for A-type instructions
extern InstrFn atlas_dispatch_a[A_OP_COUNT];

/// @brief Initialize instruction dispatch table
void instructions_init(void);

/* ===== Instruction prototypes ===== */

void instr_illegal(CPU *cpu);

/* A-type instructions */
void instr_add(CPU *cpu);
void instr_addc(CPU *cpu);
void instr_sub(CPU *cpu);
void instr_subc(CPU *cpu);
void instr_and(CPU *cpu);
void instr_or(CPU *cpu);
void instr_xor(CPU *cpu);
void instr_not(CPU *cpu);
void instr_shl(CPU *cpu);
void instr_shr(CPU *cpu);
void instr_rol(CPU *cpu);
void instr_ror(CPU *cpu);
void instr_cmp(CPU *cpu);
void instr_tst(CPU *cpu);
void instr_mov(CPU *cpu);
void instr_neg(CPU *cpu);

/* I-type instructions */
void instr_ldi(CPU *cpu);

/* M-type instructions */
void instr_ld(CPU *cpu); // load from address in register
void instr_st(CPU *cpu); // store to address in register

/* BI-type instructions */
void instr_bi(CPU *cpu);

/* BR-type instructions */
void instr_br(CPU *cpu);

/* S-type instructions */
void instr_push(CPU *cpu);
void instr_pop(CPU *cpu);
void instr_call(CPU *cpu);
void instr_ret(CPU *cpu);

/* Extended instructions can be added here */
void instr_nop(CPU *cpu);
void instr_halt(CPU *cpu);
void instr_syscall(CPU *cpu);
