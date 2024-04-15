use std::error::Error as StdError;

pub use std::concat as c;
pub use std::format as f;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, NoPhpError>;

#[derive(Debug, Error)]
pub enum NoPhpError {
    #[error("[ERR] Malformed AST")]
    MalformedAST,
    #[error("[ERR] Cannot parse a value into a NoPHP Type")]
    ValueParseError,
    #[error("[ERR:PYTHON] {0}")]
    PyErr(#[from] pyo3::PyErr),

    #[error("[ERR:OTHER] {0}")]
    StdErr(#[from] Box<dyn StdError>),

    #[error("[ERR:OTHER] {0}")]
    Other(&'static str),
}
