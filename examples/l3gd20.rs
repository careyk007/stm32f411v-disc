#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate panic_halt;

use stm32f411_disco::L3gd20;

use core::alloc::Layout;
use alloc_cortex_m::CortexMHeap;

use cortex_m_rt::entry;
use cortex_m::asm;

use stm32f4xx_hal::{
    prelude::*,
    stm32,
    spi::Spi,
};

use embedded_hal::digital::v2::OutputPin;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    // Set up allocator
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let p = stm32::Peripherals::take().unwrap();
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = p.GPIOA.split();
    let gpioe = p.GPIOE.split();

    let mut nss = gpioe.pe3.into_push_pull_output();
    nss.set_high().ok();

    let sck  = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();

    let spi = Spi::spi1(p.SPI1, (sck, miso, mosi), l3gd20::MODE, 1_000_000.hz(), clocks);
    let mut l3gd20 = L3gd20::new(spi, nss).unwrap();

    if let Ok(i) = l3gd20.who_am_i() {
        if i != 0xD4 {
            loop {} // Error
        }
    }

    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}
