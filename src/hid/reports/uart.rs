use std::time::{Duration, Instant};

use crate::device::Device;
use crate::hid::consts::*;
use crate::{Ft260Error, Ft260Result};
use super::*;

pub(crate) fn init(device: &Device) -> Ft260Result<()> {
  const BAUD_DEFAULT: u32 = 115200;
  let mut buf = feat_rep_buf();
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

pub(crate) fn set_baud_rate(device: &Device, baud_rate: u32) -> Ft260Result<()> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatSystemSetting as u8;
  buf[1] = Request::SetUartBaudRate as u8;
  buf[2] = ((baud_rate >> 0) & 0xFF) as u8;
  buf[3] = ((baud_rate >> 8) & 0xFF) as u8;
  buf[4] = ((baud_rate >> 16) & 0xFF) as u8;
  buf[5] = ((baud_rate >> 24) & 0xFF) as u8;
  device.set_feature(&buf)
}

pub(crate) fn set_flow_control(
  device: &Device,
  flow_control: UartEnableMode,
) -> Ft260Result<()> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatSystemSetting as u8;
  buf[1] = Request::SetUartMode as u8;
  buf[2] = flow_control as u8;
  device.set_feature(&buf)
}

pub(crate) fn set_data_bits(
  device: &Device,
  data_bits: UartDataBits,
) -> Ft260Result<()> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatSystemSetting as u8;
  buf[1] = Request::SetUartDataBits as u8;
  buf[2] = data_bits as u8;
  device.set_feature(&buf)
}
pub(crate) fn set_stop_bit(device: &Device, stop_bit: UartStopBit) -> Ft260Result<()> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatSystemSetting as u8;
  buf[1] = Request::SetUartStopBit as u8;
  buf[2] = stop_bit as u8;
  device.set_feature(&buf)
}
pub(crate) fn set_parity(device: &Device, parity: UartParity) -> Ft260Result<()> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatSystemSetting as u8;
  buf[1] = Request::SetUartParity as u8;
  buf[2] = parity as u8;
  device.set_feature(&buf)
}

pub(crate) fn set_breaking(device: &Device, breaking: UartBreaking) -> Ft260Result<()> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatSystemSetting as u8;
  buf[1] = Request::SetUartBreaking as u8;
  buf[2] = breaking as u8;
  device.set_feature(&buf)
}

pub(crate) fn set_xon_xoff_char(
  device: &Device,
  x_on: u8,
  x_off: u8,
) -> Ft260Result<()> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatSystemSetting as u8;
  buf[1] = Request::SetUartXonXoff as u8;
  buf[2] = x_on;
  buf[2] = x_off;
  device.set_feature(&buf)
}

#[derive(Debug)]
pub(crate) struct Config {
  pub(crate) mode: UartEnableMode,
  pub(crate) baud_rate: u32,
  pub(crate) data_bits: UartDataBits,
  pub(crate) parity: UartParity,
  pub(crate) stop_bit: UartStopBit,
  pub(crate) breaking: UartBreaking,
}

pub(crate) fn get_config(device: &Device) -> Ft260Result<Config> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatUartStatus as u8;
  if let Err(e) = device.get_feature(&mut buf) {
      return Err(e);
  }
  let mode = UartEnableMode::try_from(buf[1]);
  let data_bits = UartDataBits::try_from(buf[6]);
  let parity = UartParity::try_from(buf[7]);
  let stop_bit = UartStopBit::try_from(buf[8]);
  let breaking = UartBreaking::try_from(buf[9]);
  if let Err(e) = mode {
      return Err(e);
  }
  if let Err(e) = data_bits {
      return Err(e);
  }
  if let Err(e) = parity {
      return Err(e);
  }
  if let Err(e) = stop_bit {
      return Err(e);
  }
  if let Err(e) = breaking {
      return Err(e);
  }

  let mode = mode.unwrap();
  let baud_rate = ((buf[5] as u32) << 24)
      | ((buf[4] as u32) << 16)
      | ((buf[3] as u32) << 8)
      | ((buf[2] as u32) << 0);
  let data_bits = data_bits.unwrap();
  let parity = parity.unwrap();
  let stop_bit = stop_bit.unwrap();
  let breaking = breaking.unwrap();

  Ok(Config {
      mode,
      baud_rate,
      data_bits,
      parity,
      stop_bit,
      breaking,
  })
}

pub(crate) fn get_queue_status(device: &Device) -> usize {
  get_input_report_byte_amount_uart(device)
}

