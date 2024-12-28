
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
    let delay = Delay::new();
    if let Some(tx) = &mut board.tx1 {
        tx.write_bytes(b"data sent").unwrap();
    }
    // let mut channels = get_channels(&mut board);
    let mut bytes = [0;100];
    loop {
        if let Some(rx) = &mut board.rx3 {
            let result = match rx.read_byte() {
                Ok(r) => r, 
                Err(e) => {
                    continue;
                }
            };

            esp_println::println!("makuo{}", from_utf8(&[result]).unwrap())
        }
        esp_println::println!("makuo{}", from_utf8(&bytes).unwrap())
    }
}