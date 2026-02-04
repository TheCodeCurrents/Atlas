pub mod args;

use args::Arguments;
use clap::Parser;

use crate::args::Command;

fn main() {
    let args = Arguments::parse();

    if let Err(e) = match args.command {
        Command::Asm { input, output } => {
            atlas_assembler::assemble(&input, &output)
        },
        _ => Ok(()),
    } {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}