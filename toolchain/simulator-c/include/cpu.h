#pragma once
#include <stdint.h>

/// @brief Single register type
typedef uint8_t Register;

/// @brief Double register structure (eg. for PC, SP, TR)
typedef union {
    uint16_t value;
    uint8_t bytes[2];
    struct {
        uint8_t low;
        uint8_t high;
    };
    
} DoubleRegister;

/// @brief Register file structure
typedef union {
    Register regs[16];
    struct {
        Register r0;
        Register r1;
        Register r2;
        Register r3;
        Register r4;
        Register r5;
        Register r6;
        Register r7;
        Register r8;
        Register r9;
        DoubleRegister tr;  // temporary register
        DoubleRegister sp;  // stack pointer
        DoubleRegister pc;  // program counter
    };
} RegisterFile;

/// @brief CPU structure
typedef struct {
    RegisterFile registers;
    DoubleRegister ir;  // instruction register
    uint8_t* memory;    
} CPU;


/// @brief Initialize the CPU with the given memory size
/// @param cpu Pointer to the CPU structure to initialize
/// @param memory_size Size of the memory to allocate for the CPU
void cpu_init(CPU *cpu, uint32_t memory_size);


/// @brief Execute one instruction
/// @param cpu Pointer to the CPU structure
void cpu_step(CPU *cpu);