# Atlas

This is the **Atlas project**. My attempt at creating an entire computational ecosystem completely *from scratch*. Including everything from the ISA, over the actual CPU implementation up to the multi-tasking operating system and it's programs.

As you might imagine, this is meant to me a long term project of mine.


## Structure

When adding new things to this project, please follow this concept:

```text
Atlas/
├── docs/ # Documentation, design notes, whitepapers
│   ├── isa/ # ISA specifications per CPU architecture
│   ├── arch/ # Architecture concepts (Atlas8, future CPUs)
│   ├── hw/ # Hardware manuals, schematics, memory mapping
│   └── software/ # Software ecosystem documentation, OS design notes
├── cpu/ # CPU definitions, ISAs, and implementations
│   ├── atlas8/ # Atlas8 specific stuff
│   │   ├── isa/ # Instruction set definitions, encoding tables
│   │   ├── rtl/ # Hardware implementations
│   │   │   ├── vhdl/
│   │   │   ├── verilog/
│   │   │   └── schematics/
│   │   └── sims/ # Simulations, testbenches
│   └── future_cpu/ # Placeholder for future CPU designs
├── software/ # Software ecosystem
│   ├── os/ # Operating system, kernels, drivers
│   ├── programs/ # Example programs, demos, benchmarks
│   └── bare/ # Bare-metal examples, firmware
├── computer/ # System-level concepts
│   ├── ram/ # RAM designs, memory controller ideas
│   ├── gpu/ # GPU ideas or experiments
│   ├── motherboard/ # Mainboard / bus concepts
│   └── peripherals/ # I/O, network, storage, etc.
├── toolchain/ # Assemblers, compilers, binutils, linkers
│   ├── assembler/
│   ├── compiler/
│   ├── simulator/
│   ├── emulator/
│   └── utils/ # Build scripts, helpers, testing tools
└── tests/ # Integration tests, verification
    ├── cpu/ # CPU-specific tests
    ├── software/ # Software tests and OS validation
    └── system/ # Full computer system tests
```