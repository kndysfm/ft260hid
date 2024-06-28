use std::time::{Duration, Instant};

use crate::device::Device;
use crate::hid::consts::*;
use crate::hid::reports::*;
use crate::{Ft260Error, Ft260Result};

pub(crate) fn init(device: &Device, kbps: u16) -> Ft260Result<()> {
    ft260_set_request_u16(device, Request::SetI2cClockSpeed, kbps)
}

fn decide_i2c_report_id(length: usize) -> ReportId {
    match length {
        0x00..=0x04 => ReportId::InOutI2cReport04,
        0x05..=0x08 => ReportId::InOutI2cReport08,
        0x09..=0x0C => ReportId::InOutI2cReport0C,
        0x0D..=0x10 => ReportId::InOutI2cReport10,
        0x11..=0x14 => ReportId::InOutI2cReport14,
        0x15..=0x18 => ReportId::InOutI2cReport18,
        0x19..=0x1C => ReportId::InOutI2cReport1C,
        0x1D..=0x20 => ReportId::InOutI2cReport20,
        0x21..=0x24 => ReportId::InOutI2cReport24,
        0x25..=0x28 => ReportId::InOutI2cReport28,
        0x29..=0x2C => ReportId::InOutI2cReport2C,
        0x2D..=0x30 => ReportId::InOutI2cReport30,
        0x31..=0x34 => ReportId::InOutI2cReport34,
        0x35..=0x38 => ReportId::InOutI2cReport38,
        0x39..=0x3C => ReportId::InOutI2cReport3C,
        _ => ReportId::InOutI2cReportOverflow,
    }
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
        _ => 0usize,
    }
}

fn i2c_write_request(
    device: &Device,
    report_id: ReportId,
    slave_addr: u8,
    flag: I2cCondition,
    length: usize,
    src: &[u8],
    src_index: usize,
) -> Ft260Result<()> {
    let mut buf = feat_rep_buf();
    buf[0] = report_id as u8;
    buf[1] = slave_addr;
    buf[2] = flag.bits();
    buf[3] = length as u8;

    buf[4..(length + 4)].copy_from_slice(&src[src_index..(length + src_index)]);

    device.write_output(&buf)
}

fn i2c_read_request(
    device: &Device,
    slave_addr: u8,
    flag: I2cCondition,
    length: usize,
) -> Ft260Result<()> {
    let mut buf = feat_rep_buf();
    buf[0] = ReportId::OutI2cReadRequest as u8;
    buf[1] = slave_addr;
    buf[2] = flag.bits();

    // Split the length into bytes and store them in the buffer
    buf[3..5].copy_from_slice(&(length as u16).to_le_bytes());

    device.write_output(&buf)
}

pub(crate) fn read(
    device: &Device,
    device_address: u8,
    flag: I2cCondition,
    buf: &mut [u8],
    byte_to_read: usize,
    duration_wait: Duration,
) -> Ft260Result<usize> {
    let res = i2c_read_request(device, device_address, flag, byte_to_read);
    if res.is_err() {
        return Err(res.err().unwrap());
    }

    let sz_buf = buf.len();
    let mut idx = 0usize;
    let mut byte_returned = 0usize;
    let time_start = Instant::now();
    while byte_returned < byte_to_read {
        if get_input_reports_count_i2c(device) == 0 {
            // check timeout
            if (Instant::now() - time_start) >= duration_wait {
                break;
            } else {
                continue;
            }
        }
        if let Some(data) = pop_input_report_i2c(device) {
            let len = data.len();
            if len < 2 {
                continue;
            } // error
            if let Ok(rep_id) = data[0].try_into() {
                let sz_from_id = decide_i2c_payload_size(rep_id);
                if sz_from_id == 0 || sz_from_id > len - 2 {
                    continue;
                } // error
                let len_in_rep = data[1] as usize;
                if len_in_rep > sz_from_id {
                    continue;
                } // error
                let sz_cpy = if len_in_rep < sz_buf - idx {
                    len_in_rep
                } else {
                    sz_buf - idx
                };

                buf[idx..(sz_cpy + idx)].copy_from_slice(&data[2..(sz_cpy + 2)]);

                byte_returned += sz_cpy;
                idx += sz_cpy;
            } else {
                return Err(Ft260Error::HidError {
                    message: format!("Unknown Report ID {} detected", data[0]),
                });
            }
        }
    }
    Ok(byte_returned)
}

pub(crate) fn write(
    device: &Device,
    device_address: u8,
    flag: I2cCondition,
    buf: &[u8],
    byte_to_write: usize,
) -> Ft260Result<usize> {
    let mut byte_written = 0usize;
    let mut byte_remained = byte_to_write;

    let mut start = flag.contains(I2cCondition::Start);
    let mut restart = flag.contains(I2cCondition::ReStart);
    let stop = flag.contains(I2cCondition::Stop);

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
        let mut slice = feat_rep_buf();

        slice[..size_write].copy_from_slice(&buf[byte_written..(size_write + byte_written)]);

        byte_remained -= size_write;
        let res = i2c_write_request(
            device,
            report_id,
            device_address,
            fval,
            size_write,
            &slice,
            0,
        );
        if res.is_ok() {
            byte_written += size_write;
            if byte_remained == 0 {
                return Ok(byte_written);
            } else {
                continue;
            }
        } else {
            return Err(res.err().unwrap());
        }
    }
}

pub(crate) fn get_status(device: &Device) -> Ft260Result<I2cBusStatus> {
    let mut buf = feat_rep_buf();
    buf[0] = ReportId::FeatI2cStatus as u8;
    let res = device.get_feature(&mut buf);
    if let Ok(sz) = res {
        if sz > 2 && buf[0] == (ReportId::FeatI2cStatus as u8) {
            Ok(I2cBusStatus::from_bits(buf[1]).unwrap())
        } else {
            Err(Ft260Error::HidError {
                message: "HID Feature I2C Status was not returned".to_string(),
            })
        }
    } else {
        Err(res.err().unwrap())
    }
}

pub(crate) fn reset(device: &Device) -> Ft260Result<()> {
    ft260_set_request(device, Request::ResetI2c)
}
