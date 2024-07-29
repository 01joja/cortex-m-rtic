//! examples/message_passing.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/* 
Label [lis:sw_mess]  
Caption: 
A RTIC application with software tasks that uses message passing and capacity.
Message passing allow passing values to parameters of the tasks.
The capacity defines how many times a task can be spawned before it needs to be executed.
Output of this application will be "foo 1, 1","bar 2000","foo 1, 2" and "foo 2, 3"
*/

#[rtic::app(device = lm3s6965, dispatchers = [SSI0])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        foo::spawn(1, 1).unwrap();
        bar::spawn(2000).unwrap();
        foo::spawn(1, 2).unwrap();
        foo::spawn(2, 3).unwrap();
        // The capacity of `foo` is reached and it can't spawn any more.
        assert!(foo::spawn(1, 4).is_err()); 

        (Shared {}, Local {}, init::Monotonics())
    }

    // foo has a capacity of 3 and 2 messages
    #[task(capacity = 3)]
    fn foo(_: foo::Context, x: i8, y: u8) {
        hprintln!("foo {}, {}", x, y).unwrap();
        if x == 2 {
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        }
    }

    // bar has the minimum capacity of 1 and 1 massage
    #[task()]
    fn bar(_: bar::Context, z: i16){
        hprintln!("bar {}", z).unwrap();
    }
}
