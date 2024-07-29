//! examples/lock-free.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/* 
Label [lis:shared_lockfree]  
Caption: 
Example of a lock_free shared resource between foo and bar.
Since foo and bar has the same priority of 1 the resource counter doesn't need to be locked.
*/

#[rtic::app(device = lm3s6965, dispatchers = [GPIOA])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {
        #[lock_free] // <- lock-free shared resource
        counter: u64,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        foo::spawn().unwrap();

        (Shared { counter: 0 }, Local {}, init::Monotonics())
    }

    #[task(shared = [counter])] // <- same priority
    fn foo(c: foo::Context) {
        bar::spawn().unwrap();

        *c.shared.counter += 1; // <- no lock API required
        let counter = *c.shared.counter;
        hprintln!("  foo = {}", counter).unwrap();
    }

    #[task(shared = [counter])] // <- same priority
    fn bar(c: bar::Context) {
        foo::spawn().unwrap();

        *c.shared.counter += 1; // <- no lock API required
        let counter = *c.shared.counter;
        hprintln!("  bar = {}", counter).unwrap();

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }
}
