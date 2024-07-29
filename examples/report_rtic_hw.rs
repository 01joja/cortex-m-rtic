//! examples/init.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/* 
Label [lis:hw_tasks]  
Caption: 
Two hardware tasks foo and baz where baz has the highest priority.
The task foo is bound to the interrupt GPIOA and baz to GPIOB.
In this example baz will always have a priority over foo and therefor baz will start running and interrupt foo.
The output of this application will be "baz", "foo", "baz", and then "foo".
*/
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
        // Pends the interrupt GPIOA that is bound to foo
        rtic::pend(Interrupt::GPIOA);
        // Pends the interrupt GPIOB that is bound to baz
        rtic::pend(Interrupt::GPIOB);
        (Shared {}, Local {}, init::Monotonics())
    }

    // Hardware task bound to the interrupt GPIOA and with a priority of 1.
    // When priority is omitted it defaults to 1.
    #[task(binds = GPIOA)]
    fn foo(_: foo::Context){
        hprintln!("foo").ok();
        rtic::pend(Interrupt::GPIOB); // baz will interrupt foo
        hprintln!("foo").ok();
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    // Hardware task bound to the interrupt GPIOB and with a priority of 2
    #[task(binds = GPIOB, priority = 2)]
    fn baz(_: baz::Context){
        hprintln!("baz").ok();
    }
}
