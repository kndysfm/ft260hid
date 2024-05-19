use std::thread;
use std::time::Duration;

use ft260hid::device;
use ft260hid::io::uart;

use rand::prelude::*;
use serial_test::serial;

#[test]
#[serial]
fn test_uart_cfg() {
    let dev = device::open(1).unwrap();
    let mut uart = dev.uart();

    assert!(uart.init().is_ok());
    let cfg = uart::Config::default();
    assert!(uart.set_config(&cfg).is_ok());
    let res = uart.get_config();
    assert!(res.is_ok());
    let cfg_read = res.unwrap();
    assert_eq!(cfg, cfg_read);

    let cfg = uart::Config {
        mode: uart::Mode::RtsCts,
        baud: 57600,
        data_bits: uart::DataBits::Eight,
        stop_bit: uart::StopBit::One,
        parity: uart::Parity::None,
        breaking: uart::Breaking::Break,
    };
    assert!(uart.set_config(&cfg).is_ok());
    let res = uart.get_config();
    assert!(res.is_ok());
    let cfg_read = res.unwrap();
    assert_eq!(cfg, cfg_read);
}

#[test]
#[serial]
fn test_uart_tx_rx() {
    let mut buf_tx = [0u8; 256];

    let dev = device::open(1).unwrap();
    let mut uart = dev.uart();

    assert!(uart.init().is_ok());
    assert!(uart.set_config(&uart::Config::default()).is_ok());

    let len_list = [8usize, 32, 128, 256];
    for len in len_list {
        thread_rng().fill(&mut buf_tx);

        let size_in_fifo = uart.size_to_read();
        assert_eq!(0usize, size_in_fifo);

        let res = uart.write(&buf_tx, len);
        assert!(res.is_ok());
        let size_sent = res.unwrap();
        assert_eq!(len, size_sent);

        // to wait enqueuing RX data buffer via shorted TXD pin
        thread::sleep(Duration::from_millis(len as u64));
        let size_in_fifo = uart.size_to_read();
        assert_eq!(len, size_in_fifo);

        let mut buf_rx = [0u8; 256];
        let res = uart.read(&mut buf_rx, len, uart::DURATION_WAIT_DEFAULT);
        assert!(res.is_ok());
        let size_rec = res.unwrap();
        assert_eq!(len, size_rec);

        for i in 0..len {
            assert_eq!(buf_rx[i], buf_tx[i]);
        }
    }
}
