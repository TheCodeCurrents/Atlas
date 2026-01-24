#include "instructions.h"
#include <stddef.h>
#include <stdio.h>

/* ===== Dispatch table ===== */

InstrFn atlas_dispatch[ATLAS_OPCODE_COUNT];

/* ===== Instruction implementations ===== */

void instr_illegal(CPU *cpu)
{
    /* For now: simple trap */
    (void)cpu;
    fprintf(stderr, "Illegal instruction: 0x%02X\n", cpu->ir.bytes[0]);
    /* Later: set a fault flag or halt */
}

void instr_nop(CPU *cpu)
{
    (void)cpu;
}

void instr_halt(CPU *cpu)
{
    /* Simplest possible halt:
       overwrite PC so execution stops */
    cpu->registers.pc.value--; 
}

/* Example:
   ADD rD, rS
   Encoding:
     opcode
     operand byte: [ rD | rS ]
*/
void instr_add(CPU *cpu)
{
    uint8_t operand = cpu->memory[cpu->registers.pc.value++];

    uint8_t rd = (operand >> 4) & 0x0F;
    uint8_t rs = operand & 0x0F;

    cpu->registers.regs[rd] += cpu->registers.regs[rs];
}

/// @brief Subtract instruction
/// @param cpu 
void instr_sub(CPU *cpu)
{
    uint8_t operand = cpu->memory[cpu->registers.pc.value++];

    uint8_t rd = (operand >> 4) & 0x0F;
    uint8_t rs = operand & 0x0F;

    cpu->registers.regs[rd] -= cpu->registers.regs[rs];
}

/* ===== Initialization ===== */

void instructions_init(void)
{
    for (int i = 0; i < ATLAS_OPCODE_COUNT; ++i)
        atlas_dispatch[i] = instr_illegal;

    atlas_dispatch[0x01] = instr_add;
    atlas_dispatch[0x02] = instr_sub;
    atlas_dispatch[0xFE] = instr_nop;
    atlas_dispatch[0xFF] = instr_halt;
}