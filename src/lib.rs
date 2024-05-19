/// module to control FT260 HID device
pub mod device;
mod error;
mod hid;
/// interface modules to use GPIO, I2C and UART features
pub mod io;

/// common Error type in this crate
pub use error::Ft260Error;

/// common Result type in this crate
pub type Ft260Result<T> = Result<T, Ft260Error>;
