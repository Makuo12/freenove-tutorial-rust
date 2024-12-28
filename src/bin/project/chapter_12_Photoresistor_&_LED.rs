
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
    
    let mut pin15_channel0 = board.ledc.channel(channel::Number::Channel0, board.pin15.take().unwrap());
    pin15_channel0
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 100,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    let mut min = 10;
    let mut max = 10;
    let mut value_36 = 0;
    loop {
        if let Some(adc1) = &mut board.adc1 {
            
            let adc1_pin36 = match &mut board.adc1_pin36 {
                Some(r) => r,
                None => {
                    continue;
                }
            };
            
            match adc1.read_oneshot(adc1_pin36) {
                Ok(r) => {
                    if r != 0 {
                        value_36 = r;
                    }
                },
                Err(_) => {
                }
            };
            esp_println::println!("{}, {}", remap(value_36) as u8, remap(value_36));
            pin15_channel0.set_duty(remap(value_36) as u8);
        }
    }
}



// fn remap(value: u16) -> f32 {
//     return (value as f32 / 4095.0) * 255.0;
// }

fn remap(value: u16) -> f32 {
    return 255.0 - ((value as f32 / 1504.0) * 255.0)
}