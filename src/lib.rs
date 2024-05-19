// mod consts;
pub mod device;
mod error;
mod hid;
pub mod io;

pub use error::Ft260Error;

pub type Ft260Result<T> = Result<T, Ft260Error>;
