
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
    
    let mut pin15_channel0 = board.ledc.channel(channel::Number::Channel0, board.pin15.take().unwrap());
    pin15_channel0
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 100,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    let mut delay = Delay::new();
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
                    let voltage = (r as f32 / 4095.0) * 3.3; //Voltage
                    let rt = (10.0 * voltage) / (3.3 - voltage); // calculate the resistance value of thermistor
                    let tempk = 1.0 / (1.0 / (273.15 + 25.0) + (rt / 10.0).log(2.0) / 3950.0); //cal temp in kelvin 
                    let tempc = tempk - 273.15;
                    esp_println::println!("ADC value: {}, \t Voltage: {}, \t temp: {}", r, voltage, tempc);
                    delay.delay_millis(1000);
                },
                Err(_) => {
                }
            };
        }
    }
}
