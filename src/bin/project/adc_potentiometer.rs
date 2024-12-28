
#![no_std]
#![no_main]


use core::str::from_utf8;

use esp_backtrace as _;
use esp_hal::{
    delay::Delay, prelude::*
};
use s_learn::Board;


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut board = Board::new(peripherals);
    // let mut channels = get_channels(&mut board);
    loop {
        if let Some(adc1) = &mut board.adc1 {
            if let Some(adc1_pin) = &mut board.adc1_pin {
                let result = match adc1.read_oneshot(adc1_pin) {
                    Ok(r) => r,
                    Err(_) => {
                        continue;
                    }
                };
                // To calculate the voltage
                let vol = (result as f32 * 3.3) / 4095.0;
                esp_println::println!("Voltage: {}, ADC: {}", vol, result)
            }
        }
    }
}

