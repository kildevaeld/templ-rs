use super::compiler::CompileError;
use std::fmt;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("parse")]
    Compile(#[from] CompileError),
}
