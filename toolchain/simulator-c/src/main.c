#include <stdio.h>

#include "cpu.h"

CPU cpu;

uint8_t running = 1;

int main() {
    cpu_init(&cpu, 65536);  // Initialize CPU with 64KB of memory

    printf("Hello, World! %i   \n", cpu.registers.pc.value);

    while (running)
    {
        cpu.registers.pc.value++;  // Increment program counter
        printf("PC: %04X\n", cpu.registers.pc.value);

        // wait timer
        for (volatile int i = 0; i < 1000000; i++);
    }
    
    return 0;
}