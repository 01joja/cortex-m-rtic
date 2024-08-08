//! examples/idle.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/*
Label [lis:idle_tasks]  
Caption: 
RTIC app that only runs the idle task. 
The idle task should not return and there for the application would never terminate.
*/
#[rtic::app(device = lm3s6965)]
mod app {
    use cortex_m_semihosting::debug;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        (Shared {}, Local {}, init::Monotonics())
    }

    // idle task 
    #[idle(local = [x: u32 = 0])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
            // if the line below is removed the application would run for ever.
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        }
    }
}
