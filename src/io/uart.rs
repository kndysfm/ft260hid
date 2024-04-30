use crate::{device::Device, Ft260Result};

pub struct Uart<'a> {
  device: &'a Device,
}

impl<'a> Uart<'a> {
  pub fn new(device: &'a Device) -> Self {
    Self { device }
  }
}