#include "cpu.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "instructions.h"

/// @brief Initialize the CPU with the given memory size
/// @param cpu Pointer to the CPU structure to initialize
/// @param memory_size Size of the memory to allocate for the CPU
void cpu_init(CPU *cpu, uint32_t memory_size) {
    
    for (int i = 0; i < 16; i++) {
        cpu->registers.regs[i] = 0;     // Initialize all general-purpose registers to 0
    }
    cpu->ir.value = 0;                  // Initialize instruction register to 0
    cpu->sr = 0;                        // Initialize status register
    cpu->registers.sp.value = 0xFFFF;   // Initialize stack pointer to top of memory

    // allocate memory
    cpu->memory = (uint8_t*)malloc(memory_size);
    if (cpu->memory == NULL) {
        // Handle memory allocation failure
        fprintf(stderr, "Failed to allocate memory for CPU\n");
        exit(EXIT_FAILURE);
    }

    // Initialize memory to zero
    memset(cpu->memory, 0, memory_size);

    // initialize instruction table
    instructions_init();

}


/* ===== Fetch–Decode–Execute ===== */

void cpu_step(CPU *cpu)
{
    // Fetch: Read 16-bit instruction (little-endian)
    cpu->ir.bytes[0] = cpu->memory[cpu->registers.pc.value++];
    cpu->ir.bytes[1] = cpu->memory[cpu->registers.pc.value++];

    // Decode: Extract type field (bits 15:12)
    // Assuming little-endian: byte[1] contains bits 15:8, byte[0] contains bits 7:0
    uint8_t type_field = (cpu->ir.bytes[1] >> 4) & 0x0F;

    // Execute: Dispatch based on type
    switch (type_field) {
        case INSTR_TYPE_A: {
            // A-type: Get opcode from bits 3:0
            uint8_t opcode = cpu->ir.bytes[0] & 0x0F;
            InstrFn fn = atlas_dispatch_a[opcode];
            fn(cpu);
            break;
        }
        case INSTR_TYPE_I:
            instr_ldi(cpu);
            break;
        case INSTR_TYPE_M_0:
            instr_ld(cpu);
            break;
        case INSTR_TYPE_M_1:
            instr_st(cpu);
            break;
        case INSTR_TYPE_BI:
            instr_br_i(cpu);
            break;
        case INSTR_TYPE_BR:
            instr_br_r(cpu);
            break;
        case INSTR_TYPE_S_0:
        case INSTR_TYPE_S_1:
            // TODO: Implement S-type instructions
            instr_illegal(cpu);
            break;
        default:
            // Extended or undefined instruction types
            instr_illegal(cpu);
            break;
    }
}