// Import unified documentation module
#include "../typst/init.typ";
#import "../typst/styling/heading.typ": *

// Document configuration
#let title = "Atlas-8 ISA Specification"
#let version = "1.0"
#let date = "January 2026"
#let author = "Atlas Project"
#let overview = "Complete instruction set architecture specification for the Atlas-8 processor, including the base instruction set and optional ISA extensions for caching and memory management."

#doc-title(title, version: version, date: date, author: author, overview: overview)

#outline()

= Overview

The Atlas-8 instruction set architecture (ISA) defines a machine model and the set of instructions that the Atlas-8 processor can execute.
This document provides a comprehensive specification of the ISA.

The Atlas-8 is a 8-bit RISC architecture designed for efficiency and simplicity, while supporting a variety of complex features like caching and memory management through optional extensions.
It uses fixed-length 16-bit instructions and a load/store architecture.

== Machine Model

The Atlas-8 processor has the following key components:
- 16 8-bit registers (R0-R15)
  -> 10 general-purpose registers (R0-R9)
  -> 3 16-bit (2x8-bit) special-purpose registers (R10-R15)
- 64KB of random access memory
- Program Counter (PC) and Status Register (SR)
- 2 cache levels (L1 and L2) for instruction and data caching
- Memory Management Unit (MMU) for virtual memory support (maps 16-bit virtual addresses to 24-bit physical addresses)

== Instruction Set

The Atlas-8 ISA includes a base instruction set and optional extensions for caching and memory management.
The base instruction set consists of arithmetic, logical, data movement, control flow, and system instructions.
Each instruction is 16 bits long and follows a fixed format based on it's type. There are 8 primary instruction formats: R-type, I-type, J-type, S-type, and U-type.



= Machine Model

The Atlas-8 processor implements a simplified yet powerful machine model designed to balance performance with implementation simplicity. This section details the core components and their interactions.

== Register File

The Atlas-8 contains 16 general-purpose and special-purpose registers, each 8 bits wide, providing a total of 128 bits of fast storage.

*General-Purpose Registers (R0-R9):*
These 10 registers are available for general computation and data storage. They follow standard register calling conventions:
- R0: Zero register (always reads as 0)
- R1-R3: Argument registers
- R4-R7: Return value and temporary registers
- R8-R9: Temporary and saved registers

*Special-Purpose Registers (R10-R15):*
These 6 registers are 16-bit wide (combining two 8-bit slots) and serve specific architectural functions:
- R10-R11 (16-bit): Temporary Register (TR)
- R12-R13 (16-bit): Stack Pointer (SP)
- R14-R15 (16-bit): Program Counter (PC)

Many instructions will treat either one of the pairs as a single 16-bit register for operations requiring larger data sizes.
There are also some instructions (like load/store) that have special codes that refer to the special-purpose registers directly.

== Memory Organization

The Atlas-8 has a 64 KB flat memory address space (16-bit addressing), providing 65,536 addressable bytes. Due to the MMU extension, physical memory can be expanded up to 16 MB (24-bit addressing). The memory organization is completely up to the system designer and highly flexible with the MMU extension. Only the MMUs page table structure is fixed.

0x000000-0x00FFFF: Addressable memory space (64 KB)
0x010000-0xFFFFFF: Reserved for MMU-mapped physical memory (up to 16 MB)

Memory is obviously byte-addressable, and multi-byte data (16-bit and 32-bit values) follow little-endian byte ordering.

== Cache Hierarchy

The Atlas-8 ISA includes a cache extension that introduces a two-level cache hierarchy to improve memory access performance.

*Level 1 Cache (L1):*
- split cache I- and D-cache
- 32-byte cache lines
- 4-way associative

*Level 2 Cache (L2):*
- 8 KB unified cache
- 64-byte cache lines
- 8-way associative

Cache coherency is maintained through a write-through policy at L1 and write-back at L2.

== Memory Management Unit (MMU)

For systems requiring virtual memory support (enabled via MMU extension), the Atlas-8 includes a Memory Management Unit:

*Virtual to Physical Address Translation:*
- Inputs:
  - 16-bit virtual address
    - 6-bit virtual page number
    - 10-bit page offset
  - 8-bit process ID
- Output: 24-bit physical address

*Features:*
- 1 KB page size
- Up to 16 MB of addressable physical memory (24-bit addresses)
- User/supervisor privilege levels with memory protection

The MMU is optional and must be explicitly enabled through the ISA extension mechanism.

== Processor Modes

The Atlas-8 supports two privilege levels:

*User Mode:* Limited memory access and instruction set. Cannot execute privileged instructions or access system memory (0x8000-0xFFFF).

*Supervisor Mode:* Full instruction set and memory access. Required for operating system and system software.

Mode transitions occur through exception handling and return-from-exception instructions.

