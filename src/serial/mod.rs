use nrf52840_hal::gpio::Level;
use nrf52840_hal::gpio::*;
use nrf52840_hal::nrf52840_pac as pac;
use nrf52840_hal::uarte;
use nrf52840_hal::uarte::Uarte;
use nrf52840_hal::uarte::{Baudrate, Parity};

pub fn init(p0: pac::P0, uarte0: pac::UARTE0) {
    let port0 = p0.split();
    let pins = uarte::Pins {
        txd: port0.p0_06.into_push_pull_output(Level::High).degrade(),
        rxd: port0.p0_08.into_floating_input().degrade(),
        rts: Some(port0.p0_05.into_push_pull_output(Level::High).degrade()),
        cts: Some(port0.p0_07.into_floating_input().degrade()),
    };

    let mut cdc_uart = Uarte::new(uarte0, pins, Parity::EXCLUDED, Baudrate::BAUD115200);
    let &reference_data = b"Hello,echo Loopback\r\n";

    cdc_uart.write(&reference_data).unwrap();
}
