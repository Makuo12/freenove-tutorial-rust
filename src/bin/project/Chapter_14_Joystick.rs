
#![no_std]
#![no_main]



use micromath::F32Ext;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay, ledc::channel, prelude::*
};
use s_learn::Board;


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut board = Board::new(peripherals);
    let mut x_val = 0;
    let mut y_val = 0;
    let mut z_val = 0;
    let delay = Delay::new();
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
                    x_val = r;
                    delay.delay_millis(1000);
                },
                Err(_) => {
                }
            };
            
            let adc1_pin39 = match &mut board.adc1_pin39 {
                Some(r) => r,
                None => {
                    continue;
                }
            };
            
            match adc1.read_oneshot(adc1_pin39) {
                Ok(r) => {
                    y_val = r;
                },
                Err(_) => {
                }
            };
        }
        if let Some(button12) = &mut board.button12 {
            if button12.is_low() {
                z_val = 1;
            } else {
                z_val = 0;
            }
        }
        esp_println::println!("X: {}, Y: {}, Z: {}", x_val, y_val, z_val)
    }
}
