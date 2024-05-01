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
    wait_timer: u32) -> Ft260Result<usize> {

    let res = i2c_read_request(device, device_address, flag, byte_to_read);
    if res.is_err() { return Err(res.err().unwrap()); }
    
    let sz_buf = buf.len();
    let mut idx = 0usize;
    let mut byte_returned = 0usize;
    while byte_returned < byte_to_read {
      if get_input_reports_count_i2c(device) == 0 {
        //TODO: check timeout 
        continue;
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

pub(crate) fn ft260_i2c_master_get_status(device: &Device) -> Ft260Result<u8> {
    let mut buf = [0u8; 64];
    buf[0] = ReportId::FeatI2cStatus as u8;
    let res = device.get_feature(&mut buf);
    if let Ok(sz) = res {
        if sz > 2 && buf[0] == (ReportId::FeatI2cStatus as u8) {
            Ok(buf[1])
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

/* //

pub(crate) FT260_STATUS FT260_UART_Init(device: &Device) -> Ft260Result<()> {}
{
    const uint BaudDefault = 115200;
    return setFeature(hid, (byte)ReportId::FeatSystemSetting,
        (byte)Request::ConfigureUart,
        (byte)UartEnableMode.DtrDsr,
        (byte)((BaudDefault >> 0) & 0xFF),
        (byte)((BaudDefault >> 8) & 0xFF),
        (byte)((BaudDefault >> 16) & 0xFF),
        (byte)((BaudDefault >> 24) & 0xFF),
        (byte)UartDataBits.Eight,
        (byte)UartParity.None,
        (byte)UartStopBit.One,
        (byte)UartBreaking.NoBreak);
}

pub(crate) fn FT260_UART_SetBaudRate(device: &Device, baudRate: uint) -> Ft260Result<()> {}
{
    return setFeature(hid, (byte)ReportId::FeatSystemSetting,
        (byte)Request::SetUartBaudRate,
        (byte)((baudRate >> 0) & 0xFF),
        (byte)((baudRate >> 8) & 0xFF),
        (byte)((baudRate >> 16) & 0xFF),
        (byte)((baudRate >> 24) & 0xFF));
}

pub(crate) fn FT260_UART_SetFlowControl(device: &Device, flowControl: UartEnableMode) -> Ft260Result<()> {}
{
    return setFeature(hid, (byte)ReportId::FeatSystemSetting,
        (byte)Request::SetUartMode, (byte)flowControl);
}

pub(crate) FT260_STATUS FT260_UART_SetDataCharacteristics(device: &Device,
    dataBits: UartDataBits
    stopBits: UartStopBit
    parity: UartParity)
{
    let res = setFeature(hid, (byte)ReportId::FeatSystemSetting,
        (byte)Request::SetUartDataBit, (byte)dataBits);
    if (res != FT260_STATUS.OK) return res;
    res = setFeature(hid, (byte)ReportId::FeatSystemSetting,
        (byte)Request::SetUartStopBit, (byte)stopBits);
    if (res != FT260_STATUS.OK) return res;
    return setFeature(hid, (byte)ReportId::FeatSystemSetting,
                    (byte)Request::SetUartParity, (byte)parity);
}

pub(crate) FT260_STATUS FT260_UART_SetBreakOn(device: &Device) -> Ft260Result<()> {}
{
    return setFeature(hid, (byte)ReportId::FeatSystemSetting,
        (byte)Request::SetUartBreaking, (byte)UartBreaking.Break);
}

pub(crate) FT260_STATUS FT260_UART_SetBreakOff(device: &Device) -> Ft260Result<()> {}
{
    return setFeature(hid, (byte)ReportId::FeatSystemSetting,
        (byte)Request::SetUartBreaking, (byte)UartBreaking.NoBreak);
}

pub(crate) FT260_STATUS FT260_UART_SetXonXoffChar(device: &Device,
    Xon: u8, Xoff: u8)
{
    return setFeature(hid, (byte)ReportId::FeatSystemSetting,
        (byte)Request::SetUartXonXoff, Xon, Xoff);
}

pub(crate) struct UartConfig
{
    pub(crate) UartEnableMode Mode;
    pub(crate) uint BaudRate;
    pub(crate) UartDataBits DataBits;
    pub(crate) UartParity Parity;
    pub(crate) UartStopBit StopBit;
    pub(crate) UartBreaking Breaking;
}

pub(crate) FT260_STATUS FT260_UART_GetConfig(device: &Device, out pUartConfig: UartConfig)
{
    pUartConfig = new UartConfig();
    if (getFeature(hid, ReportId::FeatUartStatus, out &[u8] dat) &&
        dat[0] == (byte)ReportId::FeatUartStatus)
    {
        pUartConfig.Mode = (UartEnableMode)dat[1];
        pUartConfig.BaudRate = (uint)((dat[5] << 24) | (dat[4] << 16) | (dat[3] << 8) | (dat[2] << 0));
        pUartConfig.DataBits = (UartDataBits)dat[6];
        pUartConfig.Parity = (UartParity)dat[7];
        pUartConfig.StopBit = (UartStopBit)dat[8];
        pUartConfig.Breaking = (UartBreaking)dat[9];
        return FT260_STATUS.OK;
    }
    return FT260_STATUS.OTHER_ERROR;
}

pub(crate) FT260_STATUS FT260_UART_GetQueueStatus(device: &Device,
    out lpdwAmountInRxQueue: uint)
{
    Debug.Assert(IsAsyncReadMode, "UART requires asynchronous read of HidDevice.");
    lpdwAmountInRxQueue = (uint)GetInputReportByteAmountUart();
    return FT260_STATUS.OK;
}

ReportId DecideUartReportId(length: u8)
{
    if (length <= 0x04) return ReportId::InOutUartReport04;
    if (length <= 0x08) return ReportId::InOutUartReport08;
    if (length <= 0x0C) return ReportId::InOutUartReport0C;
    if (length <= 0x10) return ReportId::InOutUartReport10;
    if (length <= 0x14) return ReportId::InOutUartReport14;
    if (length <= 0x18) return ReportId::InOutUartReport18;
    if (length <= 0x1C) return ReportId::InOutUartReport1C;
    if (length <= 0x20) return ReportId::InOutUartReport20;
    if (length <= 0x24) return ReportId::InOutUartReport24;
    if (length <= 0x28) return ReportId::InOutUartReport28;
    if (length <= 0x2C) return ReportId::InOutUartReport2C;
    if (length <= 0x30) return ReportId::InOutUartReport30;
    if (length <= 0x34) return ReportId::InOutUartReport34;
    if (length <= 0x38) return ReportId::InOutUartReport38;
    if (length <= 0x3C) return ReportId::InOutUartReport3C;
    else return ReportId::InOutI2cReportOverflow;
}

const byte UartPayloadSizeMax = 0x3C;

byte DecideUartPayloadSize(id: ReportId)
{
    switch (id)
    {
        case ReportId::InOutUartReport04: return 0x04;
        case ReportId::InOutUartReport08: return 0x08;
        case ReportId::InOutUartReport0C: return 0x0C;
        case ReportId::InOutUartReport10: return 0x10;
        case ReportId::InOutUartReport14: return 0x14;
        case ReportId::InOutUartReport18: return 0x18;
        case ReportId::InOutUartReport1C: return 0x1C;
        case ReportId::InOutUartReport20: return 0x20;
        case ReportId::InOutUartReport24: return 0x24;
        case ReportId::InOutUartReport28: return 0x28;
        case ReportId::InOutUartReport2C: return 0x2C;
        case ReportId::InOutUartReport30: return 0x30;
        case ReportId::InOutUartReport34: return 0x34;
        case ReportId::InOutUartReport38: return 0x38;
        case ReportId::InOutUartReport3C: return 0x3C;
        default: return 0;
    }
}

bool UartWriteRequest(device: &Device, reportId: ReportId, length: u8, data: &[u8])
{
    Debug.Assert(length <= data.Length);

    let buf = new byte[length + 2];
    buf[0] = (byte)reportId;
    buf[1] = length;
    Array.Copy(data, 0, buf, 2, length);
    return hid.Write(buf);
}

pub(crate) FT260_STATUS FT260_UART_Read(device: &Device,
    buf: &[u8],
    dwBufferLength: uint,
    byte_to_read: uint,
    out lpdwBytesReturned: uint)
{
    Debug.Assert(IsAsyncReadMode, "UART requires asynchronous read of HidDevice.");
    Debug.Assert(buf.Length >= byte_to_read);

    lpdwBytesReturned = 0;
    int idx = 0;
    while (lpdwBytesReturned < byte_to_read)
    {
        if (GetInputReportsCountUart() == 0)
        {
            break;
        }

        let data = PopInputReportUart();
        if (data.Length < 2)
        {
            Debug.WriteLine($"[{GetCurrentMethod().Name}] Input report is too short (length = {data.Length})");
            return FT260_STATUS.OTHER_ERROR;
        }
        let size = DecideUartPayloadSize((ReportId)data[0]);
        if (size == 0 || size > data.Length - 2)
        { // other report ID, ignore it
            Debug.WriteLine($"[{GetCurrentMethod().Name}] Wrong report ID 0x{data[0]:X2}");
            return FT260_STATUS.OTHER_ERROR;
        }
        let len = data[1];
        if (data.Length < len + 2)
        {
            Debug.WriteLine($"[{GetCurrentMethod().Name}] Input report is too short (decl = {len}, actual = {data.Length - 2})");
            return FT260_STATUS.OTHER_ERROR;
        }
        let sz = Math.Min(buf.Length - idx, len);
        Array.Copy(data, 2, buf, idx, sz);
        lpdwBytesReturned += (uint)sz;
        idx += sz;
    }

    return FT260_STATUS.OK;
}

pub(crate) FT260_STATUS FT260_UART_Write(device: &Device,
    buf: &[u8],
    dwBufferLength: uint,
    byte_to_write: uint,
    out lpdwBytesWritten: uint)
{
    Debug.Assert(buf.Length >= byte_to_write);
    Debug.Assert(buf.Length >= dwBufferLength);

    uint written = 0;
    uint remained = byte_to_write;

    while (remained > 0)
    {
        byte size;
        if (remained > UartPayloadSizeMax)
        {
            size = UartPayloadSizeMax;
        }
        else
        {   // it's the last output report
            size = (byte)remained;
        }
        let rid = DecideUartReportId(size);
        Debug.Assert(0xF0 <= (byte)rid && (byte)rid <= 0xFE);
        let slice = new byte[size];
        Array.Copy(buf, written, slice, 0, size);
        remained -= size;
        let res = UartWriteRequest(hid, rid, size, slice);
        Thread.Yield();
        if (!res)
        {
            Debug.WriteLine($"[{GetCurrentMethod().Name}] Fail to write HID output report");
            lpdwBytesWritten = written;
            return FT260_STATUS.IO_ERROR;
        }
        else
        {
            Debug.WriteLine($"[{GetCurrentMethod().Name}] Succeed to write {size} bytes payload");
        }
        written += size;
    }

    lpdwBytesWritten = written;
    return FT260_STATUS.OK;
}

pub(crate) FT260_STATUS FT260_UART_Reset(device: &Device) -> Ft260Result<()> {}
{
    return setFeature(hid, (byte)ReportId::FeatSystemSetting,
        (byte)Request::ResetUart);
}

pub(crate) FT260_STATUS FT260_UART_GetDcdRiStatus(device: &Device,
    out value: UartDcdRiStatus)
{
    if (getFeature(hid, ReportId::FeatUartRiAndDcdStatus, out &[u8] dat) &&
        dat[0] == (byte)ReportId::FeatUartRiAndDcdStatus)
    {
        value = (UartDcdRiStatus)dat[1];
        return FT260_STATUS.OK;
    }
    value = 0;
    return FT260_STATUS.OTHER_ERROR;
}

pub(crate) fn FT260_UART_EnableRiWakeup(device: &Device, enable: WakeupIntEnableMode) -> Ft260Result<()> {}
{
    return setFeature(hid, (byte)ReportId::FeatSystemSetting,
        (byte)Request::EnableUartRiWaveup, (byte)enable);
}

pub(crate) FT260_STATUS FT260_UART_SetRiWakeupConfig(device: &Device,
    type: UartRiWakeupConfig)
{
    return setFeature(hid, (byte)ReportId::FeatSystemSetting,
        (byte)Request::SetUartRiWakeupConfig, (byte)type);
}




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
