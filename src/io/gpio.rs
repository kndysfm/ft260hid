use crate::device::Device;
use crate::hid::consts::*;
use crate::hid::reports;
use crate::Ft260Result;

/// Interface type to use GPIO function of the FT260 device.
#[derive(Debug)]
pub struct Gpio<'a> {
    device: &'a Device,
}

/// FT260 has 14 GPIO pins (Refer "3.3 Pin Description" in [datasheet](https://ftdichip.com/wp-content/uploads/2023/11/DS_FT260.pdf))
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pin {
    /// DIO5 (SCL / GPIO0)
    Gpio0,
    /// DIO6 (SDA / GPIO1)
    Gpio1,
    /// DIO7 (SUSPOUT_N / PWREN_N / TX_LED / GPIO2)
    Gpio2,
    /// DIO8 (INTRIN / WAKEUP / GPIO3)
    Gpio3,
    /// DIO10 (GPIO4 / DCD)
    Gpio4,
    /// DIO11 (GPIO5 / RI)
    Gpio5,
    /// DIO0 (TX_ACTIVE / TX_LED / GPIOA)
    GpioA,
    /// DIO1 (GPIOB / RTSN)
    GpioB,
    /// DIO3 (RXD / GPIOC)
    GpioC,
    /// DIO4 (TXD / GPIOD)
    GpioD,
    /// DIO2 (GPIOE / CTSN)
    GpioE,
    /// DIO9 (GPIOF / DTRN)
    GpioF,
    /// DIO12 (BCD_DET / RX_LED / PWREN_N / GPIOG)
    GpioG,
    /// DIO13 (GPIOH / DSRN)
    GpioH,
}

/// GPIO groups separated by functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    /// Pins for I2C
    Gpio_0_1,
    /// Pin for suspend out, etc.
    Gpio_2,
    /// Pin for interrupt in
    Gpio_3,
    /// Pins for UART DCD(Data Carrier Detection) / RI(Ring Indictor)
    Gpio_4_5,
    /// Pin for TX indication
    Gpio_A,
    /// Pins for UART (TXD/RXD/CTS/RTS/DTR/DSR)
    Gpio_B_C_D_E_F_H,
    /// Pin for
    Gpio_G,
}

/// Direction of GPIO pin
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    /// Input
    In,
    /// Output
    Out,
}

/// Value of GPIO pin's input or output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Val {
    // Low level
    Low,
    // High level
    High,
}

impl<'a> Gpio<'a> {
    pub(crate) fn new(device: &'a Device) -> Self {
        Self { device }
    }

    /// Enable GPIO function for specific pin group
    pub fn enable_pin(&self, group: Group) -> Ft260Result<()> {
        match group {
            Group::Gpio_0_1 => reports::gpio::set_i2c_pins(self.device, I2cEnableMode::Disabled),
            Group::Gpio_2 => {
                reports::gpio::select_gpio_2_function(self.device, Gpio2Function::Gpio)
            }
            Group::Gpio_3 => {
                reports::ft260_set_wakeup_interrupt(self.device, WakeupIntEnableMode::Disabled)
            }
            Group::Gpio_4_5 => {
                reports::gpio::set_dcd_ri_pins(self.device, UartDcdRiEnableMode::Disabled)
            }
            Group::Gpio_A => {
                reports::gpio::select_gpio_a_function(self.device, GpioAFunction::Gpio)
            }
            Group::Gpio_B_C_D_E_F_H => {
                reports::gpio::set_uart_pins(self.device, UartEnableMode::Off)
            }
            Group::Gpio_G => {
                reports::gpio::select_gpio_g_function(self.device, GpioGFunction::Gpio)
            }
        }
    }

