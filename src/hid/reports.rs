use std::time::{Duration, Instant};

use bitflags::Flags;

use crate::device::Device;
use crate::hid::consts::*;
use crate::{Ft260Error, Ft260Result};

pub(crate) type Report = Option<Vec<u8>>;

pub(crate) type FeatureReportBuffer = [u8; 64];

fn clear_input_report_queue_i2c(device: &Device) {
    let id = ReportId::InOutI2cReport04 as u8;
    device.fifo().clear(id);
}

fn pop_input_report_i2c(device: &Device) -> Report {
    let id = ReportId::InOutI2cReport04 as u8;
    device.fifo().pop_report(id)
}

fn get_input_reports_count_i2c(device: &Device) -> usize {
    let id = ReportId::InOutI2cReport04 as u8;
    device.fifo().len(id)
}

fn clear_input_report_queue_uart(device: &Device) {
    let id = ReportId::InOutUartReport04 as u8;
    device.fifo().clear(id);
}

fn pop_input_report_uart(device: &Device) -> Report {
    let id = ReportId::InOutUartReport04 as u8;
    device.fifo().pop_report(id)
}

fn get_input_reports_count_uart(device: &Device) -> usize {
    let id = ReportId::InOutUartReport04 as u8;
    device.fifo().len(id)
}

fn get_input_report_byte_amount_uart(device: &Device) -> usize {
    let id = ReportId::InOutUartReport04 as u8;
    let mut amount = 0usize;
    for rep in device.fifo().iter_peek(id) {
        if rep.len() > 1 {
            amount += rep[1] as usize; // length value
        }
    }
    amount
}

fn pop_input_report_int(device: &Device) -> Report {
    let id = ReportId::InInterruptStatus as u8;
    device.fifo().pop_report(id)
}

fn ft260_set_request(device: &Device, request: Request) -> Ft260Result<()> {
    device.set_feature(&[ReportId::FeatSystemSetting as u8, request as u8])
}

fn ft260_set_request_u8(device: &Device, request: Request, value: u8) -> Ft260Result<()> {
    device.set_feature(&[ReportId::FeatSystemSetting as u8, request as u8, value])
}

fn ft260_set_request_u8x2(
    device: &Device,
    request: Request,
    value1: u8,
    value2: u8,
) -> Ft260Result<()> {
    device.set_feature(&[
        ReportId::FeatSystemSetting as u8,
        request as u8,
        value1,
        value2,
    ])
}

fn ft260_set_request_u16(device: &Device, request: Request, value: u16) -> Ft260Result<()> {
    device.set_feature(&[
        ReportId::FeatSystemSetting as u8,
        request as u8,
        ((value >> 0) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
    ])
}

fn ft260_set_request_u32(device: &Device, request: Request, value: u32) -> Ft260Result<()> {
    device.set_feature(&[
        ReportId::FeatSystemSetting as u8,
        request as u8,
        ((value >> 0) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8,
    ])
}

fn ft260_set_feature(device: &Device, data: &[u8]) -> Ft260Result<()> {
    device.set_feature(data)
}

fn feat_rep_buf() -> FeatureReportBuffer {
    [0u8; 64]
}

fn ft260_get_feature(device: &Device, report_id: u8) -> Ft260Result<FeatureReportBuffer> {
    let mut buf = [0u8; 64];
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

pub(crate) fn ft260_set_wakeup_interrupt(
    device: &Device,
    enable: WakeupIntEnableMode,
) -> Ft260Result<()> {
    ft260_set_request_u8(device, Request::EnableInterruptWakeUp, enable as u8)
}

pub(crate) fn ft260_set_interrupt_trigger_type(
    device: &Device,
    trigger: InterruptTrigger,
    delay: InterruptDuration,
) -> Ft260Result<()> {
    ft260_set_request_u8x2(
        device,
        Request::SetInterruptTriggerCondition,
        trigger as u8,
        delay as u8,
    )
}

pub(crate) fn ft260_get_chip_version(device: &Device) -> Ft260Result<u32> {
    let mut buf = feat_rep_buf();
    buf[0] = ReportId::FeatChipCode as u8;
    let res = device.get_feature(&mut buf);
    if let Ok(sz) = res {
        if sz > 5 {
            let ver = ((buf[1] as u32) << 0)
                | ((buf[2] as u32) << 8)
                | ((buf[3] as u32) << 16)
                | ((buf[4] as u32) << 24);
            Ok(ver)
        } else {
            Err(Ft260Error::HidError {
                message: "Feature Report gotten is short".to_string(),
            })
        }
    } else {
        Err(res.err().unwrap())
    }
}

pub(crate) mod i2c;

pub(crate) mod uart;

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

pub(crate) mod gpio;
