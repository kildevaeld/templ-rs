use super::opcode::InvalidOpCodeErr;
use std::fmt;
use std::io;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum CompileError {
    #[error("invalid opcode")]
    InvalidOpCode(#[from] InvalidOpCodeErr),
    #[error("duplicate variable")]
    DuplicateVariable,
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("format error")]
    Fmt(#[from] fmt::Error),
}