fn decide_uart_report_id(length: usize) -> ReportId {
  if length <= 0x04 {
      return ReportId::InOutUartReport04;
  }
  if length <= 0x08 {
      return ReportId::InOutUartReport08;
  }
  if length <= 0x0C {
      return ReportId::InOutUartReport0C;
  }
  if length <= 0x10 {
      return ReportId::InOutUartReport10;
  }
  if length <= 0x14 {
      return ReportId::InOutUartReport14;
  }
  if length <= 0x18 {
      return ReportId::InOutUartReport18;
  }
  if length <= 0x1C {
      return ReportId::InOutUartReport1C;
  }
  if length <= 0x20 {
      return ReportId::InOutUartReport20;
  }
  if length <= 0x24 {
      return ReportId::InOutUartReport24;
  }
  if length <= 0x28 {
      return ReportId::InOutUartReport28;
  }
  if length <= 0x2C {
      return ReportId::InOutUartReport2C;
  }
  if length <= 0x30 {
      return ReportId::InOutUartReport30;
  }
  if length <= 0x34 {
      return ReportId::InOutUartReport34;
  }
  if length <= 0x38 {
      return ReportId::InOutUartReport38;
  }
  if length <= 0x3C {
      return ReportId::InOutUartReport3C;
  } else {
      return ReportId::InOutUartReportOverflow;
  }
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

fn uart_write_request(
  device: &Device,
  report_id: ReportId,
  src: &[u8],
  len_src: usize,
) -> Ft260Result<()> {
  let mut buf = feat_rep_buf();
  buf[0] = report_id as u8;
  buf[1] = len_src as u8;
  for i in 0..len_src {
      buf[2 + i] = src[i];
  }
  device.write_output(&buf)
}

pub(crate) fn read(
  device: &Device,
  buf: &mut [u8],
  byte_to_read: usize,
  duration_wait: Duration,
) -> Ft260Result<usize> {
  let sz_buf = buf.len();
  let mut idx = 0usize;
  let mut byte_returned = 0usize;
  let time_start = Instant::now();
  while byte_returned < byte_to_read {
      if get_input_reports_count_uart(device) == 0 {
          // check timeout
          if (Instant::now() - time_start) >= duration_wait {
              break;
          } else {
              continue;
          }
      }

      if let Some(data) = pop_input_report_uart(device) {
          let len = data.len();
          if len < 2 {
              continue;
          } // error
          if let Ok(rep_id) = data[0].try_into() {
              let sz_from_id = decide_uart_payload_size(rep_id);
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
              for i in 0..sz_cpy {
                  buf[idx + i] = data[2 + i];
              }
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
  buf: &[u8],
  byte_to_write: usize,
) -> Ft260Result<usize> {
  let mut byte_written = 0usize;
  let mut byte_remained = byte_to_write;

  loop {
      let size_write = if byte_remained > UART_PAYLOAD_SIZE_MAX {
          UART_PAYLOAD_SIZE_MAX
      } else {
          byte_remained
      };
      let rid = decide_uart_report_id(size_write);
      let mut slice = feat_rep_buf();
      for i in 0..size_write {
          slice[i] = buf[byte_written + i];
      }
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

pub(crate) fn reset(device: &Device) -> Ft260Result<()> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatSystemSetting as u8;
  buf[1] = Request::ResetUart as u8;
  device.set_feature(&buf)
}

pub(crate) fn get_dcd_ri_status(device: &Device) -> Ft260Result<UartDcdRiStatus> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatUartRiAndDcdStatus as u8;
  if let Err(e) = device.get_feature(&mut buf) {
      return Err(e);
  }
  if let Some(s) = UartDcdRiStatus::from_bits(buf[1]) {
      Ok(s)
  } else {
      Err(Ft260Error::ByteError {
          value: buf[1],
          message: "Couldn't get valid `UartDcdRiStatus` value".to_string(),
      })
  }
}

pub(crate) fn enable_ri_wakeup(
  device: &Device,
  enable: WakeupIntEnableMode,
) -> Ft260Result<()> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatSystemSetting as u8;
  buf[1] = Request::EnableUartRiWaveup as u8;
  buf[2] = enable as u8;
  device.set_feature(&buf)
}

pub(crate) fn set_ri_wakeup_config(
  device: &Device,
  config: UartRiWakeupConfig,
) -> Ft260Result<()> {
  let mut buf = feat_rep_buf();
  buf[0] = ReportId::FeatSystemSetting as u8;
  buf[1] = Request::SetUartRiWakeupConfig as u8;
  buf[2] = config as u8;
  device.set_feature(&buf)
}
