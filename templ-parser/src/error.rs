use peg_runtime::{error::ParseError, str::LineCol};

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("parse")]
    Parse(#[from] ParseError<LineCol>),
}
