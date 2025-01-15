#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay, gpio::{Input, Level, Output, OutputOpenDrain, Pull}, prelude::*, rtc_cntl::Rtc};
use esp_println::println;
use s_learn::board::Board;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    // columns
    // let one = board.pin15.unwrap();
    // let two = board.pin2.unwrap();
    // let three = board.pin0.unwrap();
    // let four = board.pin4.unwrap();
    // // rows
    // let five = board.in_pin12.unwrap();
    // let six = board.in_pin13.unwrap();
    // let seven = board.in_pin33.unwrap();
    // let eight = board.in_pin27.unwrap();
    let mut one = OutputOpenDrain::new(peripherals.GPIO15.degrade(), Level::High, Pull::None);
    let mut two = OutputOpenDrain::new(peripherals.GPIO2.degrade(), Level::High, Pull::None);
    let mut three = OutputOpenDrain::new(peripherals.GPIO0.degrade(), Level::High, Pull::None);
    let mut four = OutputOpenDrain::new(peripherals.GPIO4.degrade(), Level::High, Pull::None);
    let five = Input::new(peripherals.GPIO12.degrade(), Pull::Up);
    //rows
    let six = Input::new(peripherals.GPIO13.degrade(), Pull::Up);
    let seven = Input::new(peripherals.GPIO33.degrade(), Pull::Up);
    let eight = Input::new(peripherals.GPIO27.degrade(), Pull::Up);
    let delay = Delay::new();
    loop {
        one.set_low();
        if five.is_low() {
            if five.is_low() {
                println!("D");
                while five.is_low() {}
                continue;
            }
        }
        if six.is_low() {
            if six.is_low() {
                println!("C");
                while six.is_low() {}
                continue;
            }
        }
        if seven.is_low() {
            if seven.is_low() {
                println!("B");
                while seven.is_low() {}
                continue;
            }
        }
        if eight.is_low() {
            if eight.is_low() {
                println!("A");
                while eight.is_low() {}
                continue;
            }
        }
        one.set_high(); 
        delay.delay_millis(100);
        two.set_low();
        if five.is_low() {
            if five.is_low() {
                println!("#");
                while five.is_low() {}
                continue;
            }
        }
        if six.is_low() {
            if six.is_low() {
                println!("9");
                while six.is_low() {}
                continue;
            }
        }
        if seven.is_low() {
            if seven.is_low() {
                println!("6");
                while seven.is_low() {}
                continue;
            }
        }
        if eight.is_low() {
            if eight.is_low() {
                println!("3");
                while eight.is_low() {}
                continue;
            }
        }
        two.set_high();
        three.set_low();
        if five.is_low() {
            if five.is_low() {
                println!("0");
                while five.is_low() {}
                continue;
            }
        }
        if six.is_low() {
            if six.is_low() {
                println!("8");
                while six.is_low() {}
                continue;
            }
        }
        if seven.is_low() {
            if seven.is_low() {
                println!("5");
                while seven.is_low() {}
                continue;
            }
        }
        if eight.is_low() {
            if eight.is_low() {
                println!("2");
                while eight.is_low() {}
                continue;
            }
        }
        three.set_high();
        four.set_low();
        // Row 4 scanning
        if five.is_low() {
            if five.is_low() {
                println!("*");
                while five.is_low() {}
                continue;
            }
        }
        if six.is_low() {
            if six.is_low() {
                println!("7");
                while six.is_low() {}
                continue;
            }
        }
        if seven.is_low() {
            delay.delay_millis(10);
            if seven.is_low() {
                println!("4");
                while seven.is_low() {}
                continue;
            }
        }
        if eight.is_low() {
            delay.delay_millis(10);
            if eight.is_low() {
                println!("1");
                while eight.is_low() {}
                continue;
            }
        }
        four.set_low();
    }
}
