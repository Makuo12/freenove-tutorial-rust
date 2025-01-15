
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay, gpio::{Input, Output}, prelude::*, rtc_cntl::Rtc};
use s_learn::Board;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut board = Board::new(peripherals);
    let mut rtc = Rtc::new(board.lpwr);
    let mut trig_pin = board.pin0.unwrap();
    let mut echo_pin = board.in_pin12.unwrap();
    let sound_velocity = 345.0;
    let mut delay = Delay::new();
    loop {
        delay.delay_millis(100);
        let result = get_sonar(&mut trig_pin, &mut echo_pin, &mut delay, &mut rtc, sound_velocity);
        if let Some(pin15) = &mut board.pin15 {
            if result < 5.0 {
                pin15.set_high();
            } else {
                pin15.set_low();
            }
        } 
    }
}

fn get_sonar(
    trig_pin: &mut Output<'_>, 
    echo_pin: &mut Input<'_>, 
    delay: &mut Delay, 
    rtc: &mut Rtc<'_>,
    sound_velocity: f64) -> f64 {
    trig_pin.set_high();
    delay.delay_micros(10);
    trig_pin.set_low();
    loop {
        if echo_pin.is_high() {
            break;
        }
    };
    let start = rtc.current_time();
    loop {
        if echo_pin.is_low() {
            break;
        }
    }
    let end = rtc.current_time();
    let ping_time = (end - start).num_microseconds().unwrap() as f64;
    let distance = ping_time*sound_velocity / 2.0 / 10000.0;
    return distance;
    

}