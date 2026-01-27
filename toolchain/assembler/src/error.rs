use std::fmt;

/// Which phase of assembly encountered the error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssemblyPhase {
    Lex,
    Parse,
    Link,
    Encode,
}

impl fmt::Display for AssemblyPhase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lex => write!(f, "Lexer"),
            Self::Parse => write!(f, "Parser"),
            Self::Link => write!(f, "Linker"),
            Self::Encode => write!(f, "Encoder"),
        }
    }
}

/// Structured error type for the assembler
#[derive(Debug, Clone)]
pub struct AssemblerError {
    pub phase: AssemblyPhase,
    pub message: String,
}

impl AssemblerError {
    pub fn lex(message: impl Into<String>) -> Self {
        Self {
            phase: AssemblyPhase::Lex,
            message: message.into(),
        }
    }

    pub fn parse(message: impl Into<String>) -> Self {
        Self {
            phase: AssemblyPhase::Parse,
            message: message.into(),
        }
    }

    pub fn link(message: impl Into<String>) -> Self {
        Self {
            phase: AssemblyPhase::Link,
            message: message.into(),
        }
    }

    pub fn encode(message: impl Into<String>) -> Self {
        Self {
            phase: AssemblyPhase::Encode,
            message: message.into(),
        }
    }
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.phase, self.message)
    }
}

impl std::error::Error for AssemblerError {}
