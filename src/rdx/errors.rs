// src/error.rs


//! # Quant Errors (`utils`)
//! ...

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum QuantError {
    /// Raised when a node is missing or can't be found
    NodeNotFound(String),

    /// Raised when a symbol or tag is invalid or unrecognized
    InvalidSymbol(String),

    /// Raised when the structure of a quant is malformed
    MalformedQuant(String),

    /// Generic logic violation or constraint error
    LogicViolation(&'static str),

    /// Underlying I/O or system error
    Io(std::io::Error),
}

impl fmt::Display for QuantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantError::NodeNotFound(id) => write!(f, "Quant node not found: {}", id),
            QuantError::InvalidSymbol(sym) => write!(f, "Invalid symbol: {}", sym),
            QuantError::MalformedQuant(desc) => write!(f, "Malformed quant: {}", desc),
            QuantError::LogicViolation(msg) => write!(f, "Logic violation: {}", msg),
            QuantError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for QuantError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            QuantError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for QuantError {
    fn from(err: std::io::Error) -> Self {
        QuantError::Io(err)
    }
}
