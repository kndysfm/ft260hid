use std::time::{Instant, Duration};

use bitflags::Flags;

use crate::device::{Device};
use crate::{Ft260Result, Ft260Error};
use crate::hid::consts::*;

type Report = Option<Vec<u8>>;

pub(crate) fn clear_input_report_queue_i2c(device: &Device) {
  let id = ReportId::InOutI2cReport04 as u8;
  device.fifo().clear(id);
}

pub(crate) fn pop_input_report_i2c(device: &Device) -> Report {
  let id = ReportId::InOutI2cReport04 as u8;
  device.fifo().pop_report(id)
}

pub(crate) fn get_input_reports_count_i2c(device: &Device) -> usize {
  let id = ReportId::InOutI2cReport04 as u8;
  device.fifo().len(id)
}

pub(crate) fn clear_input_report_queue_uart(device: &Device) {
  let id = ReportId::InOutUartReport04 as u8;
  device.fifo().clear(id);
}

pub(crate) fn pop_input_report_uart(device: &Device) -> Report {
  let id = ReportId::InOutUartReport04 as u8;
  device.fifo().pop_report(id)
}

pub(crate) fn get_input_reports_count_uart(device: &Device) -> usize {
  let id = ReportId::InOutUartReport04 as u8;
  device.fifo().len(id)
}

pub(crate) fn get_input_report_byte_amount_uart(device: &Device) -> usize {
  let id = ReportId::InOutUartReport04 as u8;
  let mut amount = 0usize;
  for rep in device.fifo().iter_peek(id) {
    if rep.len() > 1 {
      amount += rep[1] as usize; // length value
    }
  }
  amount
}

pub(crate) fn pop_input_report_int(device: &Device) -> Report{
  let id = ReportId::InInterruptStatus as u8;
  device.fifo().pop_report(id)
}


pub(crate) fn ft260_set_request(device: &Device, request: Request) -> Ft260Result<()> {
  device.set_feature(&[ReportId::FeatSystemSetting as u8, request as u8])
}

pub(crate) fn ft260_set_request_u8(device: &Device, request: Request, value: u8) -> Ft260Result<()> {
  device.set_feature(&[ReportId::FeatSystemSetting as u8, request as u8, value])
}

pub(crate) fn ft260_set_request_u8x2(device: &Device, request: Request, value1: u8, value2: u8) -> Ft260Result<()> {
  device.set_feature(&[ReportId::FeatSystemSetting as u8, request as u8, value1, value2])
}

pub(crate) fn ft260_set_request_u16(device: &Device, request: Request, value: u16) -> Ft260Result<()> {
  device.set_feature(&[ReportId::FeatSystemSetting as u8, request as u8, 
    ((value >> 0) & 0xFF) as u8,
    ((value >> 8) & 0xFF) as u8])
}

pub(crate) fn ft260_set_request_u32(device: &Device, request: Request, value: u32) -> Ft260Result<()> {
  device.set_feature(&[ReportId::FeatSystemSetting as u8, request as u8, 
    ((value >> 0) & 0xFF) as u8,
    ((value >> 8) & 0xFF) as u8,
    ((value >> 16) & 0xFF) as u8,
    ((value >> 24) & 0xFF) as u8])
}

pub(crate) fn ft260_set_feature(device: &Device, data: &[u8]) -> Ft260Result<()> {
  device.set_feature(data)
}

pub(crate) fn ft260_get_feature(device: &Device, report_id: u8) -> Ft260Result<[u8;64]> {
    let mut buf = [0u8;64];
    buf[0] = report_id;
    let res = device.get_feature(&mut buf);
    if let Ok(sz) = res {
        Ok(buf)
    } else {
        Err(res.err().unwrap())
    }
}

pub(crate) fn ft260_set_clock(device: &Device, clk: ClkCtl) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SetClock, clk as u8)
}

pub(crate) fn ft260_set_wakeup_interrupt(device: &Device, enable: WakeupIntEnableMode) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::EnableInterruptWakeUp, enable as u8)
}

pub(crate) fn ft260_set_interrupt_trigger_type(device: &Device,
    trigger: InterruptTrigger,
    delay: InterruptDuration) -> Ft260Result<()> {
  ft260_set_request_u8x2(device, Request::SetInterruptTriggerCondition, trigger as u8, delay as u8)
}

