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

#[rtic::app(device = lm3s6965, dispatchers = [SSI0, GPIOA])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;

    #[monotonic(binds = SysTick, default = true)]
    // the monotonic
    type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

    // 2 shared resources, one is lock free.
    #[shared]
    struct Shared {
        shared_r: i16,
        #[lock_free]
        shared_lock_free: u8,
    }

    // 1 local resource
    #[local]
    struct Local {
        local_r: i8,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;
        // Initialize the monotonic (SysTick rate in QEMU is 12 MHz)
        let mono = Systick::new(systick, 12_000_000);
        // Schedule `foo` to run 1 second in the future
        foo::spawn_after(1.secs()).unwrap();
        (
            Shared { shared_r: 0, shared_lock_free: 0,}, 
            Local { local_r: 0, }, 
            init::Monotonics(mono)
        )
    }

    // a software task foo with resources and priority of 2
    #[task(priority = 2, shared=[shared_r], local=[local_r])]
    fn foo(_: foo::Context) {
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    // a software task bar with resources and priority of 1
    #[task(priority = 1, shared=[shared_r, shared_lock_free])]
    fn bar(_: bar::Context, _x: i32, _y: i32) {
    }

    // a hardware task baz with resources, priority of 1 and bound to interrupt UART0
    #[task(binds = UART0, priority = 1, shared=[shared_lock_free], local=[late_local: u16 = 0])]
    fn baz(_: baz::Context){
    }
}
