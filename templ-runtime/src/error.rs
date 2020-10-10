use std::fmt;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("parse")]
    Format(#[from] fmt::Error),
}
