# Atlas

```text
Atlas/
├── docs/                   # Documentation, design notes, whitepapers
│   ├── isa/                # ISA specifications per CPU architecture
│   ├── arch/               # Architecture concepts (Atlas8, future CPUs)
│   ├── hw/                 # Hardware manuals, schematics, memory mapping
│   └── software/           # Software ecosystem documentation, OS design notes
│
├── cpu/                    # CPU definitions, ISAs, and implementations
│   ├── atlas8/             # Atlas8 specific stuff
│   │   ├── isa/            # Instruction set definitions, encoding tables
│   │   ├── rtl/            # Hardware implementations
│   │   │   ├── vhdl/
│   │   │   ├── verilog/
│   │   │   └── schematics/
│   │   └── sims/           # Simulations, testbenches
│   └── future_cpu/         # Placeholder for future CPU designs
│
├── software/               # Software ecosystem
│   ├── os/                 # Operating system, kernels, drivers
│   ├── programs/           # Example programs, demos, benchmarks
│   └── bare/               # Bare-metal examples, firmware
│
├── computer/               # System-level stuff
│   ├── ram/                # RAM designs, memory controller ideas
│   ├── gpu/                # GPU ideas or experiments
│   ├── motherboard/        # Mainboard / bus concepts
│   └── peripherals/        # I/O, network, storage, etc.
│
├── toolchain/              # Assemblers, compilers, binutils, linkers
│   ├── assembler/
│   ├── compiler/
│   ├── simulator/
│   ├── emulator/
│   └── utils/              # Build scripts, helpers, testing tools
│
└── tests/                  # Integration tests, verification
    ├── cpu/                # CPU-specific tests
    ├── software/           # Software tests and OS validation
    ├── system/             # Full computer system tests
    └── peripherals/        # I/O, network, storage, etc.
```