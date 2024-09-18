//! examples/hardware.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, compiler_passes = [resources,hardware])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;

    #[shared]
    struct Shared {
        resource: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        // Pends the UART0 interrupt but its handler won't run until *after*
        // `init` returns because interrupts are disabled
        rtic::pend(Interrupt::UART0); // equivalent to NVIC::pend

        hprintln!("init").unwrap();

        (Shared {resource: 0}, Local {}, init::Monotonics())
    }


    #[task(binds = UART0, shared = [resource])]
    fn foo(mut c: foo::Context) {

        rtic::pend(Interrupt::UART1);
        c.shared.resource.lock(|resource| {
            rtic::pend(Interrupt::UART1);
            *resource += 1;
            hprintln!("foo {}", *resource).unwrap();
        });

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    #[task(binds = UART1, shared = [resource], priority = 3)]
    fn bar(mut c: bar::Context) {

        c.shared.resource.lock(|resource| {
            *resource += 1;
            hprintln!("bar {}", *resource).unwrap();
        });
    }
}
