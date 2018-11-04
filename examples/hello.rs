#![no_main]
#![no_std]
extern crate atsaml11xxx;

extern crate cortex_m;
extern crate cortex_m_rt;
// makes `panic!` print messages to the host stderr using semihosting
extern crate panic_semihosting;

extern crate cortex_m_semihosting;

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    let language = "Rust";
    let ranking = 1;

    write!(stdout, "{} on embedded is #{}!", language, ranking);
    loop {}
}
