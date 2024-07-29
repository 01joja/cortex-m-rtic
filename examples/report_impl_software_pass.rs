//! examples/spawn.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, dispatchers = [SSI0], compiler_passes = [software])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        bar::spawn(1).unwrap();
        foo::spawn().unwrap();
        bar::spawn(2).unwrap();
        (Shared {}, Local {}, init::Monotonics())
    }

    #[task(priority = 1)]
    fn foo(_: foo::Context) {
        hprintln!("foo").unwrap();
    } 
    
    #[task(priority = 1, capacity = 2)]
    fn bar(_: bar::Context, x: u32) {
        hprintln!("bar {}", x).unwrap();
        if x == 2{
            hprintln!("stop").unwrap();
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        }
    }
}
