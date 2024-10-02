//! examples/cancel-reschedule.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, dispatchers = [SSI0], compiler_passes = [standard])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

    #[shared]
    struct Shared {
        do_reschedule: bool
    }

    #[local]
    struct Local {
        baz_spawn_handle: baz::SpawnHandle,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;

        // Initialize the monotonic (SysTick rate in QEMU is 12 MHz)
        let mono = Systick::new(systick, 12_000_000);

        hprintln!("init").ok();

        // Schedule `foo` to run 1 second in the future
        foo::spawn_after(1.secs()).unwrap();
        let spawn_handler = baz::spawn_after(1.secs()).unwrap();

        (
            Shared {
                do_reschedule: false
            },
            Local {
                baz_spawn_handle: spawn_handler,
            },
            init::Monotonics(mono), // Give the monotonic to RTIC
        )
    }

    #[task(shared = [do_reschedule])]
    fn foo(_: foo::Context) {
        hprintln!("foo").ok();

        // Schedule `bar` to run 2 seconds in the future (1 second after foo runs)
        let _spawn_handle = baz::spawn_after(2.secs()).unwrap();
        bar::spawn_after(1.secs()).unwrap(); // Change to true
    }

    #[task(shared = [ do_reschedule], local = [baz_spawn_handle])]
    fn bar(_: bar::Context) {
        hprintln!("bar").ok();

        if do_reschedule {
            // Reschedule baz 2 seconds from now, instead of the original 1 second
            // from now.
            baz_spawn_handle.reschedule_after(2.secs()).unwrap();
            // Or baz_handle.reschedule_at(/* time */)
        } else {
            // Or cancel it
            baz_spawn_handle.cancel().unwrap();
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        }
    }

    #[task()]
    fn baz(_: baz::Context) {
        hprintln!("baz").ok();
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }
}
