
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;


#[rtic::app(device = lm3s6965, dispatchers = [SSI0, GPIOA], compiler_passes = [monotonics,resources,software,hardware])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

    #[shared]
    struct Shared {
        shared_r: i16,
        #[lock_free]
        shared_lock_free: u8,
    }

    #[local]
    struct Local {
        local_r: i8,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;
        // Initialize the monotonic (SysTick rate in QEMU is 12 MHz)
        let mono = Systick::new(systick, 12_000_000);
        (
            Shared {
                shared_r: 0,
                shared_lock_free: 0,
            }, 
            Local {
                local_r: 0,
            }, 
            init::Monotonics(mono)
        )
    }

    #[task(capacity = 1, priority = 2, shared=[ shared_r], local=[local_r])]
    fn foo(_: foo::Context) {
        hprintln!("foo").ok();
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    // foo has a capacity of 3 and 2 messages
    #[task(capacity = 1, priority = 1, shared=[ shared_r, shared_lock_free ])]
    fn bar(_: bar::Context) {
    }

    // bar has the minimum capacity of 1 and 1 massage
    #[task(binds = UART0, shared=[shared_lock_free], local=[late_local: u16 = 0])]
    fn baz(_: baz::Context){
    }

    #[idle()]
    fn idle(_: idle::Context) -> ! {
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        loop {
            cortex_m::asm::nop();
        }
    }
}
