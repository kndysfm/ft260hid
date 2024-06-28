use std::sync::MutexGuard;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
/// FT260 Device
use std::thread;
use std::thread::JoinHandle;

// use hidapi crate only in this module
use crate::hid::rep_fifo::ReportFifo;
use crate::io::{gpio::Gpio, i2c::I2c, uart::Uart};
use crate::{Ft260Error, Ft260Result};
use hidapi::{DeviceInfo, HidError};
use log::debug;

/// conversion of `HidError` into `Ft260Error`
impl From<HidError> for Ft260Error {
    fn from(err: HidError) -> Self {
        match err {
            HidError::HidApiError { message: _ }
            | HidError::HidApiErrorEmpty
            | HidError::FromWideCharError { wide_char: _ }
            | HidError::InitializationError
            | HidError::InvalidZeroSizeData
            | HidError::IncompleteSendError { sent: _, all: _ }
            | HidError::SetBlockingModeError { mode: _ }
            | HidError::OpenHidDeviceWithDeviceInfoError { device_info: _ }
            | HidError::IoError { error: _ } => Ft260Error::HidError {
                message: format!("{}", err),
            },
        }
    }
}

/// Data struct for FT260 device

#[derive(Debug)]
pub struct Device {
    hid: Arc<Mutex<hidapi::HidDevice>>,
    fifo: Arc<Mutex<ReportFifo>>,
    reading: Arc<AtomicBool>,
    handle: JoinHandle<()>,
}

/// Default USB Vendor ID for FT260 device
pub const VID_DEFAULT: u16 = 0x0403;
/// Default USB Product ID for FT260 device
pub const PID_DEFAULT: u16 = 0x6030;

/// Open FT260 HID interface with its number.  
/// `interface` should take `0` or `1` only.  
/// If it could be opened, a new thread is spawned and it continues to read HID input report from it.
pub fn open(interface: i32) -> Option<Device> {
    open_by_vid_pid(VID_DEFAULT, PID_DEFAULT, interface)
}

/// Open FT260 HID by explicit Vendor ID and Product ID.  
/// `interface` should take `0` or `1` only.
pub fn open_by_vid_pid(vendor_id: u16, product_id: u16, interface: i32) -> Option<Device> {
    Device::try_new(vendor_id, product_id, interface, 0) // try find only the first one
}

impl Device {
    /// Create new `Device` instance from `hidapi::HidDevice` instance.  
    /// When it is opened, a new thread is spawned and it continues to read HID input report from the device.
    fn new(hid: hidapi::HidDevice) -> Self {
        debug!("{:?}", &hid);
        let mutex_hid = Arc::new(Mutex::new(hid));
        let mutex_fifo = Arc::new(Mutex::new(ReportFifo::new()));
        let reading = Arc::new(AtomicBool::new(true));

        let handle = thread::spawn({
            let mutex_hid = mutex_hid.clone();
            let mutex_fifo = mutex_fifo.clone();
            let reading = reading.clone();
            debug!("now starting a thread to read HID");
            move || loop {
                let mut buf = [0u8; 256];
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
                    debug!("`reading` got to be `false`");
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

    /// Enumerate HID interfaces with specified conditions (VID, PID, IF#)  
    /// If some were found, then create new `Device` instance from `hidapi::HidDevice` instance
    fn try_new(vendor_id: u16, product_id: u16, interface: i32, index: usize) -> Option<Self> {
        if !(0..=1).contains(&interface) {
            return None;
        }

        let api = hidapi::HidApi::new().expect("Failed to create HID API context");
        let mut infs: Vec<&DeviceInfo> = Vec::new();
        for inf in api.device_list() {
            if (vendor_id, product_id, interface)
                == (inf.vendor_id(), inf.product_id(), inf.interface_number())
            {
                debug!("found: {:?}", inf);
                infs.push(inf);
            }
        }
        if index >= infs.len() {
            // out of index range
            None
        } else if let Ok(hid) = infs[index].open_device(&api) {
            debug!("opened: {:?}", index);
            Some(Self::new(hid))
        } else {
            None
        }
    }

    /// Exclusive reference to FIFO instance for HID input report from FT260 device
    pub(crate) fn fifo(&self) -> MutexGuard<'_, ReportFifo> {
        self.fifo.lock().unwrap()
    }

    /// Create instance to control GPIO features
    pub fn gpio(&self) -> Gpio {
        Gpio::new(self)
    }

    /// Create instance to control I2C features
    pub fn i2c(&self) -> I2c {
        I2c::new(self)
    }

    /// Create instance to control UART features
    pub fn uart(&self) -> Uart {
        Uart::new(self)
    }

    /// Read input report manually
    pub(crate) fn read_input(&self, buf: &mut [u8], timeout: i32) -> Ft260Result<usize> {
        match self.hid.lock().unwrap().read_timeout(buf, timeout) {
            Ok(sz) => Ft260Result::Ok(sz),
            Err(e) => Ft260Result::Err(Ft260Error::from(e)),
        }
    }

    /// Write HID output report manually
    pub(crate) fn write_output(&self, data: &[u8]) -> Ft260Result<()> {
        match self.hid.lock().unwrap().write(data) {
            Ok(_) => Ft260Result::Ok(()),
            Err(e) => Ft260Result::Err(Ft260Error::from(e)),
        }
    }

    /// Read HID feature report
    pub(crate) fn get_feature(&self, buf: &mut [u8]) -> Ft260Result<usize> {
        match self.hid.lock().unwrap().get_feature_report(buf) {
            Ok(sz) => Ft260Result::Ok(sz),
            Err(e) => Ft260Result::Err(Ft260Error::from(e)),
        }
    }

    /// Write HID feature report
    pub(crate) fn set_feature(&self, data: &[u8]) -> Ft260Result<()> {
        match self.hid.lock().unwrap().send_feature_report(data) {
            Ok(_) => Ft260Result::Ok(()),
            Err(e) => Ft260Result::Err(Ft260Error::from(e)),
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        debug!("drop it: {:?}", &self);
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
