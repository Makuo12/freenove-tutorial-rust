
#![no_std]
#![no_main]


use esp_backtrace as _;
use esp_hal::{
    delay::Delay, ledc::channel::{self, Number}, 
    prelude::*
};
use s_learn::Board;


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut board = Board::new(peripherals);
    let delay = Delay::new();

    let mut pin15_channel0 = board.ledc.channel(channel::Number::Channel0, board.led15.take().unwrap());
    pin15_channel0
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin2_channel1 = board.ledc.channel(channel::Number::Channel1, board.led2.take().unwrap());
    pin2_channel1
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin4_channel2 = board.ledc.channel(channel::Number::Channel2, board.led4.take().unwrap());
    pin4_channel2
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    // let mut channels = get_channels(&mut board);
    loop {
        pin15_channel0.set_duty(0);
        pin2_channel1.set_duty(255);
        pin4_channel2.set_duty(0);
    }
}
