# ft260hid

This unofficial library controls FT260 USB-I2C/UART bridge IC made by FTDI.  
The library depends on "hidapi" crate mainly.

## About FT260

- [Product Page](https://ftdichip.com/products/ft260q/)
- [Application Note](https://ftdichip.com/wp-content/uploads/2020/07/AN_394_User_Guide_for_FT260.pdf)

This USB device does not have virtual COM port. Instead, it has HID interface.  
For example, HID Input/Output Reports are converted into I2C/UART communication between the FT260 and its external devices.

## Examples

Unit tests are executed on the evaluation board [UMFT260EV1A](https://ftdichip.com/products/umft260ev1a/).

### GPIO

```rust
use ft260hid::device;
use ft260hid::io::gpio::{Dir, Group, Pin, Val};
// . . .
    let dev = device::open(0).unwrap();
    let gpio = dev.gpio();
    gpio.enable_pin(Group::Gpio_0_1);
    gpio.set_dir(Pin::Gpio0, Dir::Out);
    gpio.write(Pin::Gpio0, Val::Low);
    gpio.set_dir(Pin::Gpio1, Dir::In);
    gpio.set_pull_up(Pin::Gpio1);
```

### I2C

I2C EEPROM ([AT24C02D_SOT23](https://ww1.microchip.com/downloads/en/DeviceDoc/AT24C01D-AT24C02D-I2C-Compatible-Two-Wire-Serial-EEPROM-1Kbit-2Kbit-20006100A.pdf)) is mounted on UMFT260EV1A board,
 and it can be used for unit tests.

```rust
use ft260hid::device;
use ft260hid::io::i2c;
// . . .
/// I2C address of EEPROM on UMFT260EV1A
const EEPROM_ADDRESS: u8 = 0x50;
/// page size of EEPROM
const EEPROM_PAGE_SIZE: usize = 8;
// . . .
    let dev = device::open(0).unwrap();
    let mut i2c = dev.i2c();
    i2c.init(i2c::KBPS_DEFAULT);
    // address value to read EEPROM page out
    let addr = [0u8];
    let mut data_read = [0u8; EEPROM_PAGE_SIZE];
    i2c.write_read(EEPROM_ADDRESS,
                &addr,
                1,
                &mut buf,
                EEPROM_PAGE_SIZE,
                i2c::DURATION_WAIT_DEFAULT
            );
```

### UART

The TXD-RXD pins on UMFT260EV1A are shorted for unit tests.

```rust
use ft260hid::device;
use ft260hid::io::uart;
// . . .
    // interface number is `1` !!
    let dev = device::open(1).unwrap();
    let mut uart = dev.uart();
    uart.init();
    uart.set_config(&uart::Config::default());
    // UART TX
    let mut buf_tx = [0u8; 256];
    let size_sent = uart.write(&buf_tx, len).unwrap();
    // RX data in FIFO
    let size_to_read = uart.size_to_read();
    // UART RX
    let mut buf_rx = [0u8; 256];
    let size_rec = uart.read(&mut buf_rx, size_to_read, uart::DURATION_WAIT_DEFAULT).unwrap();
```


