#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use nrf9160_hal::gpio::Level;
use nrf9160_hal::{
    //clocks::Clocks,
    gpio,
    pac::{Peripherals},
    prelude::*,
};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    //let _clocks = Clocks::new(p.CLOCK_NS).enable_ext_hfosc();
    //let timer = Timer::new(p.TIMER1_NS);
    let port0 = gpio::p0::Parts::new(p.P0_NS);

    let mut red_led = port0.p0_10.into_push_pull_output(Level::Low);

    // loops forever toggling the red_led
    loop {
        match red_led.is_set_high().unwrap() {
            true => {
                red_led.set_low().unwrap();
            }

            false => {
                red_led.set_high().unwrap();
            }
        }

        delay(1_000_000);
    }
}
