//! examples/locals.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965)]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        local_to_foo: i64,
    }

    // `#[init]` cannot access locals from the `#[local]` struct as they are initialized here.
    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        (
            Shared {},
            // initial values for the `#[local]` resources
            Local {
                local_to_foo: 1,
            },
            init::Monotonics(),
        )
    }

    // `local_to_idle` can only be accessed from this context
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rtic::pend(Interrupt::UART0);
            cortex_m::asm::nop();
        }
    }

    // `local_to_foo` can only be accessed from this context
    #[task(binds = UART0, local = [local_to_foo])]
    fn foo(cx: foo::Context) {
        let local_to_foo = cx.local.local_to_foo;
        *local_to_foo += 1;
        hprintln!("foo: local_to_foo = {}", local_to_foo).unwrap();

        if local_to_foo > &mut 2{    
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        }
    }
}
