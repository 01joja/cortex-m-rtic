//! examples/message_passing.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/* 
Label [lis:sw]  
Caption: 
? use dispatched instead of executed ? 
3 Software tasks, foo and bar has the same priority of 1 and baz has a priority of 2.
They are all spawned in the init task and then executed according to priority.
foo will execute before bar because it was spawned first.
Application output is "bar","foo","bar","foo","baz".
*/

#[rtic::app(device = lm3s6965, dispatchers = [GPIOA,GPIOB])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        // spawns foo, bar and baz
        foo::spawn().unwrap(); 
        bar::spawn().unwrap();
        baz::spawn().unwrap();

        (Shared {}, Local {}, init::Monotonics())
    }

    // task baz will interrupt task foo
    #[task(priority = 1)]
    fn foo(_: foo::Context) {
        hprintln!("foo").unwrap();
        baz::spawn().unwrap();
        hprintln!("foo").unwrap();
    }

    // omitting priority will give the task a priority of 1
    #[task()]
    fn bar(_: bar::Context){
        hprintln!("bar").unwrap();
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    // baz has a higher priority and will interrupt bar and foo.
    #[task(priority = 2)]
    fn baz(_: baz::Context){
        hprintln!("baz").unwrap();
    }
}
