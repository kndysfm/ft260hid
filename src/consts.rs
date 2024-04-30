///
/// "LibFT260.h" was referred
/// [Link](https://ftdichip.com/wp-content/uploads/2022/10/LibFT260-v1.1.6.zip)
/// 

use bitflags::bitflags;


bitflags! {
  /// Represents a set of flags.
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub(crate) struct GpioSel: u16 {
    const GPIO_0  = 1 << 0;
    const GPIO_1  = 1 << 1;
    const GPIO_2  = 1 << 2;
    const GPIO_3  = 1 << 3;
    const GPIO_4  = 1 << 4;
    const GPIO_5  = 1 << 5;
    const GPIO_A  = 1 << 6;
    const GPIO_B  = 1 << 7;
    const GPIO_C  = 1 << 8;
    const GPIO_D  = 1 << 9;
    const GPIO_E  = 1 << 10;
    const GPIO_F  = 1 << 11;
    const GPIO_G  = 1 << 12;
    const GPIO_H  = 1 << 13;
  }
}

#[repr(u8)]
pub(crate) enum GpioGroup
{
    Gpio_0_1,
    Gpio_2,
    Gpio_3,
    Gpio_4_5,
    Gpio_A,
    Gpio_B_C_D_E_F_H,
    Gpio_G,
}

#[repr(u8)]
pub(crate) enum GpioValue
{
    Low = 0u8,
    High = 1u8,
}
