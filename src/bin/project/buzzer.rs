#![no_std]
#![no_main]


use esp_backtrace as _;
use esp_hal::{
    delay::Delay, prelude::*
};
use s_learn::Board;


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut board = Board::new(peripherals);
    let delay = Delay::new();

    // let mut channels = get_channels(&mut board);
    loop {
        if let Some(b) = &mut board.button12 {
            if b.is_low() {
                delay.delay_millis(50);
                if b.is_low() {
                    // We play the buzzar
                    if let Some(p) = &mut board.led4 {
                        p.set_high();
                    }
                }
                while b.is_low() {
                    if let Some(p) = &mut board.led4 {
                        p.set_high();
                    }
                }
            } else {
                if let Some(p) = &mut board.led4 {
                    p.set_low();
                }
            }
        }
    }
}
