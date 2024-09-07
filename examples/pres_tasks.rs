//! examples/init.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, peripherals = true)]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init()]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtic::pend(Interrupt::UART0); // pends foo
        hprintln!("init").unwrap();
        (Shared {}, Local {}, init::Monotonics())
    }

    #[task(binds = UART0, priority = 1)]
    fn foo(_: foo::Context){
        hprintln!("foo").unwrap();
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            hprintln!("idle").unwrap();
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
            cortex_m::asm::nop();
        }
    }
}
