///
/// "LibFT260.h" was referred
/// [Link](https://ftdichip.com/wp-content/uploads/2022/10/LibFT260-v1.1.6.zip)
/// 

use bitflags::bitflags;

/// Pin configuration of DIO7
#[repr(u8)]
pub(crate) enum Gpio2Fn
{
    /// GPIO2
    GPIO = 0u8,
    /// [default] the indicator when entering the USB suspending state
    SUSPOUT = 1u8,
    /// as the power enable indicator when the FT260 is USB enumerated
    PWREN = 2u8,
    /// 
    TX_LED = 4u8,
}

/// Pin configuration of DIO0
#[repr(u8)]
pub(crate) enum GpioAFn
{
    /// GPIOA
    GPIO = 0u8,
    /// [default] to indicate the UART transmitting is active
    TX_ACTIVE = 3u8,
    /// as the LED driving source when data is transmitted on UART TX port
    TX_LED = 4u8,
}

/// Pin configuration of DIO13
#[repr(u8)]
pub(crate) enum GpioGFn
{
    /// GPIOG
    GPIO = 0u8,
    /// as the power enable indicator when FT260 is USB enumerated. Low active
    PWREN = 2u8,
    /// as the LED driving source when data is received on UART RX port
    RX_LED = 5u8,
    /// [default] as the battery charger detection indicator output when the device is connected to a dedicated battery charger port
    BCD_DET = 6u8,
}

/// operation clock
#[repr(u8)]
pub(crate) enum Clock
{
    MHZ_12 = 0u8,
    MHZ_24,
    MHZ_48,
}

/// interrupt trigger by input to GPIO3 (DIO8)
#[repr(u8)]
pub(crate) enum InterruptTrigger
{
    RISING_EDGE = 0u8,
    LEVEL_HIGH,
    FALLING_EDGE,
    LEVEL_LOW,
}

#[repr(u8)]
pub(crate) enum InterruptLevelTimeDelay
{
    MS_1 = 1u8,
    MS_5,
    MS_30,
}

#[repr(u8)]
pub(crate) enum SuspendOutPolarity
{
    LEVEL_HIGH = 0u8,
    LEVEL_LOW,
}

#[repr(u8)]
pub(crate) enum UartMode
{
    OFF = 0u8,
    RTS_CTS_MODE,
    DTR_DSR_MODE,
    XON_XOFF_MODE,
    NO_FLOW_CTRL_MODE,
}

#[repr(u8)]
pub(crate) enum UartDataBit
{
    BIT_7 = 7u8,
    BIT_8 = 8u8,
}

#[repr(u8)]
pub(crate) enum UartStopBit
{
    BITS_1 = 0u8,
    BITS_2 = 2u8,
}

#[repr(u8)]
pub(crate) enum UartParity
{
    NONE = 0u8,
    ODD,
    EVEN,
    MARK,
    SPACE,
}

#[repr(u8)]
pub(crate) enum UartBreaking
{
    Off,
    On,
}

/// RI, Ring Indicator, can be enabled via a USB command for the UART interface. 
/// RI may be used as an alternative to WAKEUP for waking up the USB host.
#[repr(u8)]
pub(crate) enum UartRIWakeup
{
    RISING_EDGE = 0u8,
    FALLING_EDGE,
}

#[repr(u8)]
pub(crate) enum GpioDir
{
    IN = 0u8,
    OUT,
}

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

/// Represents a set of flags.
bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub(crate) struct I2cFlag: u8 {
    const NONE                      = 0x00;
    const START                     = 0x02;
    const REPEATED_START            = 0x03;
    const STOP                      = 0x04;
    const START_AND_STOP            = 0x06;
    const REPEATED_START_AND_STOP   = (0x03 | 0x06);
  }
}

/// ref :  "LibFT260-v1.1.5\samples\I2C\i2c.cpp"
/// I2C Master Controller Status (I2Cstauts variable)
///   bit 0 = controller busy: all other status bits invalid
///   bit 1 = error condition
///   bit 2 = slave address was not acknowledged during last operation
///   bit 3 = data not acknowledged during last operation
///   bit 4 = arbitration lost during last operation
///   bit 5 = controller idle
///   bit 6 = bus busy
bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub(crate) struct I2cStatus: u8 {
    const ControllerBusy        = 1 << 0;
    const Error                 = 1 << 1;
    const AddrNack              = 1 << 2;
    const DataNack              = 1 << 3;
    const ArbitrationLost       = 1 << 4;
    const ControllerIdle        = 1 << 5;
    const BusBusy               = 1 << 6;
  }
}

#[repr(u8)]
pub(crate) enum Param
{
    DS_CTL0 = 0x50u8,
    DS_CTL3 = 0x51u8,
    DS_CTL4 = 0x52u8,
    SR_CTL0 = 0x53u8,
    GPIO_PULL_UP = 0x61u8,
    GPIO_OPEN_DRAIN = 0x62u8,
    GPIO_PULL_DOWN = 0x63u8,
    GPIO_GPIO_SLEW_RATE = 0x65u8,
}

#[repr(u8)]
pub(crate) enum Param2
{
    GPIO_GROUP_SUSPEND_0 = 0x10u8,
    GPIO_GROUP_SUSPEND_A = 0x11u8,
    GPIO_DRIVE_STRENGTH = 0x64u8,
}
