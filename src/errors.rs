use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Error: Index out of bounds.")]
    OutOfBounds,

    #[error("Error: Cannot parse string into type")]
    ParseToTypeFailed,
}
