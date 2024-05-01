use std::time::Duration;

use crate::{device::Device, Ft260Error, Ft260Result};
use crate::hid::reports::{self, ft260_i2c_master_get_status, ft260_i2c_master_reset};
use crate::hid::consts::*;
use crate::io::gpio;

#[derive(Debug)]
pub struct I2c<'a> {
  device: &'a Device,
  inited: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
  None,
  Start,
  ReStart,
  Stop,
  StartAndStop,
  ReStartAndStop,
}

pub const KBPS_DEFAULT: u16 = 100;

pub const DURATION_WAIT_DEFAULT: Duration = Duration::from_millis(5000);

impl<'a> I2c<'a> {
  pub fn new(device: &'a Device) -> Self {
    Self { device, inited: false }
  }

  pub fn init(&mut self, kbps: u16) -> Ft260Result<()> {
    let gpio = self.device.gpio();
    if let Err(e) = gpio.disable_pin(gpio::Group::Gpio_0_1) {
      dbg!(&e);
      return Err(e);
    }
    match reports::ft260_i2c_master_init(&self.device, kbps) {
      Ok(()) => {
        self.inited = true;
        Ok(())
      },
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

  pub fn read(&self, addr: u8, flag: Flag, buf: &mut [u8], len: usize, duration_wait: Duration) -> Ft260Result<usize> {
    assert!(self.inited);
    reports::ft260_i2c_master_read(self.device, addr, Self::flag_to_cond(flag), buf, len, duration_wait)
  }

  pub fn write(&self, addr: u8, flag: Flag, buf: &[u8], len: usize) -> Ft260Result<usize> {
    assert!(self.inited);
    reports::ft260_i2c_master_write(self.device, addr, Self::flag_to_cond(flag), buf, len)
  }

  pub fn write_read(&self, addr: u8, buf_write: &[u8], len_write: usize, buf_read: &mut [u8], len_read: usize, duration_wait: Duration) -> Ft260Result<()> {
    assert!(self.inited);
    match ft260_i2c_master_get_status(self.device) {
      Ok(s) => {
        if s != I2cBusStatus::ControllerIdle {
          return Err(Ft260Error::I2cError { message: "Bus was not in idle".to_string() });
        }
      },
      Err(e) => { return Err(e); },
    };
    match self.write(addr, Flag::Start, buf_write, len_write) {
      Ok(bytes_written) => {
        if bytes_written != len_write {
          return Err(Ft260Error::I2cError { message: "Failed to write all data".to_string() });
        }
      },
      Err(e) => { return Err(e); },
    };
    match ft260_i2c_master_get_status(self.device) {
      Ok(s) => {
        if s.intersects(I2cBusStatus::AddressNack) || s.intersects(I2cBusStatus::DataNack) {
          return Err(Ft260Error::I2cError { message: "Device returned NACK".to_string() });
        }
      },
      Err(e) => { return Err(e); },
    };
    match self.read(addr, Flag::ReStartAndStop, buf_read, len_read, duration_wait) {
      Ok(bytes_read) => {
        if bytes_read != len_read {
          return Err(Ft260Error::I2cError { message: "Failed to read all data".to_string() });
        }
      },
      Err(e) => { return Err(e); },
    }
    Ok(())
  }

  pub fn is_idle(&self) -> bool {
    match ft260_i2c_master_get_status(self.device) {
      Ok(s) => s == I2cBusStatus::ControllerIdle,
      Err(e) => false,
    }
  }

  fn reset(&self) -> Ft260Result<()> {
    ft260_i2c_master_reset(self.device)
  }
}

impl<'a> Drop for I2c<'a> {
    fn drop(&mut self) {
      if self.inited {
        self.reset().expect("I2C master should be reset successfully")
      }
    }
}