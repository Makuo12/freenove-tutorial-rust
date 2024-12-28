
#![no_std]
#![no_main]


use core::str::from_utf8;

use esp_backtrace as _;
use esp_hal::{
    delay::Delay, ledc::channel, prelude::*
};
use s_learn::Board;


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut board = Board::new(peripherals);
    let mut pin4_channel0 = board.ledc.channel(channel::Number::Channel0, board.pin4.take().unwrap());
    pin4_channel0
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
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
                let value = remap(result, 0, 4095, 0, 1023);
                pin4_channel0.set_duty(value as u8);
            }
        }
    }
}

fn remap(adc_value: u16, old_min: u16, old_max: u16, new_min: u16, new_max: u16) -> u16 {
    return adc_value * (new_max-new_min) / (old_max-old_min)
}