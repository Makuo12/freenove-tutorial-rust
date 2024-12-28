#![no_std]
#![no_main]



use micromath::F32Ext;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay, gpio::{Output, OutputPin}, ledc::channel, prelude::*
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
    let mut servo: Servo<'_, 1000, 2000,14,50> = Servo::new(pin0_channel0);
    loop {
        loop {
            servo.set_percentage(0);
            delay.delay_millis(2000u32);
            servo.set_percentage(50);
            delay.delay_millis(2000u32);
            servo.set_percentage(100);
            delay.delay_millis(1000u32);
        }
    }
}

