#include "cpu.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/// @brief Initialize the CPU with the given memory size
/// @param cpu Pointer to the CPU structure to initialize
/// @param memory_size Size of the memory to allocate for the CPU
void cpu_init(CPU *cpu, uint32_t memory_size) {
    
    for (int i = 0; i < 16; i++) {
        cpu->registers.regs[i] = 0;     // Initialize all general-purpose registers to 0
    }
    cpu->ir.value = 0;                  // Initialize instruction register to 0
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

}