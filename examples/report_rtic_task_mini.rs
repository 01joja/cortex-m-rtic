//! examples/message_passing.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/* 
Label [lis:task_minimal]  
Caption: 
A minimal RTIC app with two tasks (see section X) and a init task (see section X).
The init task initializes the application by spawning task foo and baz.
After the init task RTIC will execute the spawned tasks, starting with the highest priority.
So this task will print "baz", "foo", "baz", "foo".
*/

// dispatchers are explained in section X
#[rtic::app(device = lm3s6965, dispatchers = [GPIOA,GPIOB])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    // init task
    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        foo::spawn().unwrap(); // spawns a foo task
        baz::spawn().unwrap(); // spawns a baz task

        (Shared {}, Local {}, init::Monotonics())
    }

    // Task foo with priority 1
    #[task(priority = 1)]
    fn foo(_: foo::Context) {
        hprintln!("foo").unwrap(); // prints "foo"
        baz::spawn().unwrap();     // spawns a baz task that interrupts foo
        hprintln!("foo").unwrap(); // prints "foo"
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    // Task baz with priority 2, will interrupt foo when spawned.
    #[task(priority = 2)]
    fn baz(_: baz::Context) {
        hprintln!("baz").unwrap(); // prints "baz"
    }
}
