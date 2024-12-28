#![no_std]



use esp_hal::{
    analog::adc::{self, Adc, AdcConfig, AdcPin, Attenuation}, gpio::{GpioPin, Input, Level, Output, Pin}, ledc::{channel::{self, Channel}, timer::{self, Timer}, LSGlobalClkSource, Ledc, LowSpeed}, peripherals::{Peripherals, ADC1}, prelude::*, touch::{Continuous, Touch, TouchPad}, uart::{UartRx, UartTx}, Blocking
};
use core::option::Option::{self, Some};


pub struct Board {
    pub ledc: Ledc<'static>,
    pub lstimer0: Timer<'static, LowSpeed>,
    pub led15: Option<Output<'static>>,
    pub led2: Option<Output<'static>>,
    pub led0: Option<Output<'static>>,
    pub led4: Option<Output<'static>>,
    pub led32: Option<Output<'static>>,
    pub led33: Option<Output<'static>>,
    pub led27: Option<Output<'static>>,
    pub touchpad14: Option<TouchPad<GpioPin<14>, Continuous, Blocking>>,
    pub button12: Option<Input<'static>>,
    pub button13: Option<Input<'static>>,
    pub tx1: Option<UartTx<'static, Blocking>>,
    pub rx3: Option<UartRx<'static, Blocking>>,
    pub adc1: Option<Adc<'static, ADC1>>,
    pub adc1_pin36: Option<AdcPin<GpioPin<36>, ADC1>>,
    pub adc1_pin39: Option<AdcPin<GpioPin<39>, ADC1>>,
    pub adc1_pin34: Option<AdcPin<GpioPin<34>, ADC1>>,
}


impl Board {
    pub fn new(peripherals: Peripherals) -> Self {
        let mut ledc = Ledc::new(peripherals.LEDC);
        ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
        
        let led15 = Some(Output::new(peripherals.GPIO15.degrade(), Level::Low));
        let led2 = Some(Output::new(peripherals.GPIO2.degrade(), Level::Low));
        let led0 = Some(Output::new(peripherals.GPIO0.degrade(), Level::Low));
        let led4 = Some(Output::new(peripherals.GPIO4.degrade(), Level::Low));
        let led32 = Some(Output::new(peripherals.GPIO32.degrade(), Level::Low));
        let led33 = Some(Output::new(peripherals.GPIO33.degrade(), Level::Low));
        let led27 = Some(Output::new(peripherals.GPIO27.degrade(), Level::Low));

        let button12 = Some(Input::new(peripherals.GPIO12.degrade(), esp_hal::gpio::Pull::Up));
        let button13 = Some(Input::new(peripherals.GPIO13.degrade(), esp_hal::gpio::Pull::Up));
        // Set LED GPIOs as an output:
        // Setting up timer for ledc for pwm
        let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
        lstimer0
            .configure(timer::config::Config {
                duty: timer::config::Duty::Duty5Bit,
                clock_source: timer::LSClockSource::APBClk,
                frequency: 24.kHz(),
            })
            .unwrap();
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
        let touchpad14 = Some(TouchPad::new(peripherals.GPIO14, &touch));
        

        Board {ledc, lstimer0, led15, led2, led0, led4, led32, led33, led27, touchpad14, button12, button13, tx1, rx3, adc1, adc1_pin36, adc1_pin39, adc1_pin34}
        
    }
}



fn get_channels<'a>(board: &'a mut Board) -> [Channel<'a, LowSpeed>; 7] {
    let mut pin15_channel0 = board.ledc.channel(channel::Number::Channel0, board.led15.take().unwrap());
    pin15_channel0
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin2_channel1 = board.ledc.channel(channel::Number::Channel1, board.led2.take().unwrap());
    pin2_channel1
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin0_channel2 = board.ledc.channel(channel::Number::Channel2, board.led0.take().unwrap());
    pin0_channel2
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin4_channel3 = board.ledc.channel(channel::Number::Channel3, board.led4.take().unwrap());
    pin4_channel3
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin32_channel4 = board.ledc.channel(channel::Number::Channel4, board.led32.take().unwrap());
    pin32_channel4
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin33_channel5 = board.ledc.channel(channel::Number::Channel5, board.led33.take().unwrap());
    pin33_channel5
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    let mut pin27_channel6 = board.ledc.channel(channel::Number::Channel6, board.led27.take().unwrap());
    pin27_channel6
        .configure(channel::config::Config {
            timer: &board.lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    // let mut pin14_channel7 = board.ledc.channel(channel::Number::Channel7, board.led14.take().unwrap());
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