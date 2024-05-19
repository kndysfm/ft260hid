use std::time::Duration;

use crate::device;
use crate::hid::consts::*;
use crate::hid::reports::*;
use crate::io::gpio;
use crate::{device::Device, Ft260Result};

macro_rules! hid_const_compatible {
  (
    $tgt:ident,
    $(#[$meta:meta])*
    $vis:vis enum $name:ident {
      $($(#[$vmeta:meta])*
      $vname:ident $(= $val:expr)?,)*
    }
  ) => {
    $(#[$meta])*
    $vis enum $name {
      $($(#[$vmeta])* $vname $(= $val)?,)*
    }

    impl $name {
      fn to_hid_const(&self) -> $tgt {
        match self {
          $($name::$vname => $tgt::$vname,)*
        }
      }
      fn from_hid_const(v:&$tgt) -> Self {
        match v {
          $($tgt::$vname => $name::$vname,)*
        }
      }
    }
}
}

/// UART Flow Control Mode
hid_const_compatible! {
  UartEnableMode,
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum Mode {
    Off,
    RtsCts,
    DtrDsr,
    XonXoff,
    NoFlowControl,
  }
}

hid_const_compatible! {
  UartDcdRiEnableMode,
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum DcdRi {
    Disabled,
    Enabled,
  }
}

hid_const_compatible! {
  UartParity,
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum Parity {
    None,
    Odd,
    Even,
    High, // parity bit is always high
    Low, // parity bit is always low
  }
}

hid_const_compatible! {
  UartStopBit,
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum StopBit {
      One,
      Two,
  }
}

hid_const_compatible! {
  UartBreaking,
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum Breaking {
      NoBreak,
      Break,
  }
}

hid_const_compatible! {
  UartDataBits,
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum DataBits {
      Seven,
      Eight,
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub mode: Mode,
    pub baud: u32,
    pub data_bits: DataBits,
    pub stop_bit: StopBit,
    pub parity: Parity,
    pub breaking: Breaking,
}

pub const BAUD_DEFAULT: u32 = 115200;

pub const DURATION_WAIT_DEFAULT: Duration = Duration::from_millis(5000);

impl Config {
    pub fn default() -> Self {
        Self {
            mode: Mode::NoFlowControl,
            baud: BAUD_DEFAULT,
            data_bits: DataBits::Eight,
            stop_bit: StopBit::One,
            parity: Parity::None,
            breaking: Breaking::NoBreak,
        }
    }
    fn from_hid(cfg: &UartConfig) -> Self {
        Self {
            mode: Mode::from_hid_const(&cfg.mode),
            baud: cfg.baud_rate,
            data_bits: DataBits::from_hid_const(&cfg.data_bits),
            stop_bit: StopBit::from_hid_const(&cfg.stop_bit),
            parity: Parity::from_hid_const(&cfg.parity),
            breaking: Breaking::from_hid_const(&cfg.breaking),
        }
    }
}

#[derive(Debug)]
pub struct Uart<'a> {
    device: &'a Device,
    inited: bool,
}

macro_rules! dbg_write {
    ($msg:expr) => {
        if cfg!(debug_assertions) {
            println!("[DEBUG] {}:{} '{}'", file!(), line!(), $msg);
        }
    };
}

impl<'a> Uart<'a> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            inited: false,
        }
    }

    pub fn init(&mut self) -> Ft260Result<()> {
        let device = self.device;
        let gpio = device.gpio();
        if let Err(e) = gpio.disable_pin(gpio::Group::Gpio_B_C_D_E_F_H) {
            dbg!(&e);
            return Err(e);
        }
        if let Err(e) = ft260_uart_init(device) {
            dbg!(&e);
            return Err(e);
        }
        self.inited = true;
        Ok(())
    }

    pub fn set_config(&self, cfg: &Config) -> Ft260Result<()> {
        let device = self.device;

        if let Err(e) = ft260_uart_set_flow_control(device, cfg.mode.to_hid_const()) {
            dbg!(&e);
            return Err(e);
        }
        if let Err(e) = ft260_uart_set_baud_rate(device, cfg.baud) {
            dbg!(&e);
            return Err(e);
        }
        if let Err(e) = ft260_uart_set_data_bits(device, cfg.data_bits.to_hid_const()) {
            dbg!(&e);
            return Err(e);
        }
        if let Err(e) = ft260_uart_set_stop_bit(device, cfg.stop_bit.to_hid_const()) {
            dbg!(&e);
            return Err(e);
        }
        if let Err(e) = ft260_uart_set_parity(device, cfg.parity.to_hid_const()) {
            dbg!(&e);
            return Err(e);
        }
        if let Err(e) = ft260_uart_set_breaking(device, cfg.breaking.to_hid_const()) {
            dbg!(&e);
            return Err(e);
        }

        Ok(())
    }

    pub fn get_config(&self) -> Ft260Result<Config> {
        let device = self.device;
        let cfg = ft260_uart_get_config(device);
        if cfg.is_err() {
            return Err(cfg.unwrap_err());
        }
        let cfg = cfg.unwrap();
        Ok(Config::from_hid(&cfg))
    }

    pub fn get_amount_in_rx_fifo(&self) -> usize {
        ft260_uart_get_queue_status(self.device)
    }

    pub fn read(&self, buf: &mut [u8], len: usize, duration_wait: Duration) -> Ft260Result<usize> {
        ft260_uart_read(self.device, buf, len, duration_wait)
    }

    pub fn write(&self, buf: &[u8], len: usize) -> Ft260Result<usize> {
        ft260_uart_write(self.device, buf, len)
    }

    pub fn reset(&self) -> Ft260Result<()> {
        ft260_uart_reset(self.device)
    }
}

impl<'a> Drop for Uart<'a> {
    fn drop(&mut self) {
        if self.inited {
            self.reset().expect("UART should be reset successfully")
        }
    }
}
