use std::thread;
use std::time::Duration;

use ft260hid::device;
use ft260hid::io::gpio::{Pin, Dir, Group, Val};

#[test]
fn read_value() {
  let dev = device::open(0).unwrap();
  let gpio = dev.gpio();
  assert!(gpio.enable_pin(Group::Gpio_B_C_D_E_F_H).is_ok());
  assert!(gpio.set_dir(Pin::GpioB, Dir::Out).is_ok()); // RTS / GPIOB
  assert!(gpio.set_dir(Pin::GpioE, Dir::In).is_ok());  // CTS / GPIOE
  assert!(gpio.set_dir(Pin::GpioC, Dir::In).is_ok());  // RXD / GPIOC
  assert!(gpio.set_dir(Pin::GpioD, Dir::Out).is_ok()); // TXD / GPIOD

  let in_out = [
    (Pin::GpioC, Pin::GpioD),
    (Pin::GpioE, Pin::GpioB),
    ];
  let delay = Duration::from_millis(1);

  for io in in_out {
    let hi_lo = [Val::High, Val::Low];
    for v in hi_lo {
      thread::sleep(delay);
      assert!(gpio.write(io.1, v).is_ok());
      thread::sleep(delay);
      let res = gpio.read(io.1); // read out
      assert!(res.is_ok()); 
      assert_eq!(res.unwrap(), v);
      let res = gpio.read(io.0); // read in
      assert!(res.is_ok()); 
      assert_eq!(res.unwrap(), v);
    }
  }

  // Try to monitor actual wave form
  assert!(gpio.enable_pin(Group::Gpio_0_1).is_ok());
  assert!(gpio.set_dir(Pin::Gpio0, Dir::Out).is_ok()); // SCL / GPIO0
  assert!(gpio.set_dir(Pin::Gpio1, Dir::Out).is_ok()); // SDA / GPIO1
  assert!(gpio.write(Pin::Gpio0, Val::Low).is_ok());
  assert!(gpio.write(Pin::Gpio1, Val::Low).is_ok());
  thread::sleep(delay);
  assert!(gpio.write(Pin::Gpio0, Val::High).is_ok());
  thread::sleep(delay);
  assert!(gpio.write(Pin::Gpio1, Val::High).is_ok());
  thread::sleep(delay);
  assert!(gpio.write(Pin::Gpio0, Val::Low).is_ok());
  thread::sleep(delay);
  assert!(gpio.write(Pin::Gpio1, Val::Low).is_ok());
  thread::sleep(delay);
  
  assert!(gpio.set_dir(Pin::Gpio0, Dir::In).is_ok()); // SCL / GPIO0
  assert!(gpio.set_dir(Pin::Gpio1, Dir::In).is_ok()); // SDA / GPIO1
  assert!(gpio.set_pull_up(Pin::Gpio0).is_ok());
  thread::sleep(delay);
  assert!(gpio.set_pull_up(Pin::Gpio1).is_ok());
  thread::sleep(delay);
  assert!(gpio.set_pull_down(Pin::Gpio0).is_ok());
  thread::sleep(delay);
  assert!(gpio.set_pull_down(Pin::Gpio1).is_ok());
  thread::sleep(delay);

}