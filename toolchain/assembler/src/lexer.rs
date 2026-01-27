
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpecialRegister {
    TR,
    SP,
    PC,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Mnemonic(String),

    Register(u8),
    RegisterPair(u8, u8),
    SpecialRegister(SpecialRegister),
    Immediate(i32),
    Label(String),
    Identifier(String),

    // Syntax
    Comma,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Plus,
    Minus,
    At,
    
    // Special
    Directive(String),
    Comment,
    Newline,
    EOF,
}

/// A simple lexer for the assembler
pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

// TODO: handle Errors and provide line and column number
impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenizes the input string into a vetor of
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while !self.is_eof() {
            self.skip_whitespace();

            if self.is_eof() {
                break;
            }

            // load current character as lower case
            let current_char = self.current_char().to_ascii_lowercase();
            
            // check for comments
            if self.check_comment() {
                continue;
            }
            
            // check for single character tokens
            if self.check_single_char(&mut tokens, current_char) {
                continue;
            }

            // check for numbers (immediates)
            if current_char.is_ascii_digit() || (current_char == '-' && self.peek_char().is_ascii_digit()) {
                if let Ok(num) = self.parse_number() {
                    tokens.push(Token::Immediate(num));
                    continue;
                }
            }

            // check for registers (r0, r1, etc.) and register pairs (r0:r1)
            if current_char == 'r' && self.peek_char().is_ascii_digit() {
                if let Ok((first_reg, is_pair)) = self.parse_register_or_pair() {
                    if is_pair {
                        // We need to parse the second register
                        if let Ok(second_reg) = self.parse_register() {
                            tokens.push(Token::RegisterPair(first_reg, second_reg));
                        }
                    } else {
                        tokens.push(Token::Register(first_reg));
                    }
                    continue;
                }
            }

            // check for directives (starting with .)
            if current_char == '.' {
                if let Ok(directive) = self.parse_directive() {
                    tokens.push(Token::Directive(directive));
                    continue;
                }
            }

            // check for identifiers, mnemonics, labels, and special registers
            if current_char.is_alphabetic() || current_char == '_' {
                let ident = self.parse_identifier();
                
                // Check if it's a special register name (TR, SP, PC)
                if let Some(special_reg) = self.parse_special_register(&ident) {
                    tokens.push(Token::SpecialRegister(special_reg));
                    continue;
                }
                
                // Check if it's a label (followed by colon)
                self.skip_whitespace();
                if !self.is_eof() && self.current_char() == ':' {
                    tokens.push(Token::Label(ident));
                    self.advance(); // consume the colon
                } else {
                    // Assume it's a mnemonic or identifier
                    tokens.push(Token::Mnemonic(ident));
                }
                continue;
            }

            // Unknown character, skip it
            return Err(format!("Unexpected character '{}' at line {}", current_char, self.line));
        }
        
        tokens.push(Token::EOF);
        return Ok(tokens);
    }

    fn check_single_char(&mut self, tokens: &mut Vec<Token>, character: char) -> bool {
        // Handle single-character tokens
            match character {
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                    true
                },
                '[' => {
                    tokens.push(Token::LeftBracket);
                    self.advance();
                    true
                },
                ']' => {
                    tokens.push(Token::RightBracket);
                    self.advance();
                    true
                },
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.advance();
                    true
                },
                ')' => {
                    tokens.push(Token::RightParen);
                    self.advance();
                    true
                },
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                    true
                },
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                    true
                },
                '@' => {
                    tokens.push(Token::At);
                    self.advance();
                    true
                },
                '\n' => {
                    tokens.push(Token::Newline);
                    self.advance();
                    self.line += 1;
                    true
                },
                _ => {
                    false
                }
            }
    }

    /// checks if position if current position is greater or equal to the input length
    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    /// advance the position by one
    fn advance(&mut self) {
        if !self.is_eof() {
            self.position += 1;
            self.column += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.current_char().is_whitespace() && self.current_char() != '\n' {
            self.advance();
        }
    } 

    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }

    fn peek_char(&self) -> char {
        self.input.chars().nth(self.position + 1).unwrap_or('\0')
    }

    fn check_comment(&mut self) -> bool {
        if self.current_char() == ';' {
            // Skip until end of line
            while !self.is_eof() && self.current_char() != '\n' {
                self.advance();
            }
            return true;
        }
        false
    }

    fn parse_number(&mut self) -> Result<i32, String> {
        let start = self.position;
        
        // Handle optional minus sign
        if self.current_char() == '-' {
            self.advance();
        }

        // Parse digits
        while !self.is_eof() && self.current_char().is_ascii_digit() {
            self.advance();
        }

        let num_str = self.input[start..self.position].to_string();
        num_str.parse::<i32>()
            .map_err(|_| format!("Invalid number: {}", num_str))
    }

    fn parse_register(&mut self) -> Result<u8, String> {
        if self.current_char() != 'r' {
            return Err("Expected 'r' for register".to_string());
        }
        self.advance();

        let start = self.position;
        while !self.is_eof() && self.current_char().is_ascii_digit() {
            self.advance();
        }

        let reg_str = self.input[start..self.position].to_string();
        reg_str.parse::<u8>()
            .map_err(|_| format!("Invalid register number: {}", reg_str))
    }

    fn parse_register_or_pair(&mut self) -> Result<(u8, bool), String> {
        let first_reg = self.parse_register()?;
        
        // Check if followed by colon and another register
        self.skip_whitespace();
        if !self.is_eof() && self.current_char() == ':' {
            self.advance(); // consume the colon
            self.skip_whitespace();
            if self.current_char() == 'r' && self.peek_char().is_ascii_digit() {
                return Ok((first_reg, true));
            } else {
                // Colon found but not followed by a register, error
                return Err("Expected register after ':'".to_string());
            }
        }
        
        Ok((first_reg, false))
    }

    fn parse_directive(&mut self) -> Result<String, String> {
        if self.current_char() != '.' {
            return Err("Expected '.' for directive".to_string());
        }
        self.advance();

        let start = self.position;
        while !self.is_eof() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }

        Ok(format!(".{}", &self.input[start..self.position]))
    }

    fn parse_identifier(&mut self) -> String {
        let start = self.position;
        while !self.is_eof() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        self.input[start..self.position].to_string()
    }

    fn parse_special_register(&self, ident: &str) -> Option<SpecialRegister> {
        match ident.to_uppercase().as_str() {
            "TR" => Some(SpecialRegister::TR),
            "SP" => Some(SpecialRegister::SP),
            "PC" => Some(SpecialRegister::PC),
            _ => None,
        }
    }
}