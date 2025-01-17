//! examples/hardware.rs
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, compiler_passes = [hardware])]
mod app {
    use cortex_m_semihosting::debug;
    use lm3s6965::Interrupt;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtic::pend(Interrupt::UART0);
        (Shared {}, Local {}, init::Monotonics())
    }

    #[task(binds = UART0,priority = 1)]
    fn foo(_: foo::Context) {
    }

    #[__rtic_task_module(has_context = false, has_monotonic = false)]
    pub mod foo {
        pub use super::foo as example;
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
            debug::exit(debug::EXIT_SUCCESS);
        }
    }

    #[__rtic_main]
    fn __rtic_main(){
        let _example_pre_init = "example pre init";
        #[__post_init]
        fn __post_init() {
            let _example_post_init = "example post init";
        }
    }
}
