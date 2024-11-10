// Q: it feels weird chaining these errors?
use crate::procedure::Error as ProcError;
use derive_more::{derive::Display, From};
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Display)]
// #[derive(Debug, thiserror::Error, Display)]
pub enum Error {
    // #[error("Something went wrong")]
    GenericError,
    // #[error("File reading error")]
    FileError(#[from] std::io::Error),
    // #[error("Failed to parse calendar")]
    #[display("Failure parsing: {_0}")]
    ParsingError(String),
    // #[error("Failure in procedure")]
    ProcedureError(#[from] ProcError),
}

// region:    --- Error Boilerplate

// impl core::fmt::Display for Error {
//     fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
//         write!(fmt, "{self:?}")
//     }
// }

// impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
