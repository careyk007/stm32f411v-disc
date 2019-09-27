#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use stm32f411v_disc::{
    led_controller::LEDController,
    l3gd20,
    L3gd20,
    Lsm303dlhc,
};

use core::alloc::Layout;

use alloc_cortex_m::CortexMHeap;

use cortex_m_rt::entry;
use cortex_m::asm;
// use cortex_m_semihosting::{hprintln};

use embedded_hal::digital::v2::OutputPin;

use stm32f4xx_hal::{
    prelude::*,
    stm32,
    timer::Timer,
    spi::Spi,
    i2c::I2c,
};

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    // Set up allocator
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
    // hprintln!("Gathering peripherals").unwrap();

    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    // Set up LED Controller
    let gpiod = p.GPIOD.split();
    let mut green_led = gpiod.pd12.into_push_pull_output();
    let mut orange_led = gpiod.pd13.into_push_pull_output();
    let mut blue_led = gpiod.pd15.into_push_pull_output();
    let mut red_led = gpiod.pd14.into_push_pull_output();

    red_led.set_high().ok();
    green_led.set_high().ok();
    orange_led.set_high().ok();
    blue_led.set_high().ok();

    let mut led_machine = LEDController::new(red_led, green_led, blue_led, orange_led);

    // Set up gyroscope
    let gpioa = p.GPIOA.split();
    let gpioe = p.GPIOE.split();

    let mut nss = gpioe.pe3.into_push_pull_output();
    nss.set_high().ok();

    let sck = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();

    let spi = Spi::spi1(p.SPI1, (sck, miso, mosi), l3gd20::MODE, 1_000_000.hz(), clocks);
    let mut l3gd20 = L3gd20::new(spi, nss).unwrap();

    // Set up accelerometer + magnetometer
    let gpiob = p.GPIOB.split();
    let scl = gpiob.pb6.into_alternate_af4();
    let sda = gpiob.pb9.into_alternate_af4();

    let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks);
    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);

    // hprintln!("Peripherals initialized, starting loop!").unwrap();

    loop {
        // your code goes here
        nb::block!(timer.wait()).unwrap();
        led_machine.advance_state();
        // Check register value for gyroscope
        if let Ok(i) = l3gd20.who_am_i() {
            if i != 0xD4 {
                loop {}
            }
        }

        let _accel = lsm303dlhc.accel().unwrap();
        let _mag   = lsm303dlhc.mag().unwrap();
        let _temp  = lsm303dlhc.temp().unwrap();

        // hprintln!("Loop").unwrap();
    }
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}
