#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <time.h>

#include "cpu.h"
#include "instructions.h"

#define BENCHMARK_ITERATIONS 100000000

/* ============================================
 * Program Loaders
 * ============================================ */

/// @brief Load a program into memory at the specified address
/// @param cpu Pointer to the CPU structure
/// @param program Byte array of the program
/// @param program_size Size of the program in bytes
/// @param start_address Address to load the program at
void load_program(CPU *cpu, const uint8_t *program, size_t program_size, uint16_t start_address)
{
    if (start_address + program_size > 65536) {
        fprintf(stderr, "Error: Program too large for memory at address 0x%04X\n", start_address);
        return;
    }
    
    memcpy(&cpu->memory[start_address], program, program_size);
    cpu->registers.pc.value = start_address;
}

/// @brief Load an instruction sequence (for testing)
/// @param cpu Pointer to the CPU structure
/// @param instructions Byte array of instructions
/// @param instruction_count Number of 16-bit instructions
void load_instructions(CPU *cpu, const uint16_t *instructions, size_t instruction_count)
{
    for (size_t i = 0; i < instruction_count; i++) {
        uint16_t instr = instructions[i];
        cpu->memory[i * 2] = (uint8_t)(instr & 0xFF);
        cpu->memory[i * 2 + 1] = (uint8_t)((instr >> 8) & 0xFF);
    }
    cpu->registers.pc.value = 0;
}

/// @brief Reset CPU state for next test
void reset_cpu(CPU *cpu)
{
    memset(cpu->registers.regs, 0, sizeof(cpu->registers.regs));
    cpu->registers.pc.value = 0;
    cpu->registers.sp.value = 0xFFFF;
    cpu->registers.tr.value = 0;
    cpu->sr = 0;
}

/// @brief Print pass/fail with formatting
void print_result(uint8_t pass)
{
    if (pass) printf("  ✓ PASS\n");
    else printf("  ✗ FAIL\n");
}

/* ============================================
 * Simple Instruction Tests
 * ============================================ */

/// @brief Test basic arithmetic operations
void test_arithmetic(void)
{
    printf("\n========== ARITHMETIC TESTS ==========\n"); 
    
    CPU cpu;
    cpu_init(&cpu, 65536);
    
    // I-type instruction encoding (little-endian):
    // 16-bit: (type << 12) | (rd << 8) | imm
    // Stored: memory[addr] = low byte, memory[addr+1] = high byte
    
    // Test 1: LDI + ADDI
    printf("Test 1: LDI R1, 0x10 -> ADDI R1, 0x05 (Expected R1: 0x15)\n");
    reset_cpu(&cpu);
    
    uint16_t prog1[] = {
        0x1110,  // LDI R1, 0x10: (1<<12)|(1<<8)|0x10
        0x2105   // ADDI R1, 0x05: (2<<12)|(1<<8)|0x05
    };
    load_instructions(&cpu, prog1, 2);
    
    // Debug: print memory
    printf("  Instruction 0: 0x%02X 0x%02X (raw: 0x%04X)\n", cpu.memory[0], cpu.memory[1], prog1[0]);
    printf("  Instruction 1: 0x%02X 0x%02X (raw: 0x%04X)\n", cpu.memory[2], cpu.memory[3], prog1[1]);
    
    cpu_step(&cpu);
    printf("  After LDI: R1=0x%02X (expected 0x10), PC=0x%04X\n", cpu.registers.regs[1], cpu.registers.pc.value);
    cpu_step(&cpu);
    printf("  After ADDI: R1=0x%02X (expected 0x15), PC=0x%04X\n", cpu.registers.regs[1], cpu.registers.pc.value);
    printf("\n");
    
    // Test 3: ORI operation
    printf("Test 3: LDI R3, 0xF0 -> ORI R3, 0x0F (Expected R3: 0xFF)\n");
    reset_cpu(&cpu);
    
    uint16_t prog3[] = {
        0x13F0,  // LDI R3, 0xF0: (1<<12)|(3<<8)|0xF0
        0x530F   // ORI R3, 0x0F: (5<<12)|(3<<8)|0x0F
    };
    load_instructions(&cpu, prog3, 2);
    
    cpu_step(&cpu);
    printf("  After LDI: R3=0x%02X (expected 0xF0)\n", cpu.registers.regs[3]);
    cpu_step(&cpu);
    printf("  After ORI: R3=0x%02X (expected 0xFF)\n", cpu.registers.regs[3]);
    print_result(cpu.registers.regs[3] == 0xFF);
    printf("\n");
}

