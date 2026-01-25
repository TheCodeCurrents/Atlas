#pragma once
#include <stdint.h>
#include "cpu.h"

/// @brief Instruction type field values (bits 15:12)
typedef enum {
    INSTR_TYPE_A = 0x0,           // Arithmetic and Logical
    INSTR_TYPE_I_LDI = 0x1,       // Load Immediate
    INSTR_TYPE_I_ADDI = 0x2,      // Add Immediate
    INSTR_TYPE_I_SUBI = 0x3,      // Subtract Immediate
    INSTR_TYPE_I_ANDI = 0x4,      // AND Immediate
    INSTR_TYPE_I_ORI = 0x5,       // OR Immediate
    INSTR_TYPE_M_LD = 0x6,        // Memory Load
    INSTR_TYPE_M_ST = 0x7,        // Memory Store
    INSTR_TYPE_BI = 0x8,          // Branch Immediate
    INSTR_TYPE_BR = 0x9,          // Branch Register
    INSTR_TYPE_S = 0xA,           // Stack Operations (xop in bits 11:8)
    INSTR_TYPE_P_PEEK = 0xB,      // Peek (read from SP + offset)
    INSTR_TYPE_P_POKE = 0xC,      // Poke (write to SP + offset)
    INSTR_TYPE_X = 0xD            // Extended (privileged, xop in bits 11:8)
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
    A_OP_MOV = 14,
    A_OP_NEG = 15,
    A_OP_COUNT = 16
} ATypeOpcode;

/// @brief S-type extended opcodes (bits 11:8)
typedef enum {
    S_OP_PUSH = 0x0,    // Push register onto stack
    S_OP_POP = 0x1,     // Pop register from stack
    S_OP_SUBSP_IMM = 0x2, // Allocate stack (immediate)
    S_OP_SUBSP_REG = 0x3, // Allocate stack (register)
    S_OP_ADDSP_IMM = 0x4, // Deallocate stack (immediate)
    S_OP_ADDSP_REG = 0x5   // Deallocate stack (register)
} STypeOpcode;

/// @brief X-type extended opcodes (bits 11:8)
typedef enum {
    X_OP_SYSCALL = 0x0,     // System call
    X_OP_ERET = 0x1,        // Return from exception
    X_OP_HALT = 0x2,        // Halt processor
    X_OP_MMU_ON = 0x4,      // Enable MMU
    X_OP_MMU_OFF = 0x5,     // Disable MMU
    X_OP_MMUWR = 0x6,       // Write MMU entry
    X_OP_MMURD = 0x7,       // Read MMU entry
    X_OP_MMUFLUSH = 0x8,    // Flush MMU translations
    X_OP_MMUPID = 0x9,      // Set active process ID
    X_OP_ICACHE_INV = 0xA,  // Invalidate instruction cache
    X_OP_DCACHE_INV = 0xB,  // Invalidate data cache
    X_OP_DCACHE_CLEAN = 0xC, // Clean data cache
    X_OP_CACHE_FLUSH = 0xD  // Flush all caches
} XTypeOpcode;

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
void instr_ldi(CPU *cpu);   // Load Immediate
void instr_addi(CPU *cpu);  // Add Immediate
void instr_subi(CPU *cpu);  // Subtract Immediate
void instr_andi(CPU *cpu);  // AND Immediate
void instr_ori(CPU *cpu);   // OR Immediate

/* M-type instructions */
void instr_ld(CPU *cpu);    // Load from memory
void instr_st(CPU *cpu);    // Store to memory

/* BI-type instructions */
void instr_br_i(CPU *cpu);  // Branch with immediate offset

/* BR-type instructions */
void instr_br_r(CPU *cpu);  // Branch with register offset

/* S-type instructions (stack operations) */
void instr_push(CPU *cpu);        // Push register onto stack
void instr_pop(CPU *cpu);         // Pop register from stack
void instr_subsp_imm(CPU *cpu);   // Allocate stack space (immediate)
void instr_subsp_reg(CPU *cpu);   // Allocate stack space (register)
void instr_addsp_imm(CPU *cpu);   // Deallocate stack space (immediate)
void instr_addsp_reg(CPU *cpu);   // Deallocate stack space (register)

/* P-type instructions (peek/poke - SP relative access) */
void instr_peek(CPU *cpu);  // Read from [SP + offset]
void instr_poke(CPU *cpu);  // Write to [SP + offset]

/* X-type instructions (extended/privileged operations) */
void instr_syscall(CPU *cpu);     // System call
void instr_eret(CPU *cpu);        // Return from exception
void instr_halt(CPU *cpu);        // Halt processor
void instr_mmu_on(CPU *cpu);      // Enable MMU
void instr_mmu_off(CPU *cpu);     // Disable MMU
void instr_mmuwr(CPU *cpu);       // Write MMU entry
void instr_mmurd(CPU *cpu);       // Read MMU entry
void instr_mmuflush(CPU *cpu);    // Flush MMU translations
void instr_mmupid(CPU *cpu);      // Set active process ID
void instr_icache_inv(CPU *cpu);  // Invalidate instruction cache
void instr_dcache_inv(CPU *cpu);  // Invalidate data cache
void instr_dcache_clean(CPU *cpu); // Clean data cache
void instr_cache_flush(CPU *cpu); // Flush all caches
