#![no_std]
#![no_main]

extern crate panic_halt;

use stm32f411_disco::Lsm303dlhc;

use cortex_m_rt::entry;

use stm32f4xx_hal::{
    prelude::*,
    stm32,
    i2c::I2c,
};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpiob = p.GPIOB.split();
    let scl = gpiob.pb6.into_alternate_af4();
    let sda = gpiob.pb9.into_alternate_af4();

    let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks);
    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    let _accel = lsm303dlhc.accel().unwrap();
    let _mag   = lsm303dlhc.mag().unwrap();
    let _temp  = lsm303dlhc.temp().unwrap();

    loop {}
}