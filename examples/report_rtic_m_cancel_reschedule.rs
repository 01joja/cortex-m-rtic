//! examples/cancel-reschedule.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/*
Label [lis:m_cancel_reschedule]  
Caption: 
An application that reschedules and cancels the task baz.
It also shows how a task handler can be passed between tasks.
*/

#[rtic::app(device = lm3s6965, dispatchers = [SSI0, GPIOA])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;

    #[monotonic(binds = SysTick)]
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

        // Schedule `foo` to run 1 and 5 second in the future
        foo::MyMono::spawn_after(1.secs()).unwrap();
        foo::MyMono::spawn_after(5.secs()).unwrap();
        (
            Shared {},
            Local {},
            init::Monotonics(mono), // Passes the monotonic to RTIC
        )
    }

    #[task(capacity = 2, local = [count: u32 = 0])]
    fn foo(cx: foo::Context) {
        hprintln!("foo").ok();

        // Schedule baz to run 6 seconds in the future (5 second after bar runs)
        let spawn_handle = baz::MyMono::spawn_after(2.secs()).unwrap();
        if *cx.local.count < 1{
            bar::MyMono::spawn_after(1.secs(), spawn_handle, true).unwrap();
        } else {
            bar::MyMono::spawn_after(1.secs(), spawn_handle, false).unwrap();
        }
        *cx.local.count +=1;
    }

    // bar either reschedules or cancels baz depending on do_reschedule
    #[task]
    fn bar(_: bar::Context, baz_handle: baz::MyMono::SpawnHandle, do_reschedule: bool) {
        hprintln!("bar").ok();

        if do_reschedule {
            // Reschedule baz 2 seconds from now, instead of the original 5 second
            // from now.
            hprintln!("reschedule").ok();
            baz_handle.reschedule_after(2.secs()).unwrap();
            // Or baz_handle.reschedule_at(/* time */)
        } else {
            // Or cancel it
            hprintln!("cancel").ok();
            baz_handle.cancel().unwrap();
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        }
    }

    #[task]
    fn baz(_: baz::Context) {
        hprintln!("baz").ok();
    }

}
