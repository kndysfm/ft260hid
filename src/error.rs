use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Ft260Error {
  HidError {
    message: String,
  },
  ByteError {
    value: u8,
    message: String,
  },
  I2cError {
    message: String,
  },
  UartError {
    message: String,
  },
  OtherError {
    error: std::io::Error,
  },
}

impl Display for Ft260Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::HidError { message } => {
        write!(f, "ft260 error: {}", message)
      },
      Self::ByteError { value, message } => {
        write!(f, "byte data error - `{:#x}`: {}", value, message)
      }
      Self::I2cError { message } => {
        write!(f, "ft260 I2C error: {}", message)
      },
      Self::UartError { message } => {
        write!(f, "ft260 UART error: {}", message)
      },
      Self::OtherError { error } => {
        write!(f, "{error}")
      },
    }
  }
}

impl Error for Ft260Error {}

impl From<std::io::Error> for Ft260Error {
  fn from(e: std::io::Error) -> Self {
      Self::OtherError { error: e }
  }
}
