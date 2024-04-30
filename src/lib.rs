// mod consts;
mod error;
mod hid;
mod io;
pub mod device;

pub use error::Ft260Error;

pub type Ft260Result<T> = Result<T, Ft260Error>;
