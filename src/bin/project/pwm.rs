#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay, gpio::{Input, Level, Output, Pin, Pull}, mcpwm::{operator::{PwmPin, PwmPinConfig}, timer::PwmWorkingMode, McPwm, PeripheralClockConfig}, prelude::*
};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let clock_cfg = PeripheralClockConfig::with_frequency(32.MHz()).unwrap();
    let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);
    let pin = Output::new(peripherals.GPIO4, Level::Low);
        // connect operator0 to timer0
    mcpwm.operator0.set_timer(&mcpwm.timer0);
    // connect operator0 to pin
    let mut pwm_pin = mcpwm
        .operator0
        .with_pin_a(pin, PwmPinConfig::UP_ACTIVE_HIGH);
    // start timer with timestamp values in the range of 0..=99 and a frequency
    // of 20 kHz
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(99, PwmWorkingMode::Increase, 20.kHz())
        .unwrap();
    mcpwm.timer0.start(timer_clock_cfg);
    let delay = Delay::new();
    // pin will be high 50% of the time
    
    loop {
        let mut i = 0;
        loop {
            pwm_pin.set_timestamp(i);
            delay.delay_millis(20);        
            if i == 100 {
                break;
            }
            i += 1
        }
        loop {
            esp_println::println!("going out");
            pwm_pin.set_timestamp(i);
            delay.delay_millis(20);        
            if i == 0 {
                break;
            }
            i -= 1
        }
    }
}

