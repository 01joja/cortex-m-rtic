//! examples/hardware.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, compiler_passes = [standard])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        // Pends the UART0 interrupt but its handler won't run until *after*
        // `init` returns because interrupts are disabled
        rtic::pend(Interrupt::UART0);
        rtic::pend(Interrupt::UART1); // equivalent to NVIC::pend

        hprintln!("init").unwrap();

        (Shared {}, Local {}, init::Monotonics())
    }


    #[task(binds = UART0)]
    fn foo(_: foo::Context) {

        hprintln!("foo").unwrap();

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    #[task(binds = UART1, priority = 3)]
    fn bar(_: bar::Context) {

        hprintln!("bar").unwrap();
    }
}
