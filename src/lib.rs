#![no_std]

pub mod board;
pub mod keyboard;


use esp_hal::{
    analog::adc::{Adc, AdcConfig, AdcPin, Attenuation}, 
    gpio::{GpioPin, Input, Level, Output, Pin}, 
    i2c::master::{Config, I2c}, 
    ledc::{channel::{self, Channel}, timer::{self, Timer as LCD_Timer}, LSGlobalClkSource, Ledc, LowSpeed}, 
    peripherals::{Peripherals, ADC1, LPWR}, 
    prelude::*, timer::timg::{Timer, TimerGroup}, touch::{Continuous, Touch, TouchPad}, 
    uart::{UartRx, UartTx}, Blocking
};
use core::option::Option::{self, Some};

// For better understanding
// led fields represent pins that have been set for output;
// button fields represent pins that have been set for input

pub struct Board {
    pub ledc: Ledc<'static>,
    pub lstimer0: LCD_Timer<'static, LowSpeed>,
    pub pin15: Option<Output<'static>>,
    pub pin2: Option<Output<'static>>,
    pub pin0: Option<Output<'static>>,
    pub pin4: Option<Output<'static>>,
    pub pin32: Option<Output<'static>>,
    pub pin33: Option<Output<'static>>,
    pub pin27: Option<Output<'static>>,
    pub touch_pin14: Option<TouchPad<GpioPin<14>, Continuous, Blocking>>,
    pub in_pin12: Option<Input<'static>>,
    pub in_pin13: Option<Input<'static>>,
    pub tx1: Option<UartTx<'static, Blocking>>,
    pub rx3: Option<UartRx<'static, Blocking>>,
    pub adc1: Option<Adc<'static, ADC1>>,
    pub adc1_pin36: Option<AdcPin<GpioPin<36>, ADC1>>,
    pub adc1_pin39: Option<AdcPin<GpioPin<39>, ADC1>>,
    pub adc1_pin34: Option<AdcPin<GpioPin<34>, ADC1>>,
    pub blocking_i2c: Option<I2c<'static, Blocking>>,
    pub lpwr: LPWR
}
// Device I2c addresses

impl Board {
    pub fn new(peripherals: Peripherals) -> Self {
        let mut ledc = Ledc::new(peripherals.LEDC);
        ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
        
        let pin15 = Some(Output::new(peripherals.GPIO15.degrade(), Level::Low));
        let pin2 = Some(Output::new(peripherals.GPIO2.degrade(), Level::Low));
        let pin0 = Some(Output::new(peripherals.GPIO0.degrade(), Level::Low));
        let pin4 = Some(Output::new(peripherals.GPIO4.degrade(), Level::Low));
        let pin32 = Some(Output::new(peripherals.GPIO32.degrade(), Level::Low));
        let pin33 = Some(Output::new(peripherals.GPIO33.degrade(), Level::Low));
        let pin27 = Some(Output::new(peripherals.GPIO27.degrade(), Level::Low));

        let in_pin12 = Some(Input::new(peripherals.GPIO12.degrade(), esp_hal::gpio::Pull::Up));
        let in_pin13 = Some(Input::new(peripherals.GPIO13.degrade(), esp_hal::gpio::Pull::Up));
        // Set LED GPIOs as an output:
        // Setting up timer for ledc for pwm
        let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
        lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty14Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 50_u32.Hz(),
        })
        .unwrap();
        // I2C
        let blocking_i2c = Some(I2c::new(
            peripherals.I2C0, Config::default())
            .with_scl(peripherals.GPIO22)
            .with_sda(peripherals.GPIO21));


        let tx1 = Some(UartTx::new(peripherals.UART0, peripherals.GPIO1.degrade()).unwrap());
        let rx3 = Some(UartRx::new(peripherals.UART1, peripherals.GPIO3.degrade()).unwrap());
        // Setting up adc for analog digital converter
        let mut adc_config = AdcConfig::new();
        let adc1_pin36 = Some(adc_config.enable_pin(peripherals.GPIO36, Attenuation::Attenuation11dB));
        let adc1_pin39 = Some(adc_config.enable_pin(peripherals.GPIO39, Attenuation::Attenuation11dB));
        let adc1_pin34= Some(adc_config.enable_pin(peripherals.GPIO34, Attenuation::Attenuation11dB));
        let adc1 = Some(Adc::new(peripherals.ADC1, adc_config));
        
        // Setting up touch
        let touch = Touch::continuous_mode(peripherals.TOUCH, None);
        let touch_pin14 = Some(TouchPad::new(peripherals.GPIO14, &touch));
        

        Board {
            ledc, lstimer0, pin15, pin2, pin0, 
            pin4, pin32, pin33, pin27, touch_pin14, 
            in_pin12, in_pin13, tx1, rx3, adc1, 
            adc1_pin36, adc1_pin39, adc1_pin34, 
            blocking_i2c, lpwr: peripherals.LPWR}
        
    }
}



fn get_channels<'a>(board: &'a mut Board) -> [Channel<'a, LowSpeed>; 7] {
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
    
    // let mut pin14_channel7 = board.ledc.channel(channel::Number::Channel7, board.pin14.take().unwrap());
    // pin14_channel7
    //     .configure(channel::config::Config {
    //         timer: &board.lstimer0,
    //         duty_pct: 10,
    //         pin_config: channel::config::PinConfig::PushPull,
    //     })
    //     .unwrap();
    
    let channels:[Channel<'a, LowSpeed>; 7] = [
        pin15_channel0,
        pin2_channel1,
        pin0_channel2,
        pin4_channel3,
        pin32_channel4,
        pin33_channel5,
        pin27_channel6,
        // pin14_channel7
    ];
    return channels;
}


pub struct Servo<'a, const MIN_DUTY: u32, const MAX_DUTY: u32, const BITS: u32, const FREQUENCY: u32> {
    channel: Channel<'a, LowSpeed>
}

impl<'a, const MIN_DUTY: u32, const MAX_DUTY: u32, const BITS: u32, const FREQUENCY: u32> Servo <'a, MIN_DUTY, MAX_DUTY, BITS, FREQUENCY> {
    const CYCLE_TIME: u32 = 1_000_000 / FREQUENCY;
    const DUTY_SPACE: u32 = 2_u32.pow(BITS);
    pub fn new(channel: Channel<'a, LowSpeed>)->Self {
        Servo { channel }
    }

    pub fn set_percentage(&mut self, percentage: u8) {
        let range: u32 = MAX_DUTY - MIN_DUTY;
        let abs_duty = MIN_DUTY + (range * percentage as u32 / 100); // in micros
        let duty = abs_duty * Self::DUTY_SPACE / Self::CYCLE_TIME;
        // esp_println::println!("duty {}", duty);
        self.channel.set_duty_hw(duty);
    }
}


