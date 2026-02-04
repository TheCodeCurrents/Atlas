use clap:: {
    Parser,
    Subcommand,
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Asm {
        /// input assembly file
        #[arg(value_name = "INPUT")]
        input: String,
        
        /// output binary file
        #[arg(value_name = "OUTPUT")]
        output: String,
    },
    Ld,
}