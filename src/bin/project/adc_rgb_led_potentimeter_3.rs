#![no_std]
#![no_main]



use esp_backtrace as _;
use esp_hal::{
    delay::Delay, ledc::channel, prelude::*
};
use s_learn::Board;


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut board = Board::new(peripherals);
    
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
    let mut value_34 = 0;
    let mut value_36 = 0;
    let mut value_39 = 0;
    loop {
        if let Some(adc1) = &mut board.adc1 {
            
            let adc1_pin36 = match &mut board.adc1_pin36 {
                Some(r) => r,
                None => {
                    esp_println::println!("failed 3");
                    continue;
                }
            };
            let adc1_pin39 = match &mut board.adc1_pin39 {
                Some(r) => r,
                None => {
                    esp_println::println!("failed 3");
                    continue;
                }
            };
            let adc1_pin34 = match &mut board.adc1_pin34 {
                Some(r) => r,
                None => {
                    esp_println::println!("failed 3");
                    continue;
                }
            };
            match adc1.read_oneshot(adc1_pin34) {
                Ok(r) => {

                    value_34 = r;
                },
                Err(_) => {
                    
                }
            };
            match adc1.read_oneshot(adc1_pin36) {
                Ok(r) => {
                    value_36 = r;
                },
                Err(_) => {
                }
            };
            match adc1.read_oneshot(adc1_pin39) {
                Ok(r) => {
                    value_39 = r;
                },
                Err(_) => {
                }
            };
            esp_println::println!("1: {}, n_1: {}, 2: {}, 3: {}", remap(value_34) as u8, value_34, remap(value_36), remap(value_39));
            pin15_channel0.set_duty(remap(value_34) as u8);
            pin2_channel1.set_duty(remap(value_36) as u8);
            pin4_channel2.set_duty(remap(value_39) as u8);
        }
    }
}



fn remap(value: u16) -> f32 {
    return (value as f32 / 4095.0) * 255.0;
}