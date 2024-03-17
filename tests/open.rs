use ft260hid::device;

#[test]
fn open_hid() {
  assert!(device::open_by_vid_pid(0x0403, 0x6030, 0).is_some());
  assert!(device::open_by_vid_pid(0x0403, 0x6030, 1).is_some());
  assert!(device::open_by_vid_pid(0x0403, 0x6030, 2).is_none());
  assert!(device::open(0).is_some());
  assert!(device::open(1).is_some());
  assert!(device::open(2).is_none());
}
