//! examples/schedule.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;
/* 
Label [lis:m_mini]  
Caption: 
An example of an RTIC application that uses a monotonic to spawn task foo in the future.
It will print "init", "foo", "foo" with 1 second delay between each print.
*/

#[rtic::app(device = lm3s6965, dispatchers = [SSI0])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;

        // Initialize the monotonic (SysTick rate in QEMU is 12 MHz)
        let mono = Systick::new(systick, 12_000_000);

        hprintln!("init").ok();

        // Schedule `foo` to run 1 second in the future
        foo::spawn_after(1.secs()).unwrap();
        // Schedule `foo` to run 2 seconds after the current time
        foo::spawn_at(monotonics::now() + 2.secs()).unwrap();

        (
            Shared {},
            Local {},
            init::Monotonics(mono), // Give the monotonic to RTIC
        )
    }

    // Software task with a capacity of 2 and a late local resource.
    // The second time it's spawned it will exit the simulator
    #[task(capacity = 2, local = [count: u32 = 0])] // Software task
    fn foo(cx: foo::Context) {
        hprintln!("foo").ok();
        *cx.local.count += 1;
        if *cx.local.count > 1{
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        }
    }
}