pub(crate) fn ft260_select_gpio2_function(device: &Device, gpio2_function: Gpio2Function) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SelectGpio2Function, gpio2_function as u8)
}

pub(crate) fn ft260_select_gpio_a_function(device: &Device, gpio_a_function: GpioAFunction) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SelectGpioAFunction, gpio_a_function as u8)
}

pub(crate) fn ft260_select_gpio_g_function(device: &Device, gpio_g_function: GpioGFunction) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SelectGpioGFunction, gpio_g_function as u8)
}

pub(crate) fn ft260_set_suspend_out_polarity(device: &Device, polarity: SuspendOutPol) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SetSuspendOutPol, polarity as u8)
}



pub(crate) fn ft260_get_chip_version(device: &Device) -> Ft260Result<u32> {
  let mut buf = [0u8; 64];
  buf[0] = ReportId::FeatChipCode as u8;
  let res = device.get_feature(&mut buf);
  if let Ok(sz) = res {
    if sz > 5 {
      let ver = ((buf[1] as u32) << 0) |
        ((buf[2] as u32) << 8) |
        ((buf[3] as u32) << 16) |
        ((buf[4] as u32) << 24) ;
      Ok(ver)
    } else {
      Err(Ft260Error::HidError { message: "Feature Report gotten is short".to_string() })
    }
  } else {
    Err(res.err().unwrap())
  }
}

        //pub(crate) FT260_STATUS FT260_GetLibVersion(/**[NativeTypeName("LPDWORD")]*/ out uint lpdwLibVersion);



pub(crate) fn ft260_enable_i2c_pin(device: &Device, enable: I2cEnableMode) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SetI2cMode, enable as u8)
}

pub(crate) fn ft260_enable_uart_pin(device: &Device, enable: UartEnableMode) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::SetUartMode, enable as u8)
}

pub(crate) fn ft260_enable_dcd_ri_pin(device: &Device, enable: UartDcdRiEnableMode) -> Ft260Result<()> {
  ft260_set_request_u8(device, Request::EnableUartDcdRi, enable as u8)
}



pub(crate) fn ft260_i2c_master_init(device: &Device, kbps: u16) -> Ft260Result<()> {
  ft260_set_request_u16(device, Request::SetI2cClockSpeed, kbps)
}

fn decide_i2c_report_id(length: usize) -> ReportId {
    if length <= 0x04 { return ReportId::InOutI2cReport04; }
    if length <= 0x08 { return ReportId::InOutI2cReport08; }
    if length <= 0x0C { return ReportId::InOutI2cReport0C; }
    if length <= 0x10 { return ReportId::InOutI2cReport10; }
    if length <= 0x14 { return ReportId::InOutI2cReport14; }
    if length <= 0x18 { return ReportId::InOutI2cReport18; }
    if length <= 0x1C { return ReportId::InOutI2cReport1C; }
    if length <= 0x20 { return ReportId::InOutI2cReport20; }
    if length <= 0x24 { return ReportId::InOutI2cReport24; }
    if length <= 0x28 { return ReportId::InOutI2cReport28; }
    if length <= 0x2C { return ReportId::InOutI2cReport2C; }
    if length <= 0x30 { return ReportId::InOutI2cReport30; }
    if length <= 0x34 { return ReportId::InOutI2cReport34; }
    if length <= 0x38 { return ReportId::InOutI2cReport38; }
    if length <= 0x3C { return ReportId::InOutI2cReport3C; }
    else { return ReportId::InOutI2cReportOverflow; }
}

const I2C_PAYLOAD_SIZE_MAX: usize = 0x3C;

fn decide_i2c_payload_size(id: ReportId) -> usize {
  match id {
    ReportId::InOutI2cReport04 => 0x04,
    ReportId::InOutI2cReport08 => 0x08,
    ReportId::InOutI2cReport0C => 0x0C,
    ReportId::InOutI2cReport10 => 0x10,
    ReportId::InOutI2cReport14 => 0x14,
    ReportId::InOutI2cReport18 => 0x18,
    ReportId::InOutI2cReport1C => 0x1C,
    ReportId::InOutI2cReport20 => 0x20,
    ReportId::InOutI2cReport24 => 0x24,
    ReportId::InOutI2cReport28 => 0x28,
    ReportId::InOutI2cReport2C => 0x2C,
    ReportId::InOutI2cReport30 => 0x30,
    ReportId::InOutI2cReport34 => 0x34,
    ReportId::InOutI2cReport38 => 0x38,
    ReportId::InOutI2cReport3C => 0x3C,
    _ => 0usize
  }
}

