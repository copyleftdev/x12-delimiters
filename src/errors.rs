use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DelimiterError {
    InvalidIsaLength,
}

impl fmt::Display for DelimiterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DelimiterError::InvalidIsaLength => {
                write!(f, "ISA segment must be at least 106 bytes long to extract delimiters")
            }
        }
    }
}

impl std::error::Error for DelimiterError {}