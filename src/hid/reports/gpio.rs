use crate::device::Device;
use crate::hid::consts::*;
use crate::{Ft260Error, Ft260Result};
use crate::hid::reports::*;

pub(crate) fn select_gpio_2_function(
  device: &Device,
  gpio2_function: Gpio2Function,
) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SelectGpio2Function, gpio2_function as u8)
}

pub(crate) fn select_gpio_a_function(
  device: &Device,
  gpio_a_function: GpioAFunction,
) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SelectGpioAFunction, gpio_a_function as u8)
}

pub(crate) fn select_gpio_g_function(
  device: &Device,
  gpio_g_function: GpioGFunction,
) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SelectGpioGFunction, gpio_g_function as u8)
}

pub(crate) fn set_suspend_out_polarity(
  device: &Device,
  polarity: SuspendOutPol,
) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SetSuspendOutPol, polarity as u8)
}

pub(crate) fn set_i2c_pins(device: &Device, enable: I2cEnableMode) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SetI2cMode, enable as u8)
}

pub(crate) fn set_uart_pins(device: &Device, enable: UartEnableMode) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SetUartMode, enable as u8)
}

pub(crate) fn set_dcd_ri_pins(
  device: &Device,
  enable: UartDcdRiEnableMode,
) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::EnableUartDcdRi, enable as u8)
}

#[derive(Clone, Copy)]
struct GpioRequest {
    /// GPIO 0-5 pin value
    pub val: GpioBitVal,
    /// GPIO 0-5 direction (0: input, 1: output)
    pub dir: GpioBitVal,
    /// GPIO A-H pin value
    pub ex_val: GpioExBitVal,
    /// GPIO A-H direction (0: input, 1: output)
    pub ex_dir: GpioExBitVal,
}

/// 4.7.1 GPIO Write Request
fn set(device: &Device, report: GpioRequest) -> Ft260Result<()> {
    ft260_set_feature(
        device,
        &[
            ReportId::FeatGpio as u8,
            report.val.bits(),
            report.dir.bits(),
            report.ex_val.bits(),
            report.ex_dir.bits(),
        ],
    )
}

/// 4.7.2 GPIO Read Request
fn get(device: &Device) -> Ft260Result<GpioRequest> {
    let rid = ReportId::FeatGpio as u8;
    let res = ft260_get_feature(device, rid);
    if let Ok(report) = res {
        assert_eq!(rid, report[0]);
        Ok(GpioRequest {
            val: GpioBitVal::from_bits(report[1]).unwrap(),
            dir: GpioBitVal::from_bits(report[2]).unwrap(),
            ex_val: GpioExBitVal::from_bits(report[3]).unwrap(),
            ex_dir: GpioExBitVal::from_bits(report[4]).unwrap(),
        })
    } else {
        Err(res.err().unwrap())
    }
}

fn pin_to_bits(pin: GpioPinNum) -> (GpioBitVal, GpioExBitVal) {
    let mut bits: GpioBitVal = GpioBitVal::None;
    if pin.contains(GpioPinNum::GPIO_0) {
        bits |= GpioBitVal::_0;
    }
    if pin.contains(GpioPinNum::GPIO_1) {
        bits |= GpioBitVal::_1;
    }
    if pin.contains(GpioPinNum::GPIO_2) {
        bits |= GpioBitVal::_2;
    }
    if pin.contains(GpioPinNum::GPIO_3) {
        bits |= GpioBitVal::_3;
    }
    if pin.contains(GpioPinNum::GPIO_4) {
        bits |= GpioBitVal::_4;
    }
    if pin.contains(GpioPinNum::GPIO_5) {
        bits |= GpioBitVal::_5;
    }
    let mut ex_bits: GpioExBitVal = GpioExBitVal::None;
    if pin.contains(GpioPinNum::GPIO_A) {
        ex_bits |= GpioExBitVal::_A;
    }
    if pin.contains(GpioPinNum::GPIO_B) {
        ex_bits |= GpioExBitVal::_B;
    }
    if pin.contains(GpioPinNum::GPIO_C) {
        ex_bits |= GpioExBitVal::_C;
    }
    if pin.contains(GpioPinNum::GPIO_D) {
        ex_bits |= GpioExBitVal::_D;
    }
    if pin.contains(GpioPinNum::GPIO_E) {
        ex_bits |= GpioExBitVal::_E;
    }
    if pin.contains(GpioPinNum::GPIO_F) {
        ex_bits |= GpioExBitVal::_F;
    }
    if pin.contains(GpioPinNum::GPIO_G) {
        ex_bits |= GpioExBitVal::_G;
    }
    if pin.contains(GpioPinNum::GPIO_H) {
        ex_bits |= GpioExBitVal::_H;
    }

    (bits, ex_bits)
}

pub(crate) fn set_dir(
    device: &Device,
    pin: GpioPinNum,
    dir: GpioDir,
) -> Ft260Result<()> {
    let res = get(device);
    if let Ok(req) = res {
        let mut req = req;
        let (bit, ex_bit) = pin_to_bits(pin);
        // 0: input, 1: output
        let bit_dir = match dir {
            GpioDir::In => false,
            GpioDir::Out => true,
        };
        req.dir.set(bit, bit_dir);
        req.ex_dir.set(ex_bit, bit_dir);
        set(device, req)
    } else {
        Err(res.err().unwrap())
    }
}

pub(crate) fn read(device: &Device, pin: GpioPinNum) -> Ft260Result<GpioValue> {
    let res = get(device);
    if let Ok(req) = res {
        let (bit, ex_bit) = pin_to_bits(pin);
        if req.val.intersects(bit) || req.ex_val.intersects(ex_bit) {
            Ok(GpioValue::High)
        } else {
            Ok(GpioValue::Low)
        }
    } else {
        Err(res.err().unwrap())
    }
}

pub(crate) fn write(
    device: &Device,
    pin: GpioPinNum,
    val: GpioValue,
) -> Ft260Result<()> {
    let res = get(device);
    if let Ok(req) = res {
        let mut req = req;
        let (bit, ex_bit) = pin_to_bits(pin);
        let bit_val = match val {
            GpioValue::Low => false,
            GpioValue::High => true,
        };
        req.val.set(bit, bit_val);
        req.ex_val.set(ex_bit, bit_val);
        set(device, req)
    } else {
        Err(res.err().unwrap())
    }
}

pub(crate) fn set_pin_params(
  device: &Device,
  pin: GpioPinNum,
  req: Request
) -> Ft260Result<()> {
  ft260_set_request_u8(device, req, (pin.bits() & 0x3F) as u8)
}