fn i2c_write_request(device: &Device, report_id: ReportId,
    slave_addr: u8, flag: I2cCondition, 
    length: usize, src: &[u8], src_index: usize) -> Ft260Result<()>{
  let mut buf = [0u8; 64];
  buf[0] = report_id as u8;
  buf[1] = slave_addr;
  buf[2] = flag.bits();
  buf[3] = length as u8;
  for i in 0..length {
    buf[4 + i] = src[src_index + i];
  }
  device.write_output(&buf)
}

fn i2c_read_request(device: &Device,
    slave_addr: u8, flag: I2cCondition, length: usize) -> Ft260Result<()> {
  let mut buf = [0u8; 64];
  buf[0] = ReportId::OutI2cReadRequest as u8;
  buf[1] = slave_addr;
  buf[2] = flag.bits();
  buf[3] = ((length >> 0) & 0xFF) as u8;
  buf[4] = ((length >> 8) & 0xFF) as u8;
  device.write_output(&buf)
}

pub(crate) fn ft260_i2c_master_read(device: &Device,
    device_address: u8,
    flag: I2cCondition,
    buf: &mut [u8],
    byte_to_read: usize,
    duration_wait: Duration) -> Ft260Result<usize> {

    let res = i2c_read_request(device, device_address, flag, byte_to_read);
    if res.is_err() { return Err(res.err().unwrap()); }
    
    let sz_buf = buf.len();
    let mut idx = 0usize;
    let mut byte_returned = 0usize;
    let time_start = Instant::now();
    while byte_returned < byte_to_read {
      if get_input_reports_count_i2c(device) == 0 {
        // check timeout 
        if (Instant::now() - time_start) >= duration_wait { break; }
        else { continue; }
      }
      if let Some(data) = pop_input_report_i2c(device) {
        let len = data.len();
        if len < 2 { continue; } // error
        if let Ok(rep_id) = data[0].try_into() {
            let sz_from_id = decide_i2c_payload_size(rep_id);
            if sz_from_id == 0 || sz_from_id > len - 2 { continue; } // error
            let len_in_rep = data[1] as usize;
            if len_in_rep > sz_from_id { continue; } // error
            let sz_cpy = if len_in_rep < sz_buf - idx { len_in_rep } else {sz_buf - idx};
            for i in 0..sz_cpy { buf[idx + i] = data[2 + i]; }
            byte_returned += sz_cpy;
            idx += sz_cpy;
        } else {
            return Err(Ft260Error::HidError { 
                message: format!("Unknown Report ID {} detected", data[0]) })
        }

      }
    }
    Ok(byte_returned)
}

pub(crate) fn ft260_i2c_master_write(device: &Device,
    device_address: u8,
    flag: I2cCondition,
    buf: &[u8],
    byte_to_write: usize) -> Ft260Result<usize>
{
    let mut byte_written = 0usize;
    let mut byte_remained = byte_to_write;

    let mut start = flag.contains(I2cCondition::Start);
    let mut restart = flag.contains(I2cCondition::ReStart);
    let mut stop = flag.contains(I2cCondition::Stop);

    loop {
        let mut fval = I2cCondition::None;
        if start || restart {
            fval.set(I2cCondition::Start, true);
            start = false;
            restart = false;
        }
        let size_write = if byte_remained > I2C_PAYLOAD_SIZE_MAX {
            I2C_PAYLOAD_SIZE_MAX
        } else {
            if stop {
                fval.set(I2cCondition::Stop, true);
            }
            byte_remained
        };
        let report_id = decide_i2c_report_id(size_write);
        let mut slice = [0u8; 64];
        for i in 0..size_write { slice[i] = buf[byte_written + i]; }
        byte_remained -= size_write;
        let res = i2c_write_request(device, report_id, device_address, fval, size_write, &slice, 0);
        if let Ok(_) = res {
            byte_written += size_write;
            if byte_remained <= 0 {
                return Ok(byte_written);
            } else {
                continue;
            }
        } else {
            return Err(res.err().unwrap());
        }
    }
}

