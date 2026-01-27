# Atlas Assembler

A clean, production-ready assembler library for the 16-bit Atlas ISA.

## Quick Start (Library)

```rust
use assembler::Assembler;

let source = r#"
    add r0, r1
    ldi r2, 255
    ld r3, [r4 + 0]
"#.to_string();

match Assembler::assemble(source) {
    Ok(result) => {
        // result.linked_ir - intermediate representation with addresses
        // result.machine_code - encoded instructions ready for execution
        // result.symbol_table - resolved symbols
    }
    Err(e) => println!("Error: {}", e),
}
```

## CLI Usage

Assemble a single file:
```bash
./target/debug/assembler input.asm output.hex
```

Assemble all .asm files in a folder:
```bash
./target/debug/assembler ./asm_programs/ output.hex
```

Output format: address | hex code | mnemonic

## Features

- ✅ All 7 instruction types (A, I, M, BI, BR, S, P, X)
- ✅ Multi-file assembly with linking
- ✅ Proper sign-bit preservation
- ✅ Structured error handling (no panics)
- ✅ Clean API, pure library (no I/O or printing)

## Run Tests

```bash
cargo run --quiet
```

Assembles 8 test programs into 43 instructions → 86 bytes of machine code.

## Library vs CLI

**Core Library** (`src/lib.rs`)
- `Assembler::assemble(source) → Result<AssemblyResult, AssemblerError>`
- Zero panics, no side effects, deterministic

**CLI** (`src/main.rs`)
- Demonstrates all instruction types
- Prints linked IR and machine code
- Shows error handling
