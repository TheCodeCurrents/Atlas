
use assembler::Assembler;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input_file_or_folder> <output_file>", args[0]);
        eprintln!("  input_file_or_folder: .asm file or folder containing .asm files");
        eprintln!("  output_file: destination for machine code output");
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    // Collect all .asm files
    let asm_files = match collect_asm_files(input_path) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            process::exit(1);
        }
    };

    if asm_files.is_empty() {
        eprintln!("No .asm files found in {}", input_path);
        process::exit(1);
    }

    // Read and combine all files
    let mut combined_source = String::new();
    for file_path in &asm_files {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                combined_source.push('\n');
                combined_source.push_str(&content);
            }
            Err(e) => {
                eprintln!("Error reading {}: {}", file_path, e);
                process::exit(1);
            }
        }
    }

    // Assemble
    match Assembler::assemble(combined_source) {
        Ok(result) => {
            // Write output
            if let Err(e) = write_output(output_path, &result) {
                eprintln!("Error writing output: {}", e);
                process::exit(1);
            }
            println!("✓ Assembly complete: {} instructions → {} bytes", 
                     result.linked_ir.len(), 
                     result.linked_ir.len() * 2);
            println!("✓ Output written to {}", output_path);
        }
        Err(e) => {
            eprintln!("Assembly error: {}", e);
            process::exit(1);
        }
    }
}

fn collect_asm_files(path: &str) -> Result<Vec<String>, String> {
    let path_obj = Path::new(path);

    if path_obj.is_file() {
        if path.ends_with(".asm") {
            Ok(vec![path.to_string()])
        } else {
            Err(format!("File {} is not an .asm file", path))
        }
    } else if path_obj.is_dir() {
        let mut files = Vec::new();
        match fs::read_dir(path_obj) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_path = entry.path();
                        if file_path.extension().and_then(|s| s.to_str()) == Some("asm") {
                            if let Some(path_str) = file_path.to_str() {
                                files.push(path_str.to_string());
                            }
                        }
                    }
                }
                files.sort(); // Consistent ordering
                Ok(files)
            }
            Err(e) => Err(format!("Cannot read directory {}: {}", path, e)),
        }
    } else {
        Err(format!("{} is not a file or directory", path))
    }
}

fn write_output(path: &str, result: &assembler::AssemblyResult) -> Result<(), String> {
    // Format: address, machine code (hex), mnemonic
    let mut output = String::new();
    output.push_str("; Machine Code Output\n");
    output.push_str("; Format: address | hex code | mnemonic\n\n");

    for (i, (addr, code)) in result.machine_code.iter().enumerate() {
        let ir_info = result.linked_ir.get(i)
            .map(|(_, m, _)| m.as_str())
            .unwrap_or("???");
        output.push_str(&format!("0x{:04X} | 0x{:04X} | {}\n", addr, code, ir_info));
    }

    fs::write(path, output)
        .map_err(|e| format!("Failed to write {}: {}", path, e))
}

#[allow(dead_code)]
fn main_tests() {
    // ============================================
    // TEST 1: All A-type instructions
    // ============================================
    let test1 = r#"
        ; Test arithmetic operations
        add r0, r1
        sub r2, r3
        and r4, r5
        or r6, r7
        xor r8, r9
        shl r10, r11
        cmp r12, r13
    "#.to_string();

    // ============================================
    // TEST 2: I-type with edge case immediates
    // ============================================
    let test2 = r#"
        ; Test immediates at boundaries
        ldi r0, 0
        ldi r1, 255
        ldi r2, 128
        addi r3, 1
        subi r4, 200
    "#.to_string();

    // ============================================
    // TEST 3: Memory and stack operations
    // ============================================
    let test3 = r#"
        ; Test memory ops with positive offsets (0 to 7)
        ; SPR codes are stored separately as -8, -7, -6
        ld r0, [r1 + 0]
        st r2, [r3 + 5]
        ld r4, [r5 + 7]
        push r4:r5
        push r6
        pop r7:r8
        pop r9
        peek r10, 255
        poke r11, 0
    "#.to_string();

    // ============================================
    // TEST 4: All branch conditions
    // ============================================
    let test4 = r#"
        ; Test all branch conditions
        beq 1
        bne 1
        bcs 1
        bcc 1
        bmi 1
        bpl 1
        bov 1
        br 1
    "#.to_string();

    // ============================================
    // TEST 5: S-type variants
    // ============================================
    let test5 = r#"
        ; Test stack operations with immediates and registers
        subsp 16
        addsp 8
        subsp r0
        addsp r1
    "#.to_string();

    // ============================================
    // TEST 6: Edge case - max immediates
    // ============================================
    let test6 = r#"
        ; Test boundary conditions
        ldi r0, 255
        ldi r1, 0
        addi r2, 255
        ld r3, [r4 + 7]
        st r5, [r6 + 0]
        peek r7, 255
    "#.to_string();

    // ============================================
    // TEST 7: Negative branch offsets
    // ============================================
    let test7 = r#"
        ; Branch target will create backward reference
        beq 2
        add r0, r1
        sub r2, r3
    "#.to_string();

    // ============================================
    // TEST 8: Empty/minimal program
    // ============================================
    let test8 = r#"
        ldi r0, 1
    "#.to_string();

    let programs = vec![
        ("test1_atype.asm", test1),
        ("test2_itype.asm", test2),
        ("test3_memory.asm", test3),
        ("test4_branches.asm", test4),
        ("test5_stack.asm", test5),
        ("test6_edge_cases.asm", test6),
        ("test7_negative.asm", test7),
        ("test8_empty.asm", test8),
    ];

    // ============================================
    // ASSEMBLY PIPELINE: All phases in one call
    // ============================================
    let combined_source = programs
        .iter()
        .map(|(_, src)| src.clone())
        .collect::<Vec<_>>()
        .join("\n");

    match Assembler::assemble(combined_source) {
        Ok(result) => {
            println!("╔════════════════════════════════════════╗");
            println!("║ ASSEMBLY COMPLETE                      ║");
            println!("╚════════════════════════════════════════╝\n");

            println!("=== Final Linked IR (with byte addresses) ===");
            println!("Total instructions: {}", result.linked_ir.len());
            println!("Total bytes: {}\n", result.linked_ir.len() * 2);
            
            for (addr, mnemonic, instruction) in &result.linked_ir {
                println!("  0x{:04X} {:8} -> {:?}", addr, mnemonic, instruction);
            }
            println!();

            println!("=== Machine Code (Encoded) ===");
            println!("Total words: {}\n", result.machine_code.len());
            for (addr, encoded) in &result.machine_code {
                println!("  0x{:04X} -> 0x{:04X}", addr, encoded);
            }
            println!();

            println!("✓ Multi-file assembly complete!");
            println!("✓ All global symbols resolved!");
            println!("✓ Code successfully encoded!");
        }
        Err(e) => {
            eprintln!("Assembly error: {}", e);
            std::process::exit(1);
        }
    }
}