pub(crate) fn ft260_i2c_master_get_status(device: &Device) -> Ft260Result<I2cBusStatus> {
    let mut buf = [0u8; 64];
    buf[0] = ReportId::FeatI2cStatus as u8;
    let res = device.get_feature(&mut buf);
    if let Ok(sz) = res {
        if sz > 2 && buf[0] == (ReportId::FeatI2cStatus as u8) {
            Ok(I2cBusStatus::from_bits(buf[1]).unwrap())
        } else {
            Err(Ft260Error::HidError { message: "HID Feature I2C Status was not returned".to_string() })
        }
    } else {
        Err(res.err().unwrap())
    }
}

pub(crate) fn ft260_i2c_master_reset(device: &Device) -> Ft260Result<()> {
    ft260_set_request(device, Request::ResetI2c)
}


pub(crate) fn ft260_uart_init(device: &Device) -> Ft260Result<()> {
    const BAUD_DEFAULT: u32 = 115200;
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatSystemSetting as u8;
    buf[1] = Request::ConfigureUart as u8;
    buf[2] = UartEnableMode::DtrDsr as u8;
    buf[3] = ((BAUD_DEFAULT >> 0) & 0xFF) as u8;
    buf[4] = ((BAUD_DEFAULT >> 8) & 0xFF) as u8;
    buf[5] = ((BAUD_DEFAULT >> 16) & 0xFF) as u8;
    buf[6] = ((BAUD_DEFAULT >> 24) & 0xFF) as u8;
    buf[7] = UartDataBits::Eight as u8;
    buf[8] = UartParity::None as u8;
    buf[9] = UartStopBit::One as u8;
    buf[10] = UartBreaking::NoBreak as u8;
    device.set_feature(&buf)
}

pub(crate) fn ft260_uart_set_baud_rate(device: &Device, baud_rate: u32) -> Ft260Result<()> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatSystemSetting as u8;
    buf[1] = Request::SetUartBaudRate as u8;
    buf[2] = ((baud_rate >> 0) & 0xFF) as u8;
    buf[3] = ((baud_rate >> 8) & 0xFF) as u8;
    buf[4] = ((baud_rate >> 16) & 0xFF) as u8;
    buf[5] = ((baud_rate >> 24) & 0xFF) as u8;
    device.set_feature(&buf)
}

pub(crate) fn ft260_uart_set_flow_control(device: &Device, flow_control: UartEnableMode) -> Ft260Result<()> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatSystemSetting as u8;
    buf[1] = Request::SetUartMode as u8;
    buf[2] = flow_control as u8;
    device.set_feature(&buf)
}

pub(crate) fn t260_uart_set_data_bits(device: &Device, data_bits: UartDataBits) -> Ft260Result<()> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatSystemSetting as u8;
    buf[1] = Request::SetUartDataBits as u8;
    buf[2] = data_bits as u8;
    device.set_feature(&buf)
}
pub(crate) fn ft260_uart_set_stop_bit(device: &Device, stop_bit: UartStopBit) -> Ft260Result<()> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatSystemSetting as u8;
    buf[1] = Request::SetUartStopBit as u8;
    buf[2] = stop_bit as u8;
    device.set_feature(&buf)
}
pub(crate) fn ft260_uart_set_parity(device: &Device, parity: UartParity) -> Ft260Result<()> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatSystemSetting as u8;
    buf[1] = Request::SetUartParity as u8;
    buf[2] = parity as u8;
    device.set_feature(&buf)
}

pub(crate) fn ft260_uart_set_break_on(device: &Device, breaking: UartBreaking) -> Ft260Result<()> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatSystemSetting as u8;
    buf[1] = Request::SetUartBreaking as u8;
    buf[2] = breaking as u8;
    device.set_feature(&buf)
}

pub(crate) fn ft260_uart_set_xon_xoff_char(device: &Device, x_on: u8, x_off: u8) -> Ft260Result<()> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatSystemSetting as u8;
    buf[1] = Request::SetUartXonXoff as u8;
    buf[2] = x_on;
    buf[2] = x_off;
    device.set_feature(&buf)
}

pub(crate) struct UartConfig
{
    pub(crate) mode: UartEnableMode,
    pub(crate) baud_rate: u32,
    pub(crate) data_bits: UartDataBits,
    pub(crate) parity: UartParity,
    pub(crate) stop_bit: UartStopBit,
    pub(crate) breaking: UartBreaking,
}

