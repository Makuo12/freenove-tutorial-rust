#![no_std]
#![no_main]

use core::borrow::BorrowMut;

use esp_backtrace as _;
use esp_hal::{
    delay::Delay, gpio::{Input, Level, Output, Pin, Pull}, 
    ledc::{channel::{self, Channel, Number}, timer::{self, Timer, TimerSpeed}, HighSpeed, LSGlobalClkSource, Ledc, LowSpeed}, 
    mcpwm::{self, operator::{PwmPin, PwmPinConfig}, timer::PwmWorkingMode, McPwm, PeripheralClockConfig}, 
    peripheral, peripherals::{Peripherals, MCPWM0}, prelude::*
};

struct Board {
    ledc: Ledc<'static>,
    lstimer0: Timer<'static, LowSpeed>,
    led15: Option<Output<'static>>,
    led2: Option<Output<'static>>,
    led0: Option<Output<'static>>,
    led4: Option<Output<'static>>,
    led32: Option<Output<'static>>,
    led33: Option<Output<'static>>,
    led27: Option<Output<'static>>,
    led14: Option<Output<'static>>,
}

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let clock_cfg = PeripheralClockConfig::with_frequency(32.MHz()).unwrap();
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    
    let led15 = Some(Output::new(peripherals.GPIO15.degrade(), Level::Low));
    let led2 = Some(Output::new(peripherals.GPIO2.degrade(), Level::Low));
    let led0 = Some(Output::new(peripherals.GPIO0.degrade(), Level::Low));
    let led4 = Some(Output::new(peripherals.GPIO4.degrade(), Level::Low));
    let led32 = Some(Output::new(peripherals.GPIO32.degrade(), Level::Low));
    let led33 = Some(Output::new(peripherals.GPIO33.degrade(), Level::Low));
    let led27 = Some(Output::new(peripherals.GPIO27.degrade(), Level::Low));
    let led14 = Some(Output::new(peripherals.GPIO14.degrade(), Level::Low));

    
    // Set LED GPIOs as an output:
    
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty5Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 24.kHz(),
        })
        .unwrap();
    let mut board = Board {ledc, lstimer0, led15, led2, led0, led4, led32, led33, led27, led14};
        
    
    
    let delay = Delay::new();

    let mut channels = get_channels(&mut board);
    loop {
        for c in &mut channels {
            for i in 0..100 {
                c.set_duty(i).expect("could not set duty");
                delay.delay_millis(200);
            }
            

        }
        for c in &mut channels {
            for i in 0..100 {
                c.set_duty(i).expect("could not set duty");
                delay.delay_millis(200);
            }

        }
    }
}

fn get_channels<'a>(board: &'a mut Board) -> [Channel<'a, LowSpeed>; 8] {
    let mut pin15_channel0 = board.ledc.channel(channel::Number::Channel0, board.pin15.take().unwrap());
    pin15_channel0
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin2_channel1 = board.ledc.channel(channel::Number::Channel1, board.pin2.take().unwrap());
    pin2_channel1
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin0_channel2 = board.ledc.channel(channel::Number::Channel2, board.pin0.take().unwrap());
    pin0_channel2
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin4_channel3 = board.ledc.channel(channel::Number::Channel3, board.pin4.take().unwrap());
    pin4_channel3
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin32_channel4 = board.ledc.channel(channel::Number::Channel4, board.pin32.take().unwrap());
    pin32_channel4
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin33_channel5 = board.ledc.channel(channel::Number::Channel5, board.pin33.take().unwrap());
    pin33_channel5
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin27_channel6 = board.ledc.channel(channel::Number::Channel6, board.pin27.take().unwrap());
    pin27_channel6
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin14_channel7 = board.ledc.channel(channel::Number::Channel7, board.pin14.take().unwrap());
    pin14_channel7
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let channels:[Channel<'a, LowSpeed>; 8] = [
        pin15_channel0,
        pin2_channel1,
        pin0_channel2,
        pin4_channel3,
        pin32_channel4,
        pin33_channel5,
        pin27_channel6,
        pin14_channel7
    ];
    return channels;
}