use serde::{de, ser};
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Serialize error: {0}")]
  Ser(String),
  #[error("Deserialize error: {0}")]
  De(String),
  #[error("This type is not supported in RESP")]
  UnsupportedType,
  #[error("I/O error")]
  IO(#[from] std::io::Error),
  #[error("Int conversion error")]
  IntConversion(#[from] std::num::TryFromIntError),
  #[error("Int parsing error")]
  IntParsing(#[from] std::num::ParseIntError),
  #[error("Wrong type for command")]
  InvalidTypeForCommand,
  #[error("This type cannot be used for reply")]
  InvalidTypeForReply,
  #[error("Unknown size is not supported")]
  UnknownSize,
  #[error("Invalid size for command array")]
  InvalidSizeForCommandArray,
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
