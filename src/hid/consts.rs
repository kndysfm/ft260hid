///
/// "LibFT260.h" was referred
/// [Link](https://ftdichip.com/wp-content/uploads/2022/10/LibFT260-v1.1.6.zip)
///
use bitflags::bitflags;

use crate::{Ft260Error};

// see "https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer"
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u8> for $name {
            type Error = Ft260Error;
            fn try_from(v: u8) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u8 => Ok($name::$vname),)*
                    _ => Err(Ft260Error::ByteError {
                                value: v,
                                message: "Failed converting byte into enum value".to_string() }),
                }
            }
        }
    }
}

#[repr(u8)]
pub(crate) enum Ft260Status {
    Ok = 0,
    InvalidHandle,
    DeviceNotFound,
    DeviceNotOpened,
    DeviceOpenFail,
    DeviceCloseFail,
    IncorrectInterface,
    IncorrectChipMode,
    DeviceManagerError,
    IoError,
    InvalidParameter,
    NullBufferPointer,
    BufferSizeError,
    UartSetFail,
    RxNoData,
    GpioWrongDirection,
    InvalidDevice,
    InvalidOpenDrainSet,
    InvalidOpenDrainReset,
    I2cReadFail,
    OtherError,
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub(crate) struct ChipMode: u8 {
    const Dcnf0  = 0x01;
    const Dcnf1  = 0x02;
  }
}

/// operation clock
#[repr(u8)]
pub(crate) enum ClkCtl {
    _12MHz = 0,
    _24MHz = 1,
    _48MHz = 2,
}

#[repr(u8)]
pub(crate) enum SuspendStatus {
    NotSuspended = 0,
    Suspended = 1,
}

#[repr(u8)]
pub(crate) enum PwrEnStatus {
    NotReqdy = 0,
    Ready = 1,
}

#[repr(u8)]
pub(crate) enum I2cEnableMode {
    Disabled = 0,
    Enabled = 1,
}

back_to_enum! {
#[derive(Debug)]
#[repr(u8)]
pub(crate) enum UartEnableMode
{
    Off = 0,
    RtsCts = 1,
    DtrDsr = 2,
    XonXoff = 3,
    NoFlowControl = 4,
}
}

#[repr(u8)]
pub(crate) enum HidOverI2cEnableMode {
    NotConfigured = 0,
    Configured = 1,
}

back_to_enum! {
#[repr(u8)]
pub(crate) enum UartDcdRiEnableMode
{
    Disabled = 0,
    Enabled = 1,
}
}

/// Pin configuration of DIO7
#[repr(u8)]
pub(crate) enum Gpio2Function {
    /// GPIO2
    Gpio = 0,
    /// [default] the indicator when entering the USB suspending state
    SuspOut = 1,
    /// as the power enable indicator when the FT260 is USB enumerated
    PwrEn = 2,
    /// the power enable indicator when the FT260 is USB enumerated
    TxLed = 4,
}

/// Pin configuration of DIO0
#[repr(u8)]
pub(crate) enum GpioAFunction {
    /// GPIOA
    Gpio = 0,
    /// [default] to indicate the UART transmitting is active
    TxActive = 3,
    /// as the LED driving source when data is transmitted on UART TX port
    TxLed = 4,
}

/// Pin configuration of DIO13
#[repr(u8)]
pub(crate) enum GpioGFunction {
    /// GPIOG
    Gpio = 0,
    /// as the power enable indicator when FT260 is USB enumerated. Low active
    PwrEn = 2,
    /// as the LED driving source when data is received on UART RX port
    RxLed = 5,
    /// [default] as the battery charger detection indicator output when the device is connected to a dedicated battery charger port
    BcdDet = 6,
}

#[repr(u8)]
pub(crate) enum SuspendOutPol {
    High = 0, // suspend output active high
    Low = 1,  // suspend output active low
}

#[repr(u8)]
pub(crate) enum WakeupIntEnableMode {
    Disabled = 0, // the pin acts as GPIO3
    Enabled = 1,
}

/// interrupt trigger by input to GPIO3 (DIO8)
/// tigger conditions on the interrupt pin
#[repr(u8)]
pub(crate) enum InterruptTrigger {
    Rising = 0x00,
    High = 0x01,
    Falling = 0x02,
    Low = 0x03,
}
/// interrupt level duration select
#[repr(u8)]
pub(crate) enum InterruptDuration {
    _1ms = 0x04,
    _5ms = 0x08,
    _30ms = 0x0C,
}
#[repr(u8)]
pub(crate) enum PowerSavingEnableMode {
    Disable = 0,
    Enable = 1,
}

