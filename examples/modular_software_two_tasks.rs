

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, 
    dispatchers = [GPIOA], 
    compiler_passes = [software,hardware])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        bar::spawn().unwrap();
        foo::spawn().unwrap();
        hprintln!("init").unwrap();

        (Shared {}, Local {}, init::Monotonics())
    }

    #[task(priority = 1)]
    fn foo(_: foo::Context) {
        hprintln!("foo").unwrap();

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    #[task(priority = 1)]
    fn bar(_: bar::Context) {
        hprintln!("bar").unwrap();
    }

}
