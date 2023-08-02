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

        run_gubben();
        hprintln!("foo").unwrap();

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    pub fn run_gubben(){
        hprintln!("nu startar jag gubben!").unwrap();
    
    }

    #[__rtic_main]
    fn __rtic_main(){
        gubben::run();
        gubben();

        #[__post_init]
        fn something(){
            hprintln!("Yes, du hittade mig :D :D");
        }
    }

    #[__rtic_task_module(has_context = true)]
    pub mod gubben{
         pub use super::run_gubben as run;
    }

    #[__rtic_task_module]
    pub mod foo{
         pub use super::run_gubben as run;
    }

    
    
    

}
