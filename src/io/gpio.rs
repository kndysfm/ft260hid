use crate::hid::consts::*;
use crate::hid::reports;
use crate::{device::Device, Ft260Error, Ft260Result};

#[derive(Debug)]
pub struct Gpio<'a> {
    device: &'a Device,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pin {
    Gpio0,
    Gpio1,
    Gpio2,
    Gpio3,
    Gpio4,
    Gpio5,
    GpioA,
    GpioB,
    GpioC,
    GpioD,
    GpioE,
    GpioF,
    GpioG,
    GpioH,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Gpio_0_1,
    Gpio_2,
    Gpio_3,
    Gpio_4_5,
    Gpio_A,
    Gpio_B_C_D_E_F_H,
    Gpio_G,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    In,
    Out,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Val {
    Low,
    High,
}

impl<'a> Gpio<'a> {
    pub fn new(device: &'a Device) -> Self {
        Self { device }
    }

    /// enable GPIO function for specific pin group
    pub fn enable_pin(&self, group: Group) -> Ft260Result<()> {
        match group {
            Group::Gpio_0_1 => reports::gpio::set_i2c_pins(self.device, I2cEnableMode::Disabled),
            Group::Gpio_2 => reports::gpio::select_gpio_2_function(self.device, Gpio2Function::Gpio),
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

    /// disable GPIO function and reset as default function
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

    /// set direction of GPIO
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
