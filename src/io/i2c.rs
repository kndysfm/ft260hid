use crate::{device::Device, Ft260Result};

pub struct I2c<'a> {
  device: &'a Device,
}

impl<'a> I2c<'a> {
  pub fn new(device: &'a Device) -> Self {
    Self { device }
  }
}