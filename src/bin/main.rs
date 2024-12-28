#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, ledc::{LEDC, LSGlobalClkSource, LowSpeed, timer, channel::{self, config::PinConfig}}, IO, gpio::{Gpio3, Output, PushPull}};
use log::info;
use esp_alloc::EspHeap;


// #[global_allocator]
// static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

// fn init_heap() {
//     const HEAP_SIZE: usize = 32 * 1024;
//     static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

//     unsafe {
//         ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
//     }
// }


// #[entry]
// fn main() -> ! {
//     init_heap();
//     esp_println::logger::init_logger_from_env();
//     log::info!("Logger is setup");

//     let peripherals = Peripherals::take();
//     let system = peripherals.SYSTEM.split();

//     let clocks = ClockControl::max(system.clock_control).freeze();

//     let io = IO::new(peripherals.GPIO,peripherals.IO_MUX);

//     let mut pin = io.pins.gpio19.into_push_pull_output();
//     let mut delay = Delay::new(&clocks);
//     loop {
//         info!("1000 ms duty");
//         for _ in 0..50 {
//             pin.set_high().unwrap();
//             delay.delay_us(1000_u32);
//             pin.set_low().unwrap();
//             delay.delay_us(19000_u32);
//         }
//         info!("2000 ms duty");
//         for _ in 0..100 {
//             pin.set_high().unwrap();
//             delay.delay_us(2000_u32);
//             pin.set_low().unwrap();
//             delay.delay_us(18000_u32);
//         }

//     }
// }


#[entry]
fn main() -> ! {
    // init_heap();
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO,peripherals.IO_MUX);

    let red_pin = io.pins.gpio0.into_push_pull_output();

    let mut ledc = LEDC::new(peripherals.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    let mut delay = Delay::new(&clocks);
    
    let mut lstimer0 = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
    .configure(timer::config::Config {
        duty: timer::config::Duty::Duty14Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: 50_u32.Hz(),
    })
    .unwrap();

    let mut channel0 = ledc.get_channel(channel::Number::Channel0, red_pin);
    channel0
    .configure(hal::ledc::channel::config::Config {
        timer: &lstimer0,
        duty_pct: 0,
        pin_config: PinConfig::PushPull,
    })
    .unwrap();
// 50Hz 1000 / 50 = 20ms / 
    // channel0.set_duty_hw(127); // Set duty to 50%
    channel0.set_duty(50).unwrap(); // Set duty to 50%
    let mut servo: Servo<'_, hal::gpio::GpioPin<Output<PushPull>, 0>, 1000, 2000,14,50> = Servo::new(channel0);
    loop {
        info!("Set to 0:");
        servo.set_percentage(0);
        delay.delay_ms(2000u32);
        info!("Set to 50:");
        servo.set_percentage(50);
        delay.delay_ms(2000u32);
        info!("Set to 100:");
        servo.set_percentage(100);
        delay.delay_ms(1000u32);
    
        println!("Loop...");
    }
}

use hal::{gpio::OutputPin, ledc::channel::{Channel, ChannelHW}};

pub struct Servo<'a, P: OutputPin, const MIN_DUTY: u32, const MAX_DUTY: u32, const BITS: u32, const FREQUENCY: u32> {
    channel: Channel<'a,LowSpeed,P>,
}

impl <'a, P: OutputPin,const MIN_DUTY: u32, const MAX_DUTY: u32, const BITS: u32, const FREQUENCY: u32> 
    Servo<'a, P, MIN_DUTY, MAX_DUTY, BITS,FREQUENCY> {
    // const POW: u32 = 2_u32.pow(BITS);
    const CYCLE_TIME: u32 = 1000000 / FREQUENCY;
    const DUTY_SPACE: u32 = 2_u32.pow(BITS);
    pub fn new(channel: Channel<'a,LowSpeed,P>)->Self {
        Servo { channel }
    }

    pub fn set_percentage(&mut self, percentage: u8) {
        let range: u32 = MAX_DUTY - MIN_DUTY;
        info!("Range: {}",range);
        let abs_duty = MIN_DUTY + (range * percentage as u32 / 100); // in micros
        info!("Setting to aba duty: {}",abs_duty);
        
        let duty = abs_duty * Self::DUTY_SPACE / Self::CYCLE_TIME;
        info!("Setting to duty: {}",duty);
        self.channel.set_duty_hw(duty);
    }
}