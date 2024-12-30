
#![no_std]
#![no_main]



use micromath::F32Ext;
use esp_backtrace as _;
use esp_hal::{
    delay::{self, Delay}, gpio::{Output, OutputPin}, ledc::channel, prelude::*
};
use s_learn::{Board, Servo};


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut board = Board::new(peripherals);
    let delay = Delay::new();
    let mut pin0_channel0 = board.ledc.channel(channel::Number::Channel0, board.pin0.take().unwrap());
    pin0_channel0
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 0,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    let mut servo: Servo<'_, 1000, 3000,14,50> = Servo::new(pin0_channel0);
    loop {
        if let Some(adc1) = &mut board.adc1 {
            if let Some(adc_pin) = &mut board.adc1_pin36 {
                match adc1.read_oneshot(adc_pin) {
                    Ok(r) => {
                        let percent = (r as f32 / 4095.0 ) * 100.0;
                        servo.set_percentage(percent as u8);
                        // delay.delay_millis(2000u); 
                        esp_println::println!("{} {} {}", r, percent as u8, percent);
                    },
                    Err(_) => {
                        continue;
                    }
                }
            }
        }
    }
}

