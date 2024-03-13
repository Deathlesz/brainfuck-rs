#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("source file error: {0}")]
    SourceFileError(&'static str),
    #[error("invalid instruction: {}", *.0 as char)]
    InvalidInstruction(u8),
    #[error(transparent)]
    ExecutionError(#[from] ExecutionError),
}

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("unexpected bracket at char {0}")]
    UnexpectedBracket(usize),
    #[error("failed to read/write: {0}")]
    IoError(#[from] std::io::Error),
}
