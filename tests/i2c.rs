use std::thread;
use std::time::Duration;

use ft260hid::device;
use ft260hid::io::i2c;

use rand::prelude::*;

const EEPROM_ADDRESS: u8 = 0x50;

fn wait_in_busy(i2c: &i2c::I2c) {
  while !i2c.is_idle() { }
}

fn wait_write(i2c: &i2c::I2c) {
  while !i2c.is_idle() { }
  thread::sleep(Duration::from_millis(5));
}

#[test]
fn test_i2c_basic() {
  let mut rand = [0u8;8];
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

  // write address to read
  let data_addr = [random::<u8>()];
  // write 8 byte data
  assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::Start, &data_addr, 1).unwrap(), 1);
  assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::Stop, &rand, rand.len()).unwrap(), rand.len());
  wait_write(&i2c);
  let mut data_read = [0u8; 8];
  assert_eq!(i2c.write(EEPROM_ADDRESS, i2c::Flag::Start, &data_addr, 1).unwrap(), 1);
  assert_eq!(i2c.read(EEPROM_ADDRESS, i2c::Flag::ReStartAndStop, &mut data_read, 8, i2c::DURATION_WAIT_DEFAULT).unwrap(), 8);
  assert_eq!(rand[7], data_read[7]);

}