use atlas_isa::{Instruction, RegisterIdentifier};

pub enum Token {
    Mnemonic(Instruction),
    Directive(Directive),
    Register(RegisterIdentifier),
    Immediate(u8),
    Label(String),

    Comma,
    Colon,
    Semicolon,
    AtSign,

    NewLine,
    EoF,   
}

pub struct Span {
    pub start: usize,
    pub end: usize

}
pub struct SpannedToken { pub token: Token, pub span: Span }


pub enum Directive {
    Import,     // import label from another file
    Export,     // export label for use in another file
}

impl Directive {
    pub fn from_str(s: &str) -> Option<Directive> {
        match s {
            "import" => Some(Directive::Import),
            "export" => Some(Directive::Export),
            _ => None,
        }
    }
}