//! examples/message_passing.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/* 
Label [lis:passes_0]  
Caption: 
A RTIC application that generates code for all passes.
*/

#[rtic::app(device = lm3s6965, dispatchers = [SSI0, GPIOA], 
    compiler_passes = [monotonics, resources, software, hardware])]
mod app {
    #[monotonic(binds = SysTick, default = true)] // *Removed in next pass*
    type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

    #[shared]
    struct Shared {
        shared_r: i16,
        #[lock_free]
        shared_lock_free: u8,
        only_shared: u8,
    }

    #[local]
    struct Local {
        local_r: i8,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        //user code
        (
            Shared { shared_r: 0, shared_lock_free: 0, only_shared: 0 }, 
            Local { local_r: 0 }, 
            init::Monotonics(mono)
        )
    }

    #[task(capacity=1, priority=2, shared=[shared_r, &only_shared], local=[local_r])]
    fn foo(_: foo::Context) {
    }

    #[task(capacity=1, priority=1, shared=[shared_r, shared_lock_free])]
    fn bar(_: bar::Context) {
    }

    #[task(binds=UART0, shared=[shared_lock_free, &only_shared], local=[late_local: u16 = 0])]
    fn baz(_: baz::Context){
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        loop {
            cortex_m::asm::nop();
        }
    }
}