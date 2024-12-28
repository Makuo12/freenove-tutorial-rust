
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Level, Output, Pin, Pull},
    prelude::*,
};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Set LED GPIOs as an output:
    let mut led1 = Output::new(peripherals.GPIO4.degrade(), Level::Low);
    let button = peripherals.GPIO13.degrade();

    let button = Input::new(button, Pull::Up);

    // let mut pins = [led1];

    let delay = Delay::new();
    let mut show_led = false;
    loop {
        if button.is_low() {
            delay.delay_millis(50);
            if button.is_low() {
                led1.toggle();
            }
            while button.is_low() {

            }
        } 
    }
}

