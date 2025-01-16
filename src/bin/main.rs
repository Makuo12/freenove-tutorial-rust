#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay, gpio::{Input, Level, Output, OutputOpenDrain, Pull}, prelude::*, rtc_cntl::Rtc};
use esp_println::println;
use keypad2::keypad_4x4::Keypad4x4;
// use keypad2::Keypad as KeypadLib;
// use s_learn::{board::Board, keyboard::Keypad};


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    // columns
    let mut col_one = OutputOpenDrain::new(peripherals.GPIO15.degrade(), Level::High, Pull::None);
    let mut col_two = OutputOpenDrain::new(peripherals.GPIO2.degrade(), Level::High, Pull::None);
    let mut col_three = OutputOpenDrain::new(peripherals.GPIO0.degrade(), Level::High, Pull::None);
    let mut col_four = OutputOpenDrain::new(peripherals.GPIO4.degrade(), Level::High, Pull::None);
    // rows
    let row_five = Input::new(peripherals.GPIO12.degrade(), Pull::Up);
    let row_six = Input::new(peripherals.GPIO13.degrade(), Pull::Up);
    let row_seven = Input::new(peripherals.GPIO33.degrade(), Pull::Up);
    let row_eight = Input::new(peripherals.GPIO27.degrade(), Pull::Up);
    let rows = (
        row_eight,
        row_seven,
        row_six,
        row_five
    );
    
    let cols = (
        col_four,
        col_three,
        col_two,
        col_one

    );
    let mut keypad = Keypad4x4::new(rows, cols);
    let mut delay = Delay::new();
    // let state = false;
    let mut character = ' ';
    loop {
        let key = keypad.read_char(&mut delay);
        if key != ' ' {
            if key != character {
                println!("{}", key);
                character = key;
            }
        } else {
            character = ' ';
        }
    }

}
