//! examples/lock.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, dispatchers = [GPIOA, GPIOB, GPIOC])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {
        resource: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        foo::spawn().unwrap();

        (Shared { resource: 0 }, Local {}, init::Monotonics())
    }

    // when omitted priority is assumed to be `1`
    #[task(shared = [resource])]
    fn foo(mut c: foo::Context) {
        hprintln!("A").unwrap();

        // the lower priority task requires a critical section to access the data
        c.shared.resource.lock(|resource| {
            // data can only be modified within this critical section (closure)
            *resource += 1;

            // bar will *not* run right now due to the critical section
            bar::spawn().unwrap();

            hprintln!("B - resource = {}", *resource).unwrap();

            // baz does not contend for `shared` so it's allowed to run now
            baz::spawn().unwrap();
        });

        // critical section is over: bar can now start

        hprintln!("E").unwrap();

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    #[task(priority = 2, shared = [resource])]
    fn bar(mut c: bar::Context) {
        // the higher priority task does still need a critical section
        let resource = c.shared.resource.lock(|resource| {
            *resource += 1;

            *resource
        });

        hprintln!("D - shared = {}", resource).unwrap();
    }

    #[task(priority = 3)]
    fn baz(_: baz::Context) {
        hprintln!("C").unwrap();
    }
}
