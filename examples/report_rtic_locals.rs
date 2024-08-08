//! examples/locals.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/*
Label [lis:locals]  
Caption: 
This applications contains 2 types of local resources, the late local recourse and the local resource.
Output of application is "foo: local_foo = 0", "bar: local_bar = 0", "foo: local_foo = 1" and "bar: local_bar = 1".
*/

#[rtic::app(device = lm3s6965, dispatchers = [UART0, UART1])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        local_foo: i32,
    }

    // `#[init]` cannot access locals from the `#[local]` struct as they are 
    // initialized here.
    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        foo::spawn().unwrap();
        bar::spawn().unwrap();
        foo::spawn().unwrap();
        bar::spawn().unwrap();

        (
            Shared {},
            // initial values for the `#[local]` resources
            Local {
                local_foo: 0,
            },
            init::Monotonics(),
        )
    }

    // `local_foo` can only be accessed from this context
    #[task(capacity = 2, local = [local_foo])]
    fn foo(cx: foo::Context) {
        let local_foo = cx.local.local_foo;
        *local_foo += 1;
        hprintln!("foo local_foo {}", local_foo).unwrap();
    }

    // late local, `local_bar`, that can only be accessed from this context
    #[task(capacity = 2, local = [local_bar: i32 = 0])]
    fn bar(cx: bar::Context) {
        let local_bar = cx.local.local_bar;
        *local_bar += 1;
        hprintln!("bar local_bar {}", local_bar).unwrap();
        if *local_bar > 1{
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        }
    }
}