pub(crate) fn ft260_uart_get_config(device: &Device) -> Ft260Result<UartConfig> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatUartStatus as u8;
    if let Err(e) = device.get_feature(&mut buf) {
        return Err(e);
    }
    let mode = UartEnableMode::try_from(buf[1]);
    let data_bits = UartDataBits::try_from(buf[5]);
    let parity = UartParity::try_from(buf[6]);
    let stop_bit = UartStopBit::try_from(buf[7]);
    let breaking = UartBreaking::try_from(buf[8]);
    if let Err(e) = mode { return Err(e); }
    if let Err(e) = data_bits { return Err(e); }
    if let Err(e) = parity { return Err(e); }
    if let Err(e) = stop_bit { return Err(e); }
    if let Err(e) = breaking { return Err(e); }

    let mode = mode.unwrap();
    let baud_rate = ((buf[5] as u32) << 24) | ((buf[4] as u32) << 16) | ((buf[3] as u32) << 8) | ((buf[2] as u32) << 0);
    let data_bits = data_bits.unwrap();
    let parity = parity.unwrap();
    let stop_bit = stop_bit.unwrap();
    let breaking = breaking.unwrap();

    Ok(UartConfig {mode, baud_rate, data_bits, parity, stop_bit, breaking})
}

pub(crate) fn ft260_uart_get_queue_status(device: &Device) -> Ft260Result<usize> {
    Ok(get_input_report_byte_amount_uart(device))
}

fn decide_uart_report_id(length: usize) -> ReportId {
    if length <= 0x04 { return ReportId::InOutUartReport04; }
    if length <= 0x08 { return ReportId::InOutUartReport08; }
    if length <= 0x0C { return ReportId::InOutUartReport0C; }
    if length <= 0x10 { return ReportId::InOutUartReport10; }
    if length <= 0x14 { return ReportId::InOutUartReport14; }
    if length <= 0x18 { return ReportId::InOutUartReport18; }
    if length <= 0x1C { return ReportId::InOutUartReport1C; }
    if length <= 0x20 { return ReportId::InOutUartReport20; }
    if length <= 0x24 { return ReportId::InOutUartReport24; }
    if length <= 0x28 { return ReportId::InOutUartReport28; }
    if length <= 0x2C { return ReportId::InOutUartReport2C; }
    if length <= 0x30 { return ReportId::InOutUartReport30; }
    if length <= 0x34 { return ReportId::InOutUartReport34; }
    if length <= 0x38 { return ReportId::InOutUartReport38; }
    if length <= 0x3C { return ReportId::InOutUartReport3C; }
    else { return ReportId::InOutUartReportOverflow; }
}

const UART_PAYLOAD_SIZE_MAX: usize = 0x3C;

fn decide_uart_payload_size(id: ReportId) -> usize {
    match id {
        ReportId::InOutUartReport04 => 0x04,
        ReportId::InOutUartReport08 => 0x08,
        ReportId::InOutUartReport0C => 0x0C,
        ReportId::InOutUartReport10 => 0x10,
        ReportId::InOutUartReport14 => 0x14,
        ReportId::InOutUartReport18 => 0x18,
        ReportId::InOutUartReport1C => 0x1C,
        ReportId::InOutUartReport20 => 0x20,
        ReportId::InOutUartReport24 => 0x24,
        ReportId::InOutUartReport28 => 0x28,
        ReportId::InOutUartReport2C => 0x2C,
        ReportId::InOutUartReport30 => 0x30,
        ReportId::InOutUartReport34 => 0x34,
        ReportId::InOutUartReport38 => 0x38,
        ReportId::InOutUartReport3C => 0x3C,
        _ => 0x00,
    }
}

fn uart_write_request(device: &Device, report_id: ReportId, src: &[u8], len_src: usize) -> Ft260Result<()> {
    let mut buf = [0u8; 64];
    buf[0] = report_id as u8;
    buf[1] = len_src as u8;
    for i in 0..len_src {
        buf[2+i] = src[i];
    }
    device.write_output(&buf)
}

