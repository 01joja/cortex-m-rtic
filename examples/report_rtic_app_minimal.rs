//! examples/smallest.rs

#![no_main]
#![no_std]

use panic_semihosting as _; // panic handler


/*
Lable:
list:app_minimal
Caption:
The smallest RTIC application possible.
It declares the target device, has shared and local resources (see section X), and has a init task (see section X).
This can be executed on a Linux or Windows that has the QEMU simulator installed.
Prints "Hello world!" and then exists the QEMU simulator.

Check peripherals 
*/
// The RTIC application will target the lm3s6965 as the device.
#[rtic::app(device = lm3s6965, peripherals = true)]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;

    // Shared resources
    #[shared]
    struct Shared {}

    // Local resources
    #[local]
    struct Local {}

    // init task
    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("Hello world!").unwrap();
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        (Shared {}, Local {}, init::Monotonics())
    }
}
