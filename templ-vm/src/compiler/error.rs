use super::opcode::InvalidOpCodeErr;
use std::fmt;
use std::io;
#[derive(Debug)]
pub enum CompileError {
    InvalidOpCode,
    DuplicateVariable,
    Io(io::Error),
    Fmt(fmt::Error),
}

impl From<InvalidOpCodeErr> for CompileError {
    fn from(error: InvalidOpCodeErr) -> Self {
        Self::InvalidOpCode
    }
}

impl From<io::Error> for CompileError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<fmt::Error> for CompileError {
    fn from(error: fmt::Error) -> Self {
        Self::Fmt(error)
    }
}
