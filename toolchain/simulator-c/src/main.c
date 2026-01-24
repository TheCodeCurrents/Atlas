#include <stdio.h>

#include "cpu.h"
#include "instructions.h"

CPU cpu;

uint8_t running = 1;

int main() {
    cpu_init(&cpu, 65536);  // Initialize CPU with 64KB of memory

    printf("Hello, World! %i   \n", cpu.registers.pc.value);

    while (running)
    {
        // fetch instruction
        cpu.ir.bytes[0] = cpu.memory[cpu.registers.pc.value++];
        cpu.ir.bytes[1] = cpu.memory[cpu.registers.pc.value++];

        // execute instruction
        
    }
    
    return 0;
}