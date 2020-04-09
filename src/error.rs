use serde::{de, ser};
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Error {
  #[error("Serialize error: {0}")]
  Ser(String),
  #[error("Deserialize error: {0}")]
  De(String),
  #[error("This type is not supported in RESP")]
  UnsupportedType,
  #[error("I/O error")]
  IOError(#[from] std::io::Error),
  #[error("Int conversion error")]
  IntConversionError(#[from] std::num::TryFromIntError),
}

impl ser::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Error::Ser(msg.to_string())
  }
}

impl de::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Error::De(msg.to_string())
  }
}