/// @brief Test register-to-register operations
void test_register_operations(void)
{
    printf("\n========== REGISTER OPERATION TESTS ==========\n");
    
    CPU cpu;
    cpu_init(&cpu, 65536);
    
    // A-type instruction encoding (little-endian):
    // 16-bit: (0<<12) | (rd<<8) | (rs<<4) | opcode
    
    // Test 1: MOV instruction (opcode 0xE)
    printf("Test 1: LDI R4, 0x42 -> MOV R5, R4 (Expected R5: 0x42)\n");
    reset_cpu(&cpu);
    
    uint16_t prog1[] = {
        0x1442,  // LDI R4, 0x42: (1<<12)|(4<<8)|0x42
        0x054E   // MOV R5, R4: (0<<12)|(5<<8)|(4<<4)|0xE
    };
    load_instructions(&cpu, prog1, 2);
    
    cpu_step(&cpu);
    printf("  After LDI: R4=0x%02X (expected 0x42)\n", cpu.registers.regs[4]);
    cpu_step(&cpu);
    printf("  After MOV: R5=0x%02X (expected 0x42)\n", cpu.registers.regs[5]);
    print_result(cpu.registers.regs[5] == 0x42);
    printf("\n");
    
    // Test 2: CMP instruction (opcode 0xC)
    printf("Test 2: LDI R6, 0x10 -> CMP R6, R6 (Expected SR: Z flag set)\n");
    reset_cpu(&cpu);
    
    uint16_t prog2[] = {
        0x1610,  // LDI R6, 0x10: (1<<12)|(6<<8)|0x10
        0x066C   // CMP R6, R6: (0<<12)|(6<<8)|(6<<4)|0xC
    };
    load_instructions(&cpu, prog2, 2);
    
    cpu_step(&cpu);
    printf("  After LDI: R6=0x%02X (expected 0x10)\n", cpu.registers.regs[6]);
    cpu_step(&cpu);
    printf("  After CMP: R6=0x%02X (should be unchanged: 0x10)\n", cpu.registers.regs[6]);
    printf("  SR: 0x%02X\n", cpu.sr);
    printf("    C (carry):  %d (expected 0)\n", cpu_flag_get(&cpu, SR_C));
    printf("    Z (zero):   %d (expected 1)\n", cpu_flag_get(&cpu, SR_Z));
    printf("    N (neg):    %d (expected 0)\n", cpu_flag_get(&cpu, SR_N));
    printf("    V (oflow):  %d (expected 0)\n", cpu_flag_get(&cpu, SR_V));
    print_result(cpu_flag_get(&cpu, SR_Z) == 1);
    printf("\n");
}

/// @brief Test immediate operations chain
void test_operation_chain(void)
{
    printf("\n========== OPERATION CHAIN TEST ==========\n");
    
    CPU cpu;
    cpu_init(&cpu, 65536);
    
    printf("Test: LDI R7, 0x05 -> ADDI R7, 0x0A -> SUBI R7, 0x03\n");
    printf("Expected R7: (5 + 10 - 3) = 12 = 0x0C\n");
    
    reset_cpu(&cpu);
    
    uint16_t prog[] = {
        0x1705,  // LDI R7, 0x05: (1<<12)|(7<<8)|0x05
        0x270A,  // ADDI R7, 0x0A: (2<<12)|(7<<8)|0x0A
        0x3703   // SUBI R7, 0x03: (3<<12)|(7<<8)|0x03
    };
    load_instructions(&cpu, prog, 3);
    
    cpu_step(&cpu);
    printf("  After LDI: R7=0x%02X\n", cpu.registers.regs[7]);
    cpu_step(&cpu);
    printf("  After ADDI: R7=0x%02X\n", cpu.registers.regs[7]);
    cpu_step(&cpu);
    printf("  After SUBI: R7=0x%02X (expected 0x0C)\n", cpu.registers.regs[7]);
    print_result(cpu.registers.regs[7] == 0x0C);
    printf("\n");
}

/* ============================================
 * Benchmark
 * ============================================ */

/// @brief Benchmark CPU performance with mixed instruction workload
void benchmark_cpu(void)
{
    printf("\n========== BENCHMARK ==========\n");
    
    CPU cpu;
    cpu_init(&cpu, 65536);
    
    // Create a repeating instruction sequence
    uint16_t prog[] = {
        0x1101,  // LDI R1, 0x01
        0x2105,  // ADDI R1, 0x05
        0x420F,  // ANDI R1, 0x0F
        0x5110,  // ORI R1, 0x10
        0x3104,  // SUBI R1, 0x04
        0x8000   // BR 0x00
    };
    
    load_instructions(&cpu, prog, 6);
    
    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);
    
    // Execute instructions multiple times
    for (int i = 0; i < BENCHMARK_ITERATIONS; i++) {
        cpu_step(&cpu);
    }
    
    clock_gettime(CLOCK_MONOTONIC, &end);
    
    double elapsed_seconds = (end.tv_sec - start.tv_sec) + (end.tv_nsec - start.tv_nsec) / 1e9;
    double mips = BENCHMARK_ITERATIONS / (elapsed_seconds * 1000000);
    uint64_t loop_iterations = BENCHMARK_ITERATIONS / 6;  // 6-instruction loop
    
    printf("Loop iterations: %llu\n", loop_iterations);
    printf("Total instructions executed: %d\n", BENCHMARK_ITERATIONS);
    printf("Time: %.9f seconds\n", elapsed_seconds);
    printf("Instructions Per Second: %.2f MIPS\n", mips);
    printf("Time per instruction: %.2f nanoseconds\n", (elapsed_seconds / BENCHMARK_ITERATIONS) * 1e9);
}

/* ============================================
 * Main Test Runner
 * ============================================ */

int main(void)
{
    printf("Atlas Simulator - Test Suite\n");
    printf("========================================\n");
    
    // Initialize instruction dispatch table
    instructions_init();
    
    // Run tests
    test_arithmetic();
    test_register_operations();
    test_operation_chain();
    benchmark_cpu();
    
    printf("\n========================================\n");
    printf("Tests Complete\n");
    
    return 0;
}
