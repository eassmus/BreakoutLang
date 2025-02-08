use crate::scanner::Type;
use std::error::Error;

#[derive(Debug)]
pub struct ProgramError {
    pub message: String,
}

impl std::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for ProgramError {}

#[derive(Debug)]
pub struct TypeError {
    pub message: String,
    pub expected: Type,
    pub found: Type,
}
impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Type Error: {}, expected: {}, found: {}",
            self.message, self.expected, self.found
        )
    }
}
impl Error for TypeError {}
