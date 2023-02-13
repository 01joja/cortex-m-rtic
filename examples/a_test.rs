//! examples/hardware.rs



#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, compiler_passes = [hardware])]
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
        rtic::pend(Interrupt::UART0); // equivalent to NVIC::pend

        hprintln!("init").unwrap();

        (Shared {}, Local {}, init::Monotonics())
    }


    #[task(binds = UART0,priority = 1)]
    fn foo(_: foo::Context) {

        gubben::run();
        hprintln!("foo").unwrap();

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    
    fn gubben(){
        hprintln!("haj!").unwrap();
    }

    fn run_gubben(){
        hprintln!("nu startar jag gubben!").unwrap();
    
    }

    #[__rtic_pass_task_module]
    pub mod gubben{
        pub use super::run_gubben as run;
    }

    // should fail, internal not public
    #[__rtic_pass_task_module]
    pub mod fail2{
        pub use super::run_gubben as hej;
        pub use super::run_gubben as kalle;
        use super::run_gubben as vector;
    }

    

}
