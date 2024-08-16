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

#[rtic::app(device = lm3s6965)]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {
        r_shared: i32,
        #[lock_free] // <- lock-free shared resource
        r_lock_free: i32,
        r_only_shared: i32,
    }

    #[local]
    struct Local {
        r_local: i32,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        foo::spawn().unwrap();

        (Shared { r_shared: 0, r_lock_free: 0, r_only_shared: 0 }, Local { r_local: 0 }, init::Monotonics())
    }

    #[task(binds = GPIOA, shared = [r_shared, r_lock_free, &r_only_shared], local = [r_local, r_late_local: i32 = 0])] // <- same priority
    fn foo(c: foo::Context) {
        bar::spawn().unwrap();  // <- bar will execute after foo
        *c.shared.counter += 1; // <- no lock API required
        let counter = *c.shared.counter;
        hprintln!("  foo = {}", counter).unwrap();
    }
}
