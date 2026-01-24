// Import unified documentation module
#import "../documentation.typ": *

// Document configuration
#let title = "Atlas-8 ISA Specification"
#let version = "1.0"
#let date = "January 2026"
#let author = "Atlas Project"
#let overview = "Complete instruction set architecture specification for the Atlas-8 processor, including the base instruction set and optional ISA extensions for caching and memory management."

#doc-title(title, version: version, date: date, author: author, overview: overview)

#outline()

#pagebreak()

= Overview

Quick summary of the Atlas-8 ISA philosophy and design goals. This document describes the instruction set architecture of the Atlas-8 processor, including the base instruction set and optional ISA extensions.

#section-marker("The Atlas-8 follows a modular extension model similar to RISC-V, allowing configurations with different feature sets while maintaining core compatibility.", color: color-base)

= Registers & Memory Model

== Register File

Description of the register file, including:
- Number of registers
- Register width
- Register organization and naming conventions

== Memory Organization

Physical memory layout and organization details.

== Addressing Modes

#section-marker([Base ISA], color: color-base) Addressing modes available in the base instruction set:
- Immediate
- Register
- Register + Offset
- [etc.]

Additional addressing modes provided by extensions are documented in their respective sections.

= Instruction Format

== Encoding Scheme

Description of how instructions are encoded.

== Instruction Width

Base instruction width and variable-length instruction support (if applicable).

== Opcode Layout

Bit field layout and opcode organization.

= Base Instruction Set Reference

== Arithmetic & Logic Instructions

#section-marker([Base ISA], color: color-base) Core arithmetic and logic operations:
- ADD, SUB, MUL, DIV
- AND, OR, XOR, NOT
- [etc.]

== Memory Instructions (Load/Store)

#section-marker([Base ISA], color: color-base) Memory access instructions:
- LOAD, STORE variants
- Memory addressing

== Control Flow Instructions

#section-marker([Base ISA], color: color-base) Branch and jump instructions:
- Conditional branches
- Unconditional jumps
- [etc.]

== Special/System Instructions

#section-marker([Base ISA], color: color-base) System-level instructions:
- NOP, HALT
- System calls
- [etc.]

= Extensions

Extensions provide optional functionality while maintaining compatibility with the base ISA.

== Cache Extension

#extension("CACHE") Optional L1 and L2 cache layers.

*Prerequisite:* None (hardware dependent)

*New Instructions:*
- CACHE_FLUSH
- CACHE_PREFETCH
- [etc.]

*Configuration Registers:* Cache control and status registers.

== MMU Extension

#extension("MMU") Optional Memory Management Unit for virtual memory support.

*Prerequisite:* None (hardware dependent)

*New Instructions:*
- TLBFLUSH
- SETTPB (Set Translation Page Base)
- [etc.]

*New Addressing Modes:*
- Virtual addressing
- [etc.]

*New Registers:* MMU control registers, TLB entries, etc.

= Exceptions & Interrupts

== Exception Types

Description of exception types:
- Invalid instruction
- Memory access violations
- [etc.]

== Interrupt Handling

Interrupt mechanism and handling procedures.

== Extension-Specific Exceptions

Exceptions introduced by optional extensions:
- #extension("MMU"): Page fault exceptions
- [etc.]

= Code Examples

== Base ISA Examples

Basic assembly examples using only base instructions:

#code-block(`; Add two numbers
ADD R1, R2, R3    ; R1 = R2 + R3
LOAD R4, [R5]     ; Load from memory
JUMP label        ; Unconditional jump`)

== Extension Examples

#extension("MMU") example with virtual memory:

#code-block(`; Virtual memory example
SETTPB R1         ; Set page table base
LOAD R2, [R3]     ; Load with virtual addressing
TLBFLUSH          ; Flush TLB`)

#extension("CACHE") example with cache operations:

#code-block(`; Cache management
CACHE_PREFETCH R1 ; Prefetch data
CACHE_FLUSH       ; Flush cache`)

= Extension Compatibility Matrix

#table(
  columns: (1fr, 1fr, 1fr, 1fr),
  [Feature], [Base], [Cache Ext], [MMU Ext],
  [Base Instructions], [✓], [✓], [✓],
  [Cache Control], [], [✓], [],
  [Virtual Addressing], [], [], [✓],
)

= Appendix: Quick Reference

== Instruction Quick Reference

/ ADD: Add two registers
/ SUB: Subtract two registers
/ LOAD: Load from memory
/ STORE: Store to memory
/ JUMP: Unconditional jump
