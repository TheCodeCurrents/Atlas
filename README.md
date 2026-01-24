# Atlas

```text
Atlas/
├── docs/               # Documentation, design notes, whitepapers
│   │   └── sims/       # Simulations, testbenches
│   └── future_cpu/     # Placeholder for future CPU designs
├── software/           # Software ecosystem
│   ├── os/             # Operating system, kernels, drivers
│   ├── programs/       # Example programs, demos, benchmarks
│   └── bare/           # Bare-metal examples, firmware
├── computer/           # System-level concepts
│   ├── ram/            # RAM designs, memory controller ideas
│   ├── gpu/            # GPU ideas or experiments
│   ├── motherboard/    # Mainboard / bus concepts
│   └── peripherals/    # I/O, network, storage, etc.
├── toolchain/          # Assemblers, compilers, binutils, linkers
│   └── future_cpu/     # Placeholder for future CPU designs
├── software/           # Software ecosystem
│   ├── os/             # Operating system, kernels, drivers
│   ├── programs/       # Example programs, demos, benchmarks
│   └── utils/          # Build scripts, helpers, testing tools
└── tests/              # Integration tests, verification
    ├── cpu/            # CPU-specific tests
    ├── software/       # Software tests and OS validation
    └── system/         # Full computer system tests
│   └── peripherals/    # I/O, network, storage, etc.
├── toolchain/          # Assemblers, compilers, binutils, linkers
│   ├── assembler/
│   ├── compiler/
│   ├── simulator/
│   ├── emulator/
│   └── utils/          # Build scripts, helpers, testing tools
└── tests/              # Integration tests, verification
    ├── cpu/            # CPU-specific tests
    ├── software/       # Software tests and OS validation
    └── system/         # Full computer system tests
```