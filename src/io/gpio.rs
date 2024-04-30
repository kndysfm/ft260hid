use crate::{device::Device, Ft260Result};
pub struct Gpio<'a> {
  device: &'a Device,
}

#[repr(u8)]
pub enum Group {
  Gpio_0_1,
  Gpio_2,
  Gpio_3,
  Gpio_4_5,
  Gpio_A,
  Gpio_B_C_D_E_F_H,
  Gpio_G,
}

impl<'a> Gpio<'a> {
  pub fn new(device: &'a Device) -> Self {
    Self { device }
  }
/*
  pub fn enable_pin(g:Group) -> Ft260Result<()> {
    match g {
      Gpio_0_1 => {
        ()
      },
      Gpio_2 => {
        ()
      },
      Gpio_3 => {
        ()
      },
      Gpio_4_5 => {
        ()
      },
      Gpio_A => {
        ()
      },
      Gpio_B_C_D_E_F_H => {
        ()
      },
      Gpio_G => {
        ()
      },
      _ => (),
    }
  }
*/
}