pub(crate) fn ft260_uart_read(device: &Device,
    buf: &mut [u8],
    byte_to_read: usize,
    duration_wait: Duration) -> Ft260Result<usize> {
        
    let sz_buf = buf.len();
    let mut idx = 0usize;
    let mut byte_returned = 0usize;
    let time_start = Instant::now();
    while byte_returned < byte_to_read {
        if get_input_reports_count_uart(device) == 0 {
            // check timeout 
            if (Instant::now() - time_start) >= duration_wait { break; }
            else { continue; }
        }

        if let Some(data) = pop_input_report_uart(device) {
            let len = data.len();
            if len < 2 { continue; } // error
            if let Ok(rep_id) = data[0].try_into() {
                let sz_from_id = decide_uart_payload_size(rep_id);
                if sz_from_id == 0 || sz_from_id > len - 2 { continue; } // error
                let len_in_rep = data[1] as usize;
                if len_in_rep > sz_from_id { continue; } // error
                let sz_cpy = if len_in_rep < sz_buf - idx { len_in_rep } else {sz_buf - idx};
                for i in 0..sz_cpy { buf[idx + i] = data[2 + i]; }
                byte_returned += sz_cpy;
                idx += sz_cpy;
            } else {
                return Err(Ft260Error::HidError { 
                    message: format!("Unknown Report ID {} detected", data[0]) })
            }
        }
    }
    Ok(byte_returned)
}

pub(crate) fn ft260_uart_write(device: &Device,
    buf: &[u8],
    byte_to_write: usize) -> Ft260Result<usize> {
    let mut byte_written = 0usize;
    let mut byte_remained = byte_to_write;
    
    loop {
        let size_write = if byte_remained > UART_PAYLOAD_SIZE_MAX { UART_PAYLOAD_SIZE_MAX } else { byte_remained};
        let rid = decide_uart_report_id(size_write);
        let mut slice = [0u8; 64];
        for i in 0..size_write { slice[i] = buf[byte_written + i]; }
        byte_remained -= size_write;
        let res = uart_write_request(device, rid, &slice, size_write);
        if let Ok(_) = res {
            byte_written += size_write;
            if byte_remained <= 0 {
                return Ok(byte_written);
            } else {
                continue;
            }
        } else {
            return Err(res.err().unwrap());
        }
    }
}

pub(crate) fn ft260_uart_reset(device: &Device) -> Ft260Result<()> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatSystemSetting as u8;
    buf[1] = Request::ResetUart as u8;
    device.set_feature(&buf)
}

pub(crate) fn ft260_uart_get_dcd_ri_status(device: &Device) -> Ft260Result<UartDcdRiStatus> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatUartRiAndDcdStatus as u8;
    if let Err(e) = device.get_feature(&mut buf) {
        return Err(e);
    }
    if let Some(s) = UartDcdRiStatus::from_bits(buf[1]) {
        Ok(s)
    } else {
        Err(Ft260Error::ByteError { value: buf[1], message: "Couldn't get valid `UartDcdRiStatus` value".to_string() })
    }
}

pub(crate) fn ft260_uart_enable_ri_wakeup(device: &Device, enable: WakeupIntEnableMode) -> Ft260Result<()> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatSystemSetting as u8;
    buf[1] = Request::EnableUartRiWaveup as u8;
    buf[2] = enable as u8;
    device.set_feature(&buf)
}

pub(crate) fn ft260_uart_set_ri_wakeup_config(device: &Device, config: UartRiWakeupConfig) -> Ft260Result<()> {
    let mut buf = [0u8, 64];
    buf[0] = ReportId::FeatSystemSetting as u8;
    buf[1] = Request::SetUartRiWakeupConfig as u8;
    buf[2] = config as u8;
    device.set_feature(&buf)
}

/*

pub(crate) bool InterruptFlag = false;

pub(crate) FT260_STATUS FT260_GetInterruptFlag(device: &Device, ref pbFlag: bool)
{
    do
    {   // read all queued Interrupt Status report
        let data = PopInputReportInt();
        if (data == null) { break; }
        if (data.Length >= 3 &&
            data[0] == (byte)ReportId::InInterruptStatus &&
            (data[1] & 1) != 0)
        {
            InterruptFlag = true;
        }
    } while (true);
    pbFlag = InterruptFlag;
    return FT260_STATUS.OK;
}

pub(crate) FT260_STATUS FT260_CleanInterruptFlag(device: &Device, ref pbFlag: bool)
{
    let res = FT260_GetInterruptFlag(hid, ref pbFlag);
    InterruptFlag = false; // clear
    return res;
}


// */

