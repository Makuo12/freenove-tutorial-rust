#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::{self, Delay},
    gpio::{Output, OutputPin},
    ledc::channel,
    prelude::*,
};
use lcd1602_diver::LCD1602;
use lcd_1602_i2c::{Cursor, Lcd};
use s_learn::{Board, Servo};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let board = Board::new(peripherals);
    let mut delay = Delay::new();
    let mut lcd = LCD1602::new_i2c(board.blocking_i2c.unwrap(), LCD_ADDRESS, &mut delay).unwrap();
    lcd.write_byte(b'F', &mut delay).unwrap();
    lcd.write_byte(b'o', &mut delay).unwrap();
    lcd.write_byte(b'r', &mut delay).unwrap();
    lcd.write_byte(b'm', &mut delay).unwrap();
    lcd.write_byte(b' ', &mut delay).unwrap();
    lcd.write_byte(b'M', &mut delay).unwrap();
    lcd.write_byte(b'a', &mut delay).unwrap();
    lcd.write_byte(b'k', &mut delay).unwrap();
    lcd.write_byte(b'u', &mut delay).unwrap();
    lcd.write_byte(b'o', &mut delay).unwrap();
    lcd.set_cursor_pos(40, &mut delay);
    lcd.write_byte(b'H', &mut delay).unwrap();
    lcd.write_byte(b'e', &mut delay).unwrap();
    lcd.write_byte(b'l', &mut delay).unwrap();
    lcd.write_byte(b'l', &mut delay).unwrap();
    lcd.write_byte(b'o', &mut delay).unwrap();
    lcd.write_byte(b' ', &mut delay).unwrap();
    lcd.write_byte(b'W', &mut delay).unwrap();
    lcd.write_byte(b'o', &mut delay).unwrap();
    lcd.write_byte(b'r', &mut delay).unwrap();
    lcd.write_byte(b'l', &mut delay).unwrap();
    lcd.write_byte(b'd', &mut delay).unwrap();
    // for address in 0x03..0x7F  {
    //     if let Some(i2c) = &mut board.blocking_i2c {
    //         let result = i2c.write(address, &[]); // Try to write with an empty buffer
    //         if result.is_ok() {
    //             esp_println::println!("Found device at address: 0x{:X}", address);
    //         } else {

    //             esp_println::println!("Not Found device at address: 0x{:X}", address);
    //         }

    //     }
    // }
    loop {
        // lcd.set_rgb(255, 255, 255).unwrap();
        // lcd.set_cursor(Cursor::On).unwrap();
        // lcd.write_str("Hello world!").unwrap();
    }
}

const LCD_ADDRESS: u8 = 0x27;
const RGB_ADDRESS: u8 = 0x62;