    /// Disable GPIO function and reset as default function
    pub fn disable_pin(&self, group: Group) -> Ft260Result<()> {
        match group {
            Group::Gpio_0_1 => reports::gpio::set_i2c_pins(self.device, I2cEnableMode::Enabled),
            Group::Gpio_2 => {
                reports::gpio::select_gpio_2_function(self.device, Gpio2Function::SuspOut)
            }
            Group::Gpio_3 => {
                reports::ft260_set_wakeup_interrupt(self.device, WakeupIntEnableMode::Enabled)
            }
            Group::Gpio_4_5 => {
                reports::gpio::set_dcd_ri_pins(self.device, UartDcdRiEnableMode::Enabled)
            }
            Group::Gpio_A => {
                reports::gpio::select_gpio_a_function(self.device, GpioAFunction::TxActive)
            }
            Group::Gpio_B_C_D_E_F_H => {
                reports::gpio::set_uart_pins(self.device, UartEnableMode::NoFlowControl)
            }
            Group::Gpio_G => {
                reports::gpio::select_gpio_g_function(self.device, GpioGFunction::BcdDet)
            }
        }
    }

    fn pin_to_num(pin: Pin) -> GpioPinNum {
        match pin {
            Pin::Gpio0 => GpioPinNum::GPIO_0,
            Pin::Gpio1 => GpioPinNum::GPIO_1,
            Pin::Gpio2 => GpioPinNum::GPIO_2,
            Pin::Gpio3 => GpioPinNum::GPIO_3,
            Pin::Gpio4 => GpioPinNum::GPIO_4,
            Pin::Gpio5 => GpioPinNum::GPIO_5,
            Pin::GpioA => GpioPinNum::GPIO_A,
            Pin::GpioB => GpioPinNum::GPIO_B,
            Pin::GpioC => GpioPinNum::GPIO_C,
            Pin::GpioD => GpioPinNum::GPIO_D,
            Pin::GpioE => GpioPinNum::GPIO_E,
            Pin::GpioF => GpioPinNum::GPIO_F,
            Pin::GpioG => GpioPinNum::GPIO_G,
            Pin::GpioH => GpioPinNum::GPIO_H,
        }
    }

    /// Set direction of GPIO
    pub fn set_dir(&self, pin_sel: Pin, dir: Dir) -> Ft260Result<()> {
        let dir = match dir {
            Dir::In => GpioDir::In,
            Dir::Out => GpioDir::Out,
        };
        reports::gpio::set_dir(self.device, Self::pin_to_num(pin_sel), dir)
    }

    /// Set output value from GPIO pin
    pub fn write(&self, pin_sel: Pin, val_out: Val) -> Ft260Result<()> {
        let val_out = match val_out {
            Val::Low => GpioValue::Low,
            Val::High => GpioValue::High,
        };
        reports::gpio::write(self.device, Self::pin_to_num(pin_sel), val_out)
    }

    /// Get input/output value of GPIO pin
    pub fn read(&self, pin_sel: Pin) -> Ft260Result<Val> {
        let pin_sel = Self::pin_to_num(pin_sel);

        let res = reports::gpio::read(self.device, pin_sel);
        if let Ok(val) = res {
            Ok(match val {
                GpioValue::Low => Val::Low,
                GpioValue::High => Val::High,
            })
        } else {
            Err(res.err().unwrap())
        }
    }

    fn set_pin_params(&self, pin_sel: Pin, req: Request) -> Ft260Result<()> {
        reports::gpio::set_pin_params(self.device, Self::pin_to_num(pin_sel), req)
    }

    /// Set pull-up
    pub fn set_pull_up(&self, pin_sel: Pin) -> Ft260Result<()> {
        self.set_pin_params(pin_sel, Request::SetGpioPullUp)
    }

    /// Set pull-down
    pub fn set_pull_down(&self, pin_sel: Pin) -> Ft260Result<()> {
        self.set_pin_params(pin_sel, Request::SetGpioPullDown)
    }

    /// Configure pins for open-drain output
    pub fn set_open_drain(&self, pin_sel: Pin) -> Ft260Result<()> {
        self.set_pin_params(pin_sel, Request::SetGpioOpenDrain)
    }
}
