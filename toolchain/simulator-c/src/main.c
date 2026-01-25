#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <time.h>

#include "cpu.h"
#include "instructions.h"

#define INSR_CNT 50


int main() {

    CPU cpu;
    
    cpu_init(&cpu, 65536);  // Initialize CPU with 64KB of memory

    // Load a simple program into memory
    cpu.memory[1] = 0x11; // LDI R1, 0x08
    cpu.memory[0] = 0x08;
    cpu.memory[3] = 0x12; // LDI R2, 0x01
    cpu.memory[2] = 0x01;
    cpu.memory[5] = 0x01; // SUB R1, R2
    cpu.memory[4] = 0x22;
    cpu.memory[7] = 0x00; // CMP R0, R1
    cpu.memory[6] = 0x1C;
    cpu.memory[9] = 0x4A;  // BEQ instruction
    cpu.memory[8] = 0x04;  // absolute branch to address 4

    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);
    

    for (int i = 0; i < INSR_CNT; i++) {
        cpu_step(&cpu);

        // print the pc
        printf("PC: 0x%04X\n", cpu.registers.pc.value);

        // printf("R0: 0x%02X\n", cpu.registers.regs[0]);
        // printf("R1: 0x%02X\n", cpu.registers.regs[1]);
        // printf("R2: 0x%02X\n", cpu.registers.regs[2]);
        // printf("R3: 0x%02X\n", cpu.registers.regs[3]);
        // printf("R4: 0x%02X\n\n", cpu.registers.regs[4]);
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