back_to_enum! {
#[derive(Debug)]
#[repr(u8)]
pub(crate) enum UartParity
{
    None = 0,
    Odd = 1,
    Even = 2,
    High = 3, // parity bit is always high
    Low = 4, // parity bit is always low
}
}

back_to_enum! {
#[derive(Debug)]
#[repr(u8)]
pub(crate) enum UartStopBit
{
    One = 0,
    Two = 2,
}
}

back_to_enum! {
#[derive(Debug)]
#[repr(u8)]
pub(crate) enum UartBreaking
{
    NoBreak = 0,
    Break = 1,
}
}

back_to_enum! {
#[derive(Debug)]
#[repr(u8)]
pub(crate) enum UartDataBits
{
    Seven = 7,
    Eight = 8,
}
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub(crate) struct UartDcdRiStatus : u8 {
    const Dcd  = 1;
    const Ri  = 2;
  }
}

/// RI, Ring Indicator, can be enabled via a USB command for the UART interface.
/// RI may be used as an alternative to WAKEUP for waking up the USB host.
#[repr(u8)]
pub(crate) enum UartRiWakeupConfig {
    RisingEdge = 0,
    FallingEdge = 1, // (default)
}

/// For Report ID 0xA1
#[repr(u8)]
pub(crate) enum Request {
    SetClock = 0x01,
    SetI2cMode = 0x02,
    SetUartMode = 0x03,
    /// <summary>
    /// Usage of GPIO3
    /// </summary>
    EnableInterruptWakeUp = 0x05,
    /// <summary>
    /// Usage of GPIO2
    /// </summary>
    SelectGpio2Function = 0x06,
    /// <summary>
    /// </summary>
    EnableUartDcdRi = 0x07,
    /// <summary>
    /// Usage of GPIOA
    /// </summary>
    SelectGpioAFunction = 0x08,
    /// <summary>
    /// Usage of GPIOG
    /// </summary>
    SelectGpioGFunction = 0x09,
    /// <summary>
    ///
    /// </summary>
    SetInterruptTriggerCondition = 0x0A,
    /// <summary>
    /// Suspend Output Polarity
    /// </summary>
    SetSuspendOutPol = 0x0B,
    /// <summary>
    /// Enable UART RI remote wakeup capability
    /// </summary>
    EnableUartRiWaveup = 0x0C,
    SetUartRiWakeupConfig = 0x0D,
    ResetI2c = 0x20,
    SetI2cClockSpeed = 0x22,
    ResetUart = 0x40,
    ConfigureUart = 0x41,
    SetUartBaudRate = 0x42,
    SetUartDataBits = 0x43,
    SetUartParity = 0x44,
    SetUartStopBit = 0x45,
    SetUartBreaking = 0x46,
    SetUartXonXoff = 0x49,
    // FT260_SetParam_U8 Params
    SetDriveStrength0 = 0x50,
    SetDriveStrength3 = 0x51,
    SetDriveStrength4 = 0x52,
    SetSlewRate0 = 0x53,
    SetGpioPullUp = 0x61,    // takes GpioBitVal
    SetGpioOpenDrain = 0x62, // takes GpioBitVal
    SetGpioPullDown = 0x63,  // takes GpioBitVal
    SetGpioSlewRate = 0x65,  // takes GpioBitVal
    // FT260_SetParam_U16 Params
    SetSuspendModeGpio0 = 0x10,   // GPIO 0-5
    SetSuspendModeGpioA = 0x11,   // GPIO A-H
    SetDriveStrengthGpio0 = 0x64, // GPIO 0-5
}

bitflags! {
    /// ref :  "LibFT260-v1.1.5\samples\I2C\i2c.cpp"
    /// I2C Master Controller Status (I2Cstauts variable)
    ///   bit 0 = controller busy: all other status bits invalid
    ///   bit 1 = error condition
    ///   bit 2 = slave address was not acknowledged during last operation
    ///   bit 3 = data not acknowledged during last operation
    ///   bit 4 = arbitration lost during last operation
    ///   bit 5 = controller idle
    ///   bit 6 = bus busy
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub(crate) struct I2cBusStatus : u8
  {
    const ControllerBusy    = 0x01;
    const Error             = 0x02;
    const AddressNack       = 0x04;
    const DataNack          = 0x08;
    const ArbitrationLost   = 0x10;
    const ControllerIdle    = 0x20;
    const BusBusy           = 0x40;
  }
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub(crate) struct I2cCondition : u8
  {
    const None            = 0;
    const Start           = 2;
    const ReStart         = 3;
    const Stop            = 4;
    const StartAndStop    = 6;
    const ReStartAndStop  = 7;
  }
}

