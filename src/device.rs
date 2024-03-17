use hidapi::DeviceInfo;

#[derive(Debug)]
pub struct Device {
  hid: hidapi::HidDevice,
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
    dbg!(format!("{hid:?}"));
    Self {hid}
  }

  fn try_new(vendor_id: u16, product_id: u16, index: usize) -> Option<Self> {
    let api = hidapi::HidApi::new().expect("Failed to create HID API context");
    let mut infs: Vec<&DeviceInfo> = Vec::new();
    for inf in api.device_list() {
      if (vendor_id, product_id) == (inf.vendor_id(), inf.product_id()) {
        dbg!("found:");
        dbg!(inf);
        infs.push(inf);
      }
    }
    if index >= infs.len() {
      // out of index range
      None
    } else {
      if let Ok(hid) = infs[index].open_device(&api) {
        dbg!("opened:");
        dbg!(index);
        Some(Self::new(hid))
      } else {
        None
      }
    }
  }

  pub(crate) fn read_input(&self, buf: &mut [u8], timeout: i32) -> usize{
    match self.hid.read_timeout(buf, timeout) {
      Ok(sz) => sz,
      Err(_) => 0usize,
    }
  }

  pub(crate) fn write_output(&self, data: &[u8]) -> bool {
    self.hid.write(data).is_ok()
  }
  
  pub(crate) fn get_feature(&self, buf: &mut [u8]) -> usize {
    match self.hid.get_feature_report(buf) {
      Ok(sz) => sz,
      Err(_) => 0usize,
    }
  }

  pub(crate) fn set_feature(&self, data: &[u8]) -> bool {
    self.hid.send_feature_report(data).is_ok()
  }

}
