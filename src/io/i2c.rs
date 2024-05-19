use std::time::Duration;

use crate::hid::consts::*;
use crate::hid::reports;
use crate::io::gpio;
use crate::{device::Device, Ft260Error, Ft260Result};

/// Interface type to use I2C function of the FT2260 device.
#[derive(Debug)]
pub struct I2c<'a> {
    device: &'a Device,
    inited: bool,
}

/// Flags to indicate I2C bus conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    /// Normal condition
    None,
    /// START condition
    Start,
    /// Repeated START condition
    ReStart,
    /// STOP condition
    Stop,
    /// START and STOP
    StartAndStop,
    /// Repeated START and STOP
    ReStartAndStop,
}

/// Default I2C clock speed value
pub const KBPS_DEFAULT: u16 = 100;

/// Default timeout duration
pub const DURATION_WAIT_DEFAULT: Duration = Duration::from_millis(5000);

impl<'a> I2c<'a> {
    /// create a new `I2c` instance
    pub(crate) fn new(device: &'a Device) -> Self {
        Self {
            device,
            inited: false,
        }
    }

    /// Initialize I2C function with clock speed
    pub fn init(&mut self, kbps: u16) -> Ft260Result<()> {
        let gpio = self.device.gpio();
        if let Err(e) = gpio.disable_pin(gpio::Group::Gpio_0_1) {
            dbg!(&e);
            return Err(e);
        }
        match reports::i2c::init(&self.device, kbps) {
            Ok(()) => {
                self.inited = true;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn flag_to_cond(f: Flag) -> I2cCondition {
        match f {
            Flag::None => I2cCondition::None,
            Flag::Start => I2cCondition::Start,
            Flag::ReStart => I2cCondition::ReStart,
            Flag::Stop => I2cCondition::Stop,
            Flag::StartAndStop => I2cCondition::StartAndStop,
            Flag::ReStartAndStop => I2cCondition::ReStartAndStop,
        }
    }

    /// Read I2C data
    pub fn read(
        &self,
        addr: u8,
        flag: Flag,
        buf: &mut [u8],
        len: usize,
        duration_wait: Duration,
    ) -> Ft260Result<usize> {
        assert!(self.inited);
        reports::i2c::read(
            self.device,
            addr,
            Self::flag_to_cond(flag),
            buf,
            len,
            duration_wait,
        )
    }

    /// Write I2C data
    pub fn write(&self, addr: u8, flag: Flag, buf: &[u8], len: usize) -> Ft260Result<usize> {
        assert!(self.inited);
        reports::i2c::write(self.device, addr, Self::flag_to_cond(flag), buf, len)
    }

    /// Write and read I2C data
    pub fn write_read(
        &self,
        addr: u8,
        buf_write: &[u8],
        len_write: usize,
        buf_read: &mut [u8],
        len_read: usize,
        duration_wait: Duration,
    ) -> Ft260Result<()> {
        assert!(self.inited);
        match reports::i2c::get_status(self.device) {
            Ok(s) => {
                if s != I2cBusStatus::ControllerIdle {
                    return Err(Ft260Error::I2cError {
                        message: "Bus was not in idle".to_string(),
                    });
                }
            }
            Err(e) => {
                return Err(e);
            }
        };
        match self.write(addr, Flag::Start, buf_write, len_write) {
            Ok(bytes_written) => {
                if bytes_written != len_write {
                    return Err(Ft260Error::I2cError {
                        message: "Failed to write all data".to_string(),
                    });
                }
            }
            Err(e) => {
                return Err(e);
            }
        };
        match reports::i2c::get_status(self.device) {
            Ok(s) => {
                if s.intersects(I2cBusStatus::AddressNack) || s.intersects(I2cBusStatus::DataNack) {
                    return Err(Ft260Error::I2cError {
                        message: "Device returned NACK".to_string(),
                    });
                }
            }
            Err(e) => {
                return Err(e);
            }
        };
        match self.read(
            addr,
            Flag::ReStartAndStop,
            buf_read,
            len_read,
            duration_wait,
        ) {
            Ok(bytes_read) => {
                if bytes_read != len_read {
                    return Err(Ft260Error::I2cError {
                        message: "Failed to read all data".to_string(),
                    });
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
        Ok(())
    }

    /// If I2C bus is idling or busy
    pub fn is_idle(&self) -> Option<bool> {
        match reports::i2c::get_status(self.device) {
            Ok(s) => Some(s == I2cBusStatus::ControllerIdle),
            Err(e) => None,
        }
    }

    fn reset(&self) -> Ft260Result<()> {
        reports::i2c::reset(self.device)
    }
}

impl<'a> Drop for I2c<'a> {
    fn drop(&mut self) {
        if self.inited {
            self.reset()
                .expect("I2C master should be reset successfully")
        }
    }
}