#[derive(Clone, Copy)]
struct GpioRequest
{
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
fn ft260_gpio_set(device: &Device, report: GpioRequest) -> Ft260Result<()> {
    ft260_set_feature(device, &[
        ReportId::FeatGpio as u8, report.val.bits(), report.dir.bits(), report.ex_val.bits(), report.ex_dir.bits()
    ])
}

/// 4.7.2 GPIO Read Request
fn ft260_gpio_get(device: &Device) -> Ft260Result<GpioRequest> {
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
    } else{
        Err(res.err().unwrap())
    }
}

fn ft260_gpio_pin_to_bits(pin: GpioPinNum) -> (GpioBitVal, GpioExBitVal) {
    let mut bits: GpioBitVal = GpioBitVal::None;
    if pin.contains(GpioPinNum::GPIO_0) { bits |= GpioBitVal::_0; }
    if pin.contains(GpioPinNum::GPIO_1) { bits |= GpioBitVal::_1; }
    if pin.contains(GpioPinNum::GPIO_2) { bits |= GpioBitVal::_2; }
    if pin.contains(GpioPinNum::GPIO_3) { bits |= GpioBitVal::_3; }
    if pin.contains(GpioPinNum::GPIO_4) { bits |= GpioBitVal::_4; }
    if pin.contains(GpioPinNum::GPIO_5) { bits |= GpioBitVal::_5; }
    let mut ex_bits: GpioExBitVal = GpioExBitVal::None;
    if pin.contains(GpioPinNum::GPIO_A) { ex_bits |= GpioExBitVal::_A; }
    if pin.contains(GpioPinNum::GPIO_B) { ex_bits |= GpioExBitVal::_B; }
    if pin.contains(GpioPinNum::GPIO_C) { ex_bits |= GpioExBitVal::_C; }
    if pin.contains(GpioPinNum::GPIO_D) { ex_bits |= GpioExBitVal::_D; }
    if pin.contains(GpioPinNum::GPIO_E) { ex_bits |= GpioExBitVal::_E; }
    if pin.contains(GpioPinNum::GPIO_F) { ex_bits |= GpioExBitVal::_F; }
    if pin.contains(GpioPinNum::GPIO_G) { ex_bits |= GpioExBitVal::_G; }
    if pin.contains(GpioPinNum::GPIO_H) { ex_bits |= GpioExBitVal::_H; }

    (bits, ex_bits)
}

pub(crate) fn ft260_gpio_set_dir(device: &Device, pin: GpioPinNum,  dir: GpioDir) -> Ft260Result<()> {
    let res = ft260_gpio_get(device);
    if let Ok(req) = res {
        let mut req = req;
        let (bit, ex_bit) = ft260_gpio_pin_to_bits(pin);
        // 0: input, 1: output
        let bit_dir = match dir {
            GpioDir::In => false,
            GpioDir::Out => true,
        };
        req.dir.set(bit, bit_dir);
        req.ex_dir.set(ex_bit, bit_dir);
        ft260_gpio_set(device, req)
    } else {
        Err(res.err().unwrap())
    }
}

pub(crate) fn ft260_gpio_read(device: &Device, pin: GpioPinNum) -> Ft260Result<GpioValue> {
    let res = ft260_gpio_get(device);
    if let Ok(req) = res {
        let (bit, ex_bit) = ft260_gpio_pin_to_bits(pin);
        if req.val.intersects(bit) || req.ex_val.intersects(ex_bit) {
            Ok(GpioValue::High)
        } else {
            Ok(GpioValue::Low)
        }
    } else {
        Err(res.err().unwrap())
    }
}

pub(crate) fn ft260_gpio_write(device: &Device, pin: GpioPinNum, val: GpioValue) -> Ft260Result<()> {
    let res = ft260_gpio_get(device);
    if let Ok(req) = res {
        let mut req = req;
        let (bit, ex_bit) = ft260_gpio_pin_to_bits(pin);
        let bit_val = match val {
            GpioValue::Low => false,
            GpioValue::High => true,
        };
        req.val.set(bit, bit_val);
        req.ex_val.set(ex_bit, bit_val);
        ft260_gpio_set(device, req)
    } else {
        Err(res.err().unwrap())
    }
}
