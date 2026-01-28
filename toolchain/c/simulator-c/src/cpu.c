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
        case INSTR_TYPE_I_LDI:
            instr_ldi(cpu);
            break;
        case INSTR_TYPE_I_ADDI:
            instr_addi(cpu);
            break;
        case INSTR_TYPE_I_SUBI:
            instr_subi(cpu);
            break;
        case INSTR_TYPE_I_ANDI:
            instr_andi(cpu);
            break;
        case INSTR_TYPE_I_ORI:
            instr_ori(cpu);
            break;
        case INSTR_TYPE_M_LD:
            instr_ld(cpu);
            break;
        case INSTR_TYPE_M_ST:
            instr_st(cpu);
            break;
        case INSTR_TYPE_BI:
            instr_br_i(cpu);
            break;
        case INSTR_TYPE_BR:
            instr_br_r(cpu);
            break;
        case INSTR_TYPE_S: {
            // S-type: Decode extended opcode from bits 11:8
            uint8_t xop = (cpu->ir.bytes[1] >> 0) & 0x0F;  // Extract bits 11:8 as xop (in position 0-3 of high byte after removing type field)
            switch (xop) {
                case S_OP_PUSH:
                    instr_push(cpu);
                    break;
                case S_OP_POP:
                    instr_pop(cpu);
                    break;
                case S_OP_SUBSP_IMM:
                    instr_subsp_imm(cpu);
                    break;
                case S_OP_SUBSP_REG:
                    instr_subsp_reg(cpu);
                    break;
                case S_OP_ADDSP_IMM:
                    instr_addsp_imm(cpu);
                    break;
                case S_OP_ADDSP_REG:
                    instr_addsp_reg(cpu);
                    break;
                default:
                    instr_illegal(cpu);
                    break;
            }
            break;
        }
        case INSTR_TYPE_P_PEEK:
            instr_peek(cpu);
            break;
        case INSTR_TYPE_P_POKE:
            instr_poke(cpu);
            break;
        case INSTR_TYPE_X: {
            // X-type: Decode extended opcode from bits 11:8
            uint8_t xop = (cpu->ir.bytes[1] >> 0) & 0x0F;  // Extract bits 11:8 as xop
            switch (xop) {
                case X_OP_SYSCALL:
                    instr_syscall(cpu);
                    break;
                case X_OP_ERET:
                    instr_eret(cpu);
                    break;
                case X_OP_HALT:
                    instr_halt(cpu);
                    break;
                case X_OP_MMU_ON:
                    instr_mmu_on(cpu);
                    break;
                case X_OP_MMU_OFF:
                    instr_mmu_off(cpu);
                    break;
                case X_OP_MMUWR:
                    instr_mmuwr(cpu);
                    break;
                case X_OP_MMURD:
                    instr_mmurd(cpu);
                    break;
                case X_OP_MMUFLUSH:
                    instr_mmuflush(cpu);
                    break;
                case X_OP_MMUPID:
                    instr_mmupid(cpu);
                    break;
                case X_OP_ICACHE_INV:
                    instr_icache_inv(cpu);
                    break;
                case X_OP_DCACHE_INV:
                    instr_dcache_inv(cpu);
                    break;
                case X_OP_DCACHE_CLEAN:
                    instr_dcache_clean(cpu);
                    break;
                case X_OP_CACHE_FLUSH:
                    instr_cache_flush(cpu);
                    break;
                default:
                    instr_illegal(cpu);
                    break;
            }
            break;
        }
        default:
            // Undefined instruction type
            instr_illegal(cpu);
            break;
    }
}