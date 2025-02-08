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
