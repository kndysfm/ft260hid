# ft260hid

FTDI 社製 USB-I2C/UART 変換 IC FT260 を Rust から制御するための非公式ライブラリーです。

## FT260 について

- [データシート](https://ftdichip.com/wp-content/uploads/2023/11/DS_FT260.pdf)
- [アプリケーションノート](https://ftdichip.com/wp-content/uploads/2020/07/AN_394_User_Guide_for_FT260.pdf)

この USB シリアル変換 IC の特徴として、仮想シリアルポートではなく HID インターフェース経由で制御を行うところにあります。  
そのため取り扱いが一般的な変換 IC と比べてやや複雑になる一方で、通常のシリアル通信以外に例えば GPIO の制御を行ったり [HID over I2C](https://learn.microsoft.com/en-us/windows-hardware/drivers/hid/hid-over-i2c-guide) に準拠した通信を行ったりといった、比較的柔軟な利用が可能になっています。

## 使用例

詳しくは同梱の単体テストを参照してください。  
テストには市販の同 IC の評価ボード [UMFT260EV1A](https://ftdichip.com/products/umft260ev1a/) を使用しております。

### GPIO

```rust
use ft260hid::device;
use ft260hid::io::gpio::{Dir, Group, Pin, Val};
// . . .
    // FT260 の HID インターフェースを開く
    let dev = device::open(0).unwrap();
    // GPIO 機能インターフェースを作成
    let gpio = dev.gpio();
    // GPIO 0,1 (I2C 兼用) ピンを GPIO として使用する
    gpio.enable_pin(Group::Gpio_0_1);
    // GPIO 0 を出力に設定
    gpio.set_dir(Pin::Gpio0, Dir::Out);
    // GPIO 0 から Low 出力
    gpio.write(Pin::Gpio0, Val::Low);
    // GPIO 1 を入力に設定
    gpio.set_dir(Pin::Gpio1, Dir::In);
    // GPIO 1 の内部プルアップを有効化
    gpio.set_pull_up(Pin::Gpio1);
```

### I2C

UMFT260EV1A ボード上の I2C EEPROM ([AT24C02D_SOT23](https://ww1.microchip.com/downloads/en/DeviceDoc/AT24C01D-AT24C02D-I2C-Compatible-Two-Wire-Serial-EEPROM-1Kbit-2Kbit-20006100A.pdf)) を用いて単体テストを行うことが可能です。

```rust
use ft260hid::device;
use ft260hid::io::i2c;
// . . .
/// UMFT260EV1A ボード上の EEPROM の I2C アドレス 
const EEPROM_ADDRESS: u8 = 0x50;
/// EEPROM のページサイズ
const EEPROM_PAGE_SIZE: usize = 8;
// . . .
    // FT260 の HID インターフェースを開く
    let dev = device::open(0).unwrap();
    // I2C 機能インターフェースを作成
    let mut i2c = dev.i2c();
    // I2C 機能の初期化
    i2c.init(i2c::KBPS_DEFAULT);
    // EEPROM ページ読み出しのための Write データ
    let addr = [0u8];
    // Read データバッファ
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

UMFT260EV1A ボード上の TXD-RXD ピンをショートさせることで、単体テストが可能になります。

```rust
use ft260hid::device;
use ft260hid::io::uart;
// . . .
    // FT260 の HID インターフェースを開く (インターフェース番号に注意)
    let dev = device::open(1).unwrap();
    // UART 機能インターフェースを作成
    let mut uart = dev.uart();
    // UART 機能の初期化
    uart.init();
    // デフォルト値を設定
    uart.set_config(&uart::Config::default());
    // UART TX
    let mut buf_tx = [0u8; 256];
    let size_sent = uart.write(&buf_tx, len).unwrap();
    // FIFO の中の RX データのサイズ
    let size_to_read = uart.size_to_read();
    // UART RX
    let mut buf_rx = [0u8; 256];
    let size_rec = uart.read(&mut buf_rx, size_to_read, uart::DURATION_WAIT_DEFAULT).unwrap();
```


## 免責事項

本ソフトウェアは FTDI 社の公式製品ではなく、独自に開発された非公式のオープンソースソフトウェアです。本ソフトウェアの使用は自己責任で行ってください。本ソフトウェアの使用によって生じたいかなる直接的または間接的な損害はすべて使用者の責任となります。

本ソフトウェアは FTDI 社の IC を制御するためのものですが FTDI 社による公式のサポートや保証はありません。本ソフトウェアの使用に関連するリスクはすべて使用者が負うものとします。

本ソフトウェアの使用により、FTDI 社の製品保証が無効になる可能性があることを予めご了承ください。また、本ソフトウェアの使用が原因で FTDI 社の製品が損傷した場合、修理や交換の責任は使用者が負うものとします。

本ソフトウェアはオープンソースライセンスの下で提供されています。ライセンス条項に従って、自由に使用、改変、再配布することができますが、それにはライセンスに記載されている条件が適用されます。ライセンスの全文は、本ソフトウェアに同梱されているLICENSEファイル（またはオンラインで公開されているURL）で確認できます。

本免責事項は、予告なしに変更されることがあります。最新の情報については、本ソフトウェアの公式リポジトリまたはウェブサイトをご確認ください。
