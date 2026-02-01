use std::{env};

pub mod command;

use command::Command;
use atlas_assembler::Parser;

fn main() -> Result<(), String> {
    println!("Atlas CLI Tool");

    let args = env::args().collect::<Vec<String>>();

    if args.len() <= 1 {
        return Err(String::from("Expected subcommand."))
    }
    
    let subcommand = Command::from_args(&args)?;

    match subcommand {
        Command::ASM { input, output } => {
            println!("Assembling from {} to {}", input, output);
            
            // get string from input file
            let input_content = std::fs::read_to_string(&input)
                .map_err(|e| format!("Failed to read input file: {}", e))?;

            // Parse the input
            let parser = Parser::new(&input_content);
            
            println!("\n=== Parsed Instructions ===");
            let mut inst_count = 0;
            for (i, result) in parser.enumerate() {
                match result {
                    Ok(instruction) => {
                        println!("[{}] {:?}", i, instruction);
                        inst_count += 1;
                    }
                    Err(e) => {
                        eprintln!("Parse error at instruction {}: {}", i, e);
                        return Err(format!("Failed to parse: {}", e));
                    }
                }
            }
            println!("=== End Instructions ({} instructions) ===\n", inst_count);
        }
        _ => {
            return Err(String::from("Unknown command."));
        }
    }

    return Ok(())
}