use crate::hid::consts::*;
use crate::hid::reports;
use crate::io::gpio;
use crate::{device::Device, Ft260Result};
use log::{debug};
use std::time::Duration;

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
      fn to_hid_const(self) -> $tgt {
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

hid_const_compatible! {
  UartEnableMode,
  /// UART Flow Control Mode
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum Mode {
    /// > "OFF, and switch UART pins to GPIO"
    Off,
    /// > "RTS_CTS mode (GPIOB =>RTSN, GPIOE =>CTSN)"
    RtsCts,
    /// > "DTR_DSR mode (GPIOF =>DTRN, GPIOH => DSRN)"
    DtrDsr,
    /// > "XON_XOFF (software flow control)"
    XonXoff,
    /// > "No flow control mode"
    NoFlowControl,
  }
}

hid_const_compatible! {
  UartDcdRiEnableMode,
  /// UART DCD & RI mode
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum DcdRi {
    Disabled,
    Enabled,
  }
}

hid_const_compatible! {
  UartParity,
  /// UART Parity mode
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum Parity {
    /// No parity
    None,
    /// Odd parity
    Odd,
    /// Even parity
    Even,
    /// Parity bit is always high
    High,
    /// Parity bit is always low
    Low,
  }
}

hid_const_compatible! {
  UartStopBit,
  /// Stop bit
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum StopBit {
      /// one stop bit
      One,
      /// two stop bits
      Two,
  }
}

hid_const_compatible! {
  UartBreaking,
  /// > "When active the TXD line goes into ‘spacing’ state which causes a break in the receiving UART."
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum Breaking {
      /// No break
      NoBreak,
      /// Break
      Break,
  }
}

hid_const_compatible! {
  UartDataBits,
  /// The number of UART data bits
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum DataBits {
      /// 7 data bits
      Seven,
      /// 8 data bits
      Eight,
  }
}

/// Parameters set to configure UART function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Flow control mode
    pub mode: Mode,
    /// UART Baud rate
    pub baud: u32,
    /// Data bits
    pub data_bits: DataBits,
    /// Stop bit
    pub stop_bit: StopBit,
    /// Parity mode
    pub parity: Parity,
    /// Breaking mode
    pub breaking: Breaking,
}

/// Default UART Baud rate value
pub const BAUD_DEFAULT: u32 = 115200;
/// Default timeout duration
pub const DURATION_WAIT_DEFAULT: Duration = Duration::from_millis(5000);

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: Mode::NoFlowControl,
            baud: BAUD_DEFAULT,
            data_bits: DataBits::Eight,
            stop_bit: StopBit::One,
            parity: Parity::None,
            breaking: Breaking::NoBreak,
        }
    }
}

impl Config {
    fn from_hid(cfg: &reports::uart::Config) -> Self {
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

/// Interface type to use UART function of the FT2260 device.
#[derive(Debug)]
pub struct Uart<'a> {
    device: &'a Device,
    inited: bool,
}

impl<'a> Uart<'a> {
    pub(crate) fn new(device: &'a Device) -> Self {
        Self {
            device,
            inited: false,
        }
    }

    /// Initialize UART function
    pub fn init(&mut self) -> Ft260Result<()> {
        let device = self.device;
        let gpio = device.gpio();
        if let Err(e) = gpio.disable_pin(gpio::Group::Gpio_B_C_D_E_F_H) {
            debug!("{:?}", &e);
            debug!("Failed to disable Gpio B,C,D,E,F,H");
            return Err(e);
        }
        if let Err(e) = reports::uart::init(device) {
            debug!("{:?}", &e);
            debug!("Failed to initialize UART function");
            return Err(e);
        }
        self.inited = true;
        Ok(())
    }

    /// Configure UART function with parameters set
    pub fn set_config(&self, cfg: &Config) -> Ft260Result<()> {
        let device = self.device;

        reports::uart::set_flow_control(device, cfg.mode.to_hid_const())?;
        reports::uart::set_baud_rate(device, cfg.baud)?;
        reports::uart::set_data_bits(device, cfg.data_bits.to_hid_const())?;
        reports::uart::set_stop_bit(device, cfg.stop_bit.to_hid_const())?;
        reports::uart::set_parity(device, cfg.parity.to_hid_const())?;
        reports::uart::set_breaking(device, cfg.breaking.to_hid_const())?;

        Ok(())
    }

    /// Get current UART configuration parameters
    pub fn get_config(&self) -> Ft260Result<Config> {
        let device = self.device;
        let cfg = reports::uart::get_config(device);
        if cfg.is_err() {
            debug!("Failed to get UART config");
            return Err(cfg.unwrap_err());
        }
        let cfg = cfg.unwrap();
        Ok(Config::from_hid(&cfg))
    }

    /// Get data amount in RX data FIFO
    pub fn size_to_read(&self) -> usize {
        reports::uart::get_queue_status(self.device)
    }

    /// Read RX data from FIFO
    pub fn read(&self, buf: &mut [u8], len: usize, duration_wait: Duration) -> Ft260Result<usize> {
        reports::uart::read(self.device, buf, len, duration_wait)
    }

    /// Write TX data
    pub fn write(&self, buf: &[u8], len: usize) -> Ft260Result<usize> {
        reports::uart::write(self.device, buf, len)
    }

    /// Reset UART function  
    /// > "The request will reset the FT260 UART controller."
    fn reset(&self) -> Ft260Result<()> {
        reports::uart::reset(self.device)
    }
}

impl<'a> Drop for Uart<'a> {
    fn drop(&mut self) {
        if self.inited {
            self.reset().expect("UART should be reset successfully")
        }
    }
}
