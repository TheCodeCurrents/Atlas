#pragma once
#include <stdint.h>
#include "cpu.h"

#define ATLAS_OPCODE_COUNT 256

/// @brief Instruction function type
typedef void (*InstrFn)(CPU *cpu);

/// @brief Global instruction dispatch table
extern InstrFn atlas_dispatch[ATLAS_OPCODE_COUNT];

/// @brief Initialize instruction dispatch table
void instructions_init(void);

/* ===== Instruction prototypes ===== */

void instr_illegal(CPU *cpu);
void instr_nop(CPU *cpu);
void instr_halt(CPU *cpu);

/* Example ALU instruction */
void instr_add(CPU *cpu);
