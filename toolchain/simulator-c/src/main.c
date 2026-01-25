#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <time.h>

#include "cpu.h"
#include "instructions.h"

#define INSR_CNT 5


int main() {

    CPU cpu;
    
    cpu_init(&cpu, 65536);  // Initialize CPU with 64KB of memory

    // Load a simple program into memory
    cpu.memory[1] = 0x11; // LDI R1, 0x42
    cpu.memory[0] = 0x42;
    cpu.memory[3] = 0x12; // LDI R2, 0x05
    cpu.memory[2] = 0x27;
    cpu.memory[5] = 0x01; // ADD R1, R2
    cpu.memory[4] = 0x20;
    

    cpu.memory[7] = 0x04; // MOV R4, R1 (opcode 15, rs=1, rd=4)
    cpu.memory[6] = 0x1F;

    cpu.memory[9] = 0x23; // LD R3, R4+0
    cpu.memory[8] = 0x40;

    cpu.memory[105] = 0x23;

    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);
    

    for (int i = 0; i < INSR_CNT; i++) {
        cpu_step(&cpu);
    }
    
    clock_gettime(CLOCK_MONOTONIC, &end);

    double elapsed_seconds = (end.tv_sec - start.tv_sec) + (end.tv_nsec - start.tv_nsec) / 1e9;
    double mips = INSR_CNT / (elapsed_seconds * 1000000);

    printf("R0: 0x%02X\n", cpu.registers.regs[0]);
    printf("R1: 0x%02X\n", cpu.registers.regs[1]);
    printf("R2: 0x%02X\n", cpu.registers.regs[2]);
    printf("R3: 0x%02X\n", cpu.registers.regs[3]);
    printf("R4: 0x%02X\n", cpu.registers.regs[4]);
    
    printf("\nExecution time: %.9f seconds\n", elapsed_seconds);
    printf("Instructions Per Second: %.0f MIPS\n", mips);
    
    return 0;
}