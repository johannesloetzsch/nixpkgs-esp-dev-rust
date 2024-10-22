#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Io, Output, Level}, prelude::*};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio23, Level::High);
    let delay = Delay::new();

    esp_println::logger::init_logger_from_env();

    loop {
        log::info!("Hello world!");
        led.set_high();
        delay.delay(1000.millis());
        led.set_low();
        delay.delay(1000.millis());
    }
}
