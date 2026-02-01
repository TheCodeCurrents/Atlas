
pub enum Command {
    ASM {
        input: String,
        output: String
    },
    TEST,
}

impl Command {
    pub fn from_args(args: &[String]) -> Result<Command, String> {
        if args.len() < 2 {
            return Err(String::from("Expected subcommand."))
        }

        let subcommand = &args[1];

        match subcommand.as_str() {
            "asm" => {
                // atlas assm -i <input> -o <output>
                let mut input = String::new();
                let mut output = String::new();
                let mut i = 2;
                while i < args.len() {
                    match args[i].as_str() {
                        "-i" => {
                            i += 1;
                            if i < args.len() {
                                input = args[i].clone();
                            } else {
                                return Err(String::from("Expected input file after -i"));
                            }
                        }
                        "-o" => {
                            i += 1;
                            if i < args.len() {
                                output = args[i].clone();
                            } else {
                                return Err(String::from("Expected output file after -o"));
                            }
                        }
                        _ => {
                            return Err(format!("Unknown argument: {}", args[i]));
                        }
                    }
                    i += 1;
                }
                Ok(Command::ASM { input, output })
            },
            "test" => Ok(Command::TEST),
            _ => Err(format!("Unknown subcommand: {}", subcommand))
        }
    }
}