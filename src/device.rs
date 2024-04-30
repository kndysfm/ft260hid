use std::sync::MutexGuard;
/// FT260 Device
use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};

// use hidapi crate only in this module 
use hidapi::{DeviceInfo, HidError};

use crate::{Ft260Result, Ft260Error};
use crate::hid::rep_fifo::ReportFifo;
use crate::io::{gpio::Gpio, i2c::I2c, uart::Uart};

/// conversion of `HidError` into `Ft260Error`
impl From<HidError> for Ft260Error {
  fn from(err: HidError) -> Self{
    match err {
      HidError::HidApiError  { message: _ } |
      HidError::HidApiErrorEmpty |
      HidError::FromWideCharError { wide_char: _ } |
      HidError::InitializationError |
      HidError::InvalidZeroSizeData |
      HidError::IncompleteSendError { sent: _, all: _ } |
      HidError::SetBlockingModeError  { mode: _ } |
      HidError::OpenHidDeviceWithDeviceInfoError { device_info: _ } |
      HidError::IoError { error: _ } => {
        Ft260Error::HidError { message: format!("{}", err), }
      },
    }
  }
}

#[derive(Debug)]
pub struct Device {
  hid: Arc<Mutex<hidapi::HidDevice>>,
  fifo: Arc<Mutex<ReportFifo>>,
  reading: Arc<AtomicBool>,
  handle: JoinHandle<()>,
}

pub const VID_DEFAULT: u16 = 0x0403;
pub const PID_DEFAULT: u16 = 0x6030;

pub fn open(index: usize) -> Option<Device> {
  open_by_vid_pid(VID_DEFAULT, PID_DEFAULT, index)
}

pub fn open_by_vid_pid(vendor_id: u16, product_id: u16, index: usize) -> Option<Device> {
  Device::try_new(vendor_id, product_id, index)
}

impl Device {

  fn new(hid: hidapi::HidDevice) -> Self {
    dbg!(&hid);
    let mutex_hid = Arc::new(Mutex::new(hid));
    let mutex_fifo = Arc::new(Mutex::new(ReportFifo::new()));
    let reading = Arc::new(AtomicBool::new(true));

    let handle = thread::spawn({
      let mutex_hid = mutex_hid.clone();
      let mutex_fifo = mutex_fifo.clone();
      let reading = reading.clone();
      move || loop {
        print!("now starting a thread to read HID");
        let mut buf = [0u8;256];
        let mut has_report = false;
        if let Ok(hid) = mutex_hid.lock() {
          if let Ok(sz) = hid.read_timeout(&mut buf, 0) {
            has_report = sz > 0;
          }
        }
        thread::yield_now();
        if has_report {
          if let Ok(mut fifo) = mutex_fifo.lock() {
            fifo.push_report(buf.to_vec());
          }
        }
        thread::yield_now();
        if !reading.load(Ordering::Relaxed) {
          print!("`reading` got to be `false`");
          return;
        }
      }
    });

    Self {
      hid: mutex_hid, 
      fifo: mutex_fifo,
      reading,
      handle,
    }
  }

  fn try_new(vendor_id: u16, product_id: u16, index: usize) -> Option<Self> {
    let api = hidapi::HidApi::new().expect("Failed to create HID API context");
    let mut infs: Vec<&DeviceInfo> = Vec::new();
    for inf in api.device_list() {
      if (vendor_id, product_id) == (inf.vendor_id(), inf.product_id()) {
        print!("found:");
        dbg!(inf);
        infs.push(inf);
      }
    }
    if index >= infs.len() {
      // out of index range
      None
    } else {
      if let Ok(hid) = infs[index].open_device(&api) {
        print!("opened:");
        dbg!(index);
        Some(Self::new(hid))
      } else {
        None
      }
    }
  }

  pub(crate) fn fifo<'a>(&'a self) -> MutexGuard<'a, ReportFifo> {
    self.fifo.lock().unwrap()
  }

  pub fn gpio(&self) -> Gpio {
    Gpio::new(self)
  }

  pub fn i2c(&self) -> I2c {
    I2c::new(self)
  }

  pub fn uart(&self) -> Uart {
    Uart::new(self)
  }

  pub(crate) fn read_input(&self, buf: &mut [u8], timeout: i32) -> Ft260Result<usize> {
    match self.hid.lock().unwrap().read_timeout(buf, timeout) {
      Ok(sz) => Ft260Result::Ok(sz),
      Err(e) => Ft260Result::Err(Ft260Error::from(e)),
    }
  }

  pub(crate) fn write_output(&self, data: &[u8]) -> Ft260Result<()> {
    match self.hid.lock().unwrap().write(data) {
      Ok(_) => Ft260Result::Ok(()),
      Err(e) => Ft260Result::Err(Ft260Error::from(e)),
    }
  }
  
  pub(crate) fn get_feature(&self, buf: &mut [u8]) -> Ft260Result<usize> {
    match self.hid.lock().unwrap().get_feature_report(buf) {
      Ok(sz) => Ft260Result::Ok(sz),
      Err(e) => Ft260Result::Err(Ft260Error::from(e)),
    }
  }

  pub(crate) fn set_feature(&self, data: &[u8]) -> Ft260Result<()> {
    match self.hid.lock().unwrap().send_feature_report(data) {
      Ok(_) => Ft260Result::Ok(()),
      Err(e) => Ft260Result::Err(Ft260Error::from(e)),
    }
  }

}

impl Drop for Device {
  fn drop(&mut self) {
    print!("drop it:");
    dbg!(&self);
    // stop reading thread
    self.reading.store(false, Ordering::Relaxed);
    loop { 
      if self.handle.is_finished() { 
        println!("joined.");
        return; 
      } 
    }
  }
}
