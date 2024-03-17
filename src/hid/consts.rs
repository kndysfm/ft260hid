use bitflags::bitflags;

#[repr(u8)]
pub(crate) enum Ft260Status
{
    OK = 0,
    INVALID_HANDLE,
    DEVICE_NOT_FOUND,
    DEVICE_NOT_OPENED,
    DEVICE_OPEN_FAIL,
    DEVICE_CLOSE_FAIL,
    INCORRECT_INTERFACE,
    INCORRECT_CHIP_MODE,
    DEVICE_MANAGER_ERROR,
    IO_ERROR,
    INVALID_PARAMETER,
    NULL_BUFFER_POINTER,
    BUFFER_SIZE_ERROR,
    UART_SET_FAIL,
    RX_NO_DATA,
    GPIO_WRONG_DIRECTION,
    INVALID_DEVICE,
    INVALID_OPEN_DRAIN_SET,
    INVALID_OPEN_DRAIN_RESET,
    I2C_READ_FAIL,
    OTHER_ERROR,
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub(crate) struct ChipMode: u8 {
    const Dcnf0  = 0x01;
    const Dcnf1  = 0x02;
  }
}

#[repr(u8)]

pub(crate) enum ClkCtl
{
    _12MHz = 0,
    _24MHz = 1,
    _48MHz = 2,
}

#[repr(u8)]
pub(crate) enum SuspendStatus
{
    NotSuspended = 0,
    Suspended = 1,
}

#[repr(u8)]
pub(crate) enum PwrEnStatus
{
    NotReqdy = 0,
    Ready = 1,
}

#[repr(u8)]
pub(crate) enum I2cEnableMode
{ 
    Disabled = 0,
    Enabled = 1,
}

#[repr(u8)]
pub(crate) enum UartEnableMode
{
    Off = 0,
    RtsCts = 1,
    DtrDsr = 2,
    XonXoff = 3,
    NoFlowControl = 4,
}

#[repr(u8)]
pub(crate) enum HidOverI2cEnableMode
{
    NotConfigured = 0,
    Configured = 1,
}

#[repr(u8)]
pub(crate) enum UartDcdRiEnableMode
{
    Disabled = 0,
    Enabled = 1,
}

#[repr(u8)]
pub(crate) enum Gpio2Function
{
    Gpio = 0,
    SuspOut = 1,
    _PwrEn = 2,
    TxLed = 4,
}
/// Byte 9
#[repr(u8)]
pub(crate) enum GpioAFunction
{
    Gpio = 0,
    TxActive = 3,
    TxLed = 4,
}

#[repr(u8)]
pub(crate) enum GpioGFunction
{
    Gpio= 0,
    _PwrEn = 2,
    RxLed = 5,
    BcdDet = 6,
}

#[repr(u8)]
pub(crate) enum SuspendOutPol
{
    High = 0, // suspend output active high
    Low = 1, // suspend output active low
}

#[repr(u8)]
pub(crate) enum WakeupIntEnableMode
{
    Disabled = 0, // the pin acts as GPIO3
    Enabled = 1,
}

/// tigger conditions on the interrupt pin
#[repr(u8)]
pub(crate) enum InterruptTrigger
{
    Rising = 0x00,
    High = 0x01,
    Falling = 0x02,
    Low = 0x03,
}
/// interrupt level duration select
#[repr(u8)]
pub(crate) enum InterruptDuration
{
    _1ms = 0x04,
    _5ms = 0x08,
    _30ms = 0x0C,
}
#[repr(u8)]
pub(crate) enum PowerSavingEnableMode
{ 
    Disable = 0,
    Enable = 1,
}

#[repr(u8)]
pub(crate) enum UartParity
{
    None = 0,
    Odd = 1,
    Even = 2,
    High = 3, // parity bit is alwasy high
    Low = 4, // parity bit is alwasy low
}

#[repr(u8)]
pub(crate) enum UartStopBit
{
    One = 0,
    Two = 2,
}
#[repr(u8)]
pub(crate) enum UartBreaking
{
    NoBreak = 0,
    Break = 1,
}
#[repr(u8)]
pub(crate) enum UartDataBits
{
    Seven = 7,
    Eight = 8,
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub(crate) struct UartDcdRiStatus : u8 {
    const Dcd  = 1;
    const Ri  = 2;
  }
}

#[repr(u8)]
pub(crate) enum UartRiWakeupConfig
{
    RisingEdge = 0,
    FallingEdge = 1, // (default)
}

/// For Report ID 0xA1
#[repr(u8)]
pub(crate) enum Request
{
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
    SetUartDataBit = 0x43,
    SetUartParity = 0x44,
    SetUartStopBit = 0x45,
    SetUartBreaking = 0x46,
    SetUartXonXoff = 0x49,
    // FT260_SetParam_U8 Params
    SetDriveStrength0 = 0x50,
    SetDriveStrength3 = 0x51,
    SetDriveStrength4 = 0x52,
    SetSlewRate0 = 0x53,
    SetGpioPullUp = 0x61,       // takes GpioBitVal
    SetGpioOpenDrain = 0x62,    // takes GpioBitVal
    SetGpioPullDown = 0x63,     // takes GpioBitVal
    SetGpioSlewRate = 0x65,     // takes GpioBitVal
    // FT260_SetParam_U16 Params
    SetSuspendModeGpio0 = 0x10, // GPIO 0-5
    SetSuspendModeGpioA = 0x11, // GPIO A-H
    SetDriveStrengthGpio0 = 0x64, // GPIO 0-5
}

bitflags! {
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
  pub(crate) struct I2cContition : u8
  {
    const None            = 0;
    const Start           = 2;
    const ReStart         = 3;
    const Stop            = 4;
    const StartAndStop    = 6;
  }
}

#[repr(u8)]
pub(crate) enum GpioBitPos
{
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

#[repr(u8)]
pub(crate) enum GpioDir
{
    In = 0,
    Out = 1,
}

#[repr(u8)]
pub(crate) enum GpioExBitPos
{
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

/// Each report length value include ID
pub(crate) const REPORT_LENGTH_FEATURE_CHIP_VENDOR: usize = 13;
pub(crate) const REPORT_LENGTH_FEATURE_SYSTEM_STATUS: usize = 26;
