#![no_std]
pub mod led_controller;

pub extern crate l3gd20;
pub extern crate lsm303dlhc;
pub extern crate stm32f4xx_hal;

use stm32f4xx_hal::spi::Spi;
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::gpio::{
    Alternate,
    Output,
    PushPull,
    AF5,
    AF4,
    gpioa::{ PA5, PA6, PA7 },
    gpiob::{ PB6, PB9 },
    gpioe::PE3
};
use stm32f4xx_hal::stm32::{
    SPI1,
    I2C1,
};

/// Board-specific implementation of the L3GD20 Gyroscope
/// 
/// # Example
/// 
/// ```
/// #![no_std]
/// #![no_main]
/// #![feature(alloc_error_handler)]
/// 
/// extern crate panic_halt;
/// 
/// use stm32f411_disco::L3gd20;
/// 
/// use core::alloc::Layout;
/// use alloc_cortex_m::CortexMHeap;
/// 
/// use cortex_m_rt::entry;
/// use cortex_m::asm;
/// 
/// use stm32f4xx_hal::{
///     prelude::*,
///     stm32,
///     spi::Spi,
/// };
/// 
/// use embedded_hal::digital::v2::OutputPin;
/// 
/// #[global_allocator]
/// static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
/// 
/// const HEAP_SIZE: usize = 1024; // in bytes
/// 
/// #[entry]
/// fn main() -> ! {
///     // Set up allocator
///     unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
/// 
///     let p = stm32::Peripherals::take().unwrap();
///     let rcc = p.RCC.constrain();
///     let clocks = rcc.cfgr.freeze();
/// 
///     let gpioa = p.GPIOA.split();
///     let gpioe = p.GPIOE.split();
/// 
///     let mut nss = gpioe.pe3.into_push_pull_output();
///     nss.set_high().ok();
/// 
///     let sck  = gpioa.pa5.into_alternate_af5();
///     let miso = gpioa.pa6.into_alternate_af5();
///     let mosi = gpioa.pa7.into_alternate_af5();
/// 
///     let spi = Spi::spi1(p.SPI1, (sck, miso, mosi), l3gd20::MODE, 1_000_000.hz(), clocks);
///     let mut l3gd20 = L3gd20::new(spi, nss).unwrap();
/// 
///     if let Ok(i) = l3gd20.who_am_i() {
///         if i != 0xD4 {
///             loop {} // Error
///         }
///     }
/// 
///     loop {}
/// }
/// 
/// // define what happens in an Out Of Memory (OOM) condition
/// #[alloc_error_handler]
/// fn alloc_error(_layout: Layout) -> ! {
///     asm::bkpt();
/// 
///     loop {}
/// }
/// ```
pub type L3gd20 = l3gd20::L3gd20<
    Spi<
        SPI1, 
        (
            PA5<Alternate<AF5>>, 
            PA6<Alternate<AF5>>, 
            PA7<Alternate<AF5>>
        )
    >, 
    PE3<Output<PushPull>>
>;

/// Board-specific implementation of the LSM303DLHC Accelerometer + Magnetometer
/// 
/// # Example
/// 
/// ```
/// #![no_std]
/// #![no_main]
/// 
/// extern crate panic_halt;
/// 
/// use stm32f411_disco::Lsm303dlhc;
/// 
/// use cortex_m_rt::entry;
/// 
/// use stm32f4xx_hal::{
///     prelude::*,
///     stm32,
///     i2c::I2c,
/// };
/// 
/// #[entry]
/// fn main() -> ! {
///     let p = stm32::Peripherals::take().unwrap();
///     let rcc = p.RCC.constrain();
///     let clocks = rcc.cfgr.freeze();
/// 
///     let gpiob = p.GPIOB.split();
///     let scl = gpiob.pb6.into_alternate_af4();
///     let sda = gpiob.pb9.into_alternate_af4();
/// 
///     let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks);
///     let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();
/// 
///     let _accel = lsm303dlhc.accel().unwrap();
///     let _mag   = lsm303dlhc.mag().unwrap();
///     let _temp  = lsm303dlhc.temp().unwrap();
/// 
///     loop {}
/// }
/// ```
pub type Lsm303dlhc = lsm303dlhc::Lsm303dlhc<
    I2c<
        I2C1,
        (
            PB6<Alternate<AF4>>,
            PB9<Alternate<AF4>>,
        )
    >
>;