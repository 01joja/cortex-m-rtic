//! examples/lock.rs

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
    struct Shared {
        a_shared: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtic::pend(Interrupt::GPIOA); // pends foo
        (Shared { a_shared: 0 }, Local {}, init::Monotonics())
    }

    #[task(binds = GPIOA, priority = 1, shared = [a_shared] )]
    fn foo(mut c: foo::Context) {
        hprintln!("A").unwrap();
        c.shared.a_shared.lock(|a_shared| {
            rtic::pend(Interrupt::GPIOB); 
            *a_shared += 1; 
            hprintln!("B - shared {}", *a_shared).unwrap();
            rtic::pend(Interrupt::GPIOC); 
        });
        hprintln!("D").unwrap();
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    #[task(binds = GPIOB, priority = 2, shared = [a_shared])]
    fn bar(mut c: bar::Context) {
        // the higher priority task does still need a critical section
        let shared = c.shared.a_shared.lock(|a_shared| {
            *a_shared += 1;
            *a_shared
        });
        hprintln!("C - shared {}", shared).unwrap();
    }
}