#[repr(u8)]
pub(crate) enum GpioBitPos {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
}

bitflags! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct GpioBitVal : u8
{
    const None   = 0;
    const _0   = 1 << 0;
    const _1   = 1 << 1;
    const _2   = 1 << 2;
    const _3   = 1 << 3;
    const _4   = 1 << 4;
    const _5   = 1 << 5;
    const All = Self::_0.bits() |
                Self::_1.bits() |
                Self::_2.bits() |
                Self::_3.bits() |
                Self::_4.bits() |
                Self::_5.bits() ;
}
}

bitflags! {
/// Represents a set of flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct GpioPinNum: u16 {
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
pub(crate) enum GpioDir {
    In = 0,
    Out = 1,
}

#[repr(u8)]
pub(crate) enum GpioValue {
    Low = 0,
    High = 1,
}

#[repr(u8)]
pub(crate) enum GpioExBitPos {
    _A = 0,
    _B = 1,
    _C = 2,
    _D = 3,
    _E = 4,
    _F = 5,
    _G = 6,
    _H = 7,
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub(crate) struct GpioExBitVal : u8
  {
    const None  = 0;
    const _A  = 1 << 0;
    const _B  = 1 << 1;
    const _C  = 1 << 2;
    const _D  = 1 << 3;
    const _E  = 1 << 4;
    const _F  = 1 << 5;
    const _G  = 1 << 6;
    const _H  = 1 << 7;
    const All = Self::_A.bits() |
                Self::_B.bits() |
                Self::_C.bits() |
                Self::_D.bits() |
                Self::_E.bits() |
                Self::_F.bits() |
                Self::_G.bits() |
                Self::_H.bits() ;
  }
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub(crate) struct UartInterruptStatus : u8
  {
    const Dcd  = 1;
    const Ri  = 2;
  }
}

back_to_enum! {
#[repr(u8)]
pub(crate) enum ReportId
{
    FeatChipCode = 0xA0, // Feature Chip code
    FeatSystemSetting = 0xA1, // Feature System Setting
    FeatGpio = 0xB0, // Feature GPIO
    InInterruptStatus = 0xB1, // Input Interrupt Status(from UART interface)
    /// I2c reports
    FeatI2cStatus = 0xC0, // Feature I2C Status
    OutI2cReadRequest = 0xC2, // Output I2C Read Request
    // 0xD0 to 0xDE Input, Output I2C Report
    InOutI2cReport04 = 0xD0,
    InOutI2cReport08 = 0xD1,
    InOutI2cReport0C = 0xD2,
    InOutI2cReport10 = 0xD3,
    InOutI2cReport14 = 0xD4,
    InOutI2cReport18 = 0xD5,
    InOutI2cReport1C = 0xD6,
    InOutI2cReport20 = 0xD7,
    InOutI2cReport24 = 0xD8,
    InOutI2cReport28 = 0xD9,
    InOutI2cReport2C = 0xDA,
    InOutI2cReport30 = 0xDB,
    InOutI2cReport34 = 0xDC,
    InOutI2cReport38 = 0xDD,
    InOutI2cReport3C = 0xDE,
    InOutI2cReportOverflow = 0xDF,
    /// Uart reports
    FeatUartStatus = 0xE0, // Feature UART Status
    FeatUartRiAndDcdStatus = 0xE2, // Feature UART RI and DCD Status
    // 0xF0 to 0xFE Input, Output UART Report
    InOutUartReport04 = 0xF0,
    InOutUartReport08 = 0xF1,
    InOutUartReport0C = 0xF2,
    InOutUartReport10 = 0xF3,
    InOutUartReport14 = 0xF4,
    InOutUartReport18 = 0xF5,
    InOutUartReport1C = 0xF6,
    InOutUartReport20 = 0xF7,
    InOutUartReport24 = 0xF8,
    InOutUartReport28 = 0xF9,
    InOutUartReport2C = 0xFA,
    InOutUartReport30 = 0xFB,
    InOutUartReport34 = 0xFC,
    InOutUartReport38 = 0xFD,
    InOutUartReport3C = 0xFE,
    InOutUartReportOverflow = 0xFF,
}
}

/// Each report length value include ID
pub(crate) const REPORT_LENGTH_FEATURE_CHIP_VENDOR: usize = 13;
pub(crate) const REPORT_LENGTH_FEATURE_SYSTEM_STATUS: usize = 26;
