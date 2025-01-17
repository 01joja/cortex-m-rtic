//! examples/lock.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/* 
Label [lis:shared]  
Caption: 
The RTIC application shows how a shared resource is used and how the critical sections impacts the tasks.
It prints "A", "B - shared 1", "C", "D - shared 2", "E". 
*/
#[rtic::app(device = lm3s6965, dispatchers = [GPIOA, GPIOB, GPIOC])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {
        a_shared: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        foo::spawn().unwrap();

        (Shared { a_shared: 0 }, Local {}, init::Monotonics())
    }

    #[task(shared = [a_shared])]
    fn foo(mut c: foo::Context) {
        hprintln!("A").unwrap();
        // the lower priority task requires a critical section to access the data
        c.shared.a_shared.lock(|a_shared| {
            // bar will *not* run right now due to the critical section
            bar::spawn().unwrap(); 
            // data can only be modified within this critical section (closure)
            *a_shared += 1; 
            hprintln!("B - shared {}", *a_shared).unwrap();
            // baz does not contend for `shared` so it's allowed to run now
            baz::spawn().unwrap();
        });
        // critical section is over: bar can now start
        hprintln!("E").unwrap();
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    #[task(priority = 2, shared = [a_shared])]
    fn bar(mut c: bar::Context) {
        // the higher priority task does still need a critical section
        let shared = c.shared.a_shared.lock(|a_shared| {
            *a_shared += 1;
            *a_shared
        });
        hprintln!("D - shared {}", shared).unwrap();
    }

    #[task(priority = 3)]
    fn baz(_: baz::Context) {
        hprintln!("C").unwrap();
    }
}
