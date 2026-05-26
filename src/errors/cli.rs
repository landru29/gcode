use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("error: {0}")]
    GenericError(String), 
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::GenericError(_) => 2,
        }
    }
}