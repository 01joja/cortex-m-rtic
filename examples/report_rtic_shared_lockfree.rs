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
Since foo and bar has the same priority of 1 the resource lock_free doesn't need to be locked.
*/

#[rtic::app(device = lm3s6965, dispatchers = [GPIOA])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {
        #[lock_free] // <- lock-free shared resource
        lock_free: u64,
        only_share: u64,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        foo::spawn().unwrap();

        (Shared { lock_free: 0, only_share: 3 }, Local {}, init::Monotonics())
    }

    #[task(shared = [lock_free, &only_share])] // <- same priority
    fn foo(c: foo::Context) {
        bar::spawn().unwrap();  // <- bar will execute after foo
        *c.shared.lock_free += 1; // <- no lock API required
        let lock_free = *c.shared.lock_free;
        let only_share = *c.shared.only_share;
        hprintln!("foo {} {}", lock_free, only_share).unwrap();
    }

    #[task(shared = [lock_free, &only_share])] // <- same priority
    fn bar(c: bar::Context) {
        foo::spawn().unwrap();
        *c.shared.lock_free += 1; // <- no lock API required
        let lock_free = *c.shared.lock_free;
        let only_share = *c.shared.only_share;
        hprintln!("bar {} {}", lock_free, only_share).unwrap();
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }
}
