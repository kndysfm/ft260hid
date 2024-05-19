use std::thread;
use std::time::Duration;

use ft260hid::device;
use ft260hid::io::i2c;

use rand::prelude::*;
use serial_test::serial;

const EEPROM_ADDRESS: u8 = 0x50;
const EEPROM_PAGE_SIZE: usize = 8;

fn wait_in_busy(i2c: &i2c::I2c) {
  loop {
    match i2c.is_idle() { 
      Some(true) => { break; },
      Some(false) => { continue; },
      None => panic!(),
    }
  }
}

fn wait_write(i2c: &i2c::I2c) {
  wait_in_busy(i2c);
  // write cycle time 5ms typ.
  thread::sleep(Duration::from_millis(5));
}

#[test]
#[serial]
fn test_i2c_basic() {
  let mut rand = [0u8;EEPROM_PAGE_SIZE];
  thread_rng().fill(&mut rand);

  let dev = device::open(0).unwrap();
  let mut i2c = dev.i2c();

  // Test I2C communication with "AT24C02D" mounted on "UMFT260EV1A" board
  // 7bit device address : 0b1010000 = 0x50

  assert!(i2c.init(i2c::KBPS_DEFAULT).is_ok());
  wait_in_busy(&i2c);
  let data_write = [ rand[0], rand[1] ]; // [random address, random value]
  assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::StartAndStop, &data_write, 2).unwrap(), 2);
  wait_write(&i2c);
  let mut data_read = [0u8;1];
  // write address to read
  assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::Start, &data_write, 1).unwrap(), 1);
  // read 1 byte
  assert_eq!(i2c.read(EEPROM_ADDRESS, i2c::Flag::ReStartAndStop, &mut data_read, 1, i2c::DURATION_WAIT_DEFAULT).unwrap(), 1);
  // compare values written and read out
  assert_eq!(rand[1], data_read[0]);
  wait_in_busy(&i2c);

  let data_write = [ rand[2], rand[3] ];
  assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::StartAndStop, &data_write, 2).unwrap(), 2);
  wait_write(&i2c);
  let mut data_read = [0u8;1];
  // write address to read
  assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::Start, &data_write, 1).unwrap(), 1);
  // read 1 byte
  assert_eq!(i2c.read(EEPROM_ADDRESS, i2c::Flag::ReStartAndStop, &mut data_read, 1, i2c::DURATION_WAIT_DEFAULT).unwrap(), 1);
  // compare values written and read out
  assert_eq!(rand[3], data_read[0]);
  wait_in_busy(&i2c);

  // write address to read (clear 3 LSB bit to prevent of roll-over within a page sized in up to 8 bytes)
  let data_addr = [random::<u8>() & 0xF8u8];
  // write 8 byte data
  assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::Start, &data_addr, 1).unwrap(), 1);
  assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::Stop, &rand, rand.len()).unwrap(), rand.len());
  wait_write(&i2c);
  let mut data_read = [0u8; EEPROM_PAGE_SIZE];
  assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::Start, &data_addr, 1).unwrap(), 1);
  assert_eq!(i2c.read(EEPROM_ADDRESS, i2c::Flag::ReStartAndStop, &mut data_read, EEPROM_PAGE_SIZE, i2c::DURATION_WAIT_DEFAULT).unwrap(), EEPROM_PAGE_SIZE);
  assert_eq!(rand[EEPROM_PAGE_SIZE-1], data_read[EEPROM_PAGE_SIZE-1]);

}

/// test repeated read write
#[test]
#[serial]
fn test_i2c_write_read() {
  let dev = device::open(0).unwrap();
  let mut i2c = dev.i2c();
  assert!(i2c.init(i2c::KBPS_DEFAULT).is_ok());

  for i in 0..10 {
      let mut data = [0u8;EEPROM_PAGE_SIZE];
      thread_rng().fill(&mut data);
      let addr = [random::<u8>() & 0xF8u8];
      // write 8 byte data
      assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::Start, &addr, addr.len()).unwrap(), addr.len());
      assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::Stop, &data, data.len()).unwrap(), data.len());
      wait_write(&i2c);
      // write-read 8 byte data
      let mut buf = [0u8; EEPROM_PAGE_SIZE];
      assert!(i2c.write_read(EEPROM_ADDRESS, &addr, 1, &mut buf, EEPROM_PAGE_SIZE, i2c::DURATION_WAIT_DEFAULT).is_ok());
      assert_eq!(data[EEPROM_PAGE_SIZE-1], buf[EEPROM_PAGE_SIZE-1]);
  }
}

/// test long read write
#[test]
#[serial]
fn test_i2c_long_data() {
  let mut data = [0u8;256];
  thread_rng().fill(&mut data);
  let kbps_list = [400u16, 100u16];
  let len_list = [256usize, 128usize, 64usize, 32usize, 16usize];

  let dev = device::open(0).unwrap();
  let mut i2c = dev.i2c();
  for kbps in kbps_list {
    assert!(i2c.init(kbps).is_ok());
    wait_in_busy(&i2c);
    for len in len_list {
      assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::StartAndStop, &data, len).unwrap(), len);
      wait_write(&i2c);
      let mut buf = [0u8; 256];
      assert!(i2c.write_read(EEPROM_ADDRESS, &data, 1, &mut buf, len, i2c::DURATION_WAIT_DEFAULT).is_ok());
      wait_in_busy(&i2c);
    }
  }
}