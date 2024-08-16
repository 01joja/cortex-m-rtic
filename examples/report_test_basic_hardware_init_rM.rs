//! examples/hardware.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, compiler_passes = [standard])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init(local = [t: i32 = 5])]
    fn init(context: init::Context) -> (Shared, Local, init::Monotonics) {
        let local_t = context.local.t;
        hprintln!("{:?}",local_t).unwrap();

        hprintln!("init").unwrap();

        
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator

        (Shared {}, Local {}, init::Monotonics())
    }

}
