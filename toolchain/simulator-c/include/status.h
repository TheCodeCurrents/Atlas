#pragma once

#include <stdint.h>
#include "cpu.h"

static inline void sr_set_zn(CPU *cpu, uint8_t result)
{
    cpu_flag_set(cpu, SR_Z, result == 0);
    cpu_flag_set(cpu, SR_N, (result & 0x80u) != 0);
}

static inline void sr_logic(CPU *cpu, uint8_t result)
{
    sr_set_zn(cpu, result);
    cpu_flag_set(cpu, SR_V, 0);
}

static inline void sr_add8(CPU *cpu, uint8_t a, uint8_t b, uint8_t carry_in, uint8_t result)
{
    uint16_t wide = (uint16_t)a + (uint16_t)b + (uint16_t)carry_in;
    cpu_flag_set(cpu, SR_C, wide > 0xFFu);
    cpu_flag_set(cpu, SR_V, ((~(a ^ b) & (a ^ result)) & 0x80u) != 0);
    sr_set_zn(cpu, result);
}

static inline void sr_sub8(CPU *cpu, uint8_t a, uint8_t b, uint8_t borrow_in, uint8_t result)
{
    uint16_t subtrahend = (uint16_t)b + (uint16_t)borrow_in;
    cpu_flag_set(cpu, SR_C, (uint16_t)a >= subtrahend);
    cpu_flag_set(cpu, SR_V, (((a ^ b) & (a ^ result)) & 0x80u) != 0);
    sr_set_zn(cpu, result);
}
