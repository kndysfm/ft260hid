//!
//! # To test library for FT260
//! 
//! ## Equipment
//! 
//! Consider to use [UMFT260EV1A](https://ftdichip.com/products/umft260ev1a/),
//! development kit board.  
//! EEPROM chip is mounted on it, and connected with FT260 device by I2C lines, SCL and SDA.  
//! UART and GPIO functions can be tested by short-circuiting between FT260 pins.
//! 
//! ## Settings
//! 
//! ### Interfaces
//! 
//! To test I2C and UART, set `DCNF0=1` (JP7) and `DCNF1=1` (JP9).
//! With this configuration, two HID interfaces should be created.
//! One is for I2C, and the other is for UART.
//! 
//! ### I2C
//! 
//! No need to modify. But if `JP8` has been cut already, solder it again.
//! 
//! ### UART
//! 
//! To test TX and RX simultaneously, FT260 plays both roles, sender and receiver.
//! On a bread-board or somehow with something, short-circuit `RTS`` - `CTS``, and `RXD` and `TXD`.  
//! Or `JP5.8` to `JP5.9`, and `JP5.9` to `JP5.10`
//! 
//! ### GPIO
//! 
//! UART ports can be used also for testing GPIO functions.  
//! To add, short-circuit `IO2` - `IO3`, or `JP6.9` to `JP.8`.  
//! 
pub mod open;
pub mod gpio;
pub mod i2c;
pub mod uart;