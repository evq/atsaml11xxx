#![no_main]
#![no_std]
extern crate atsaml11xxx;

extern crate cortex_m;
extern crate cortex_m_rt;
// makes `panic!` print messages to the host stderr using semihosting
extern crate panic_semihosting;
use cortex_m_rt::entry;

pub use cortex_m::asm::{bkpt, nop};
// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    let peripherals = atsaml11xxx::Peripherals::take().unwrap();
    unsafe {
        peripherals.PORT_SEC.group0.dirset.write(|w| w.bits(1 << 7));
        loop {
            peripherals.PORT_SEC.group0.outclr.write(|w| w.bits(1 << 7));

            for _x in 0..100000 {
                nop();
            }

            peripherals.PORT_SEC.group0.outset.write(|w| w.bits(1 << 7));

            for _x in 0..100000 {
                nop();
            }
        }
    }
}
