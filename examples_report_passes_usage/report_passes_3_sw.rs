//! examples/message_passing.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/* 
Label [lis:passes_3_sw]  
Caption: 
Takes care of the software tasks and generates the dispatchers as hardware tasks.
*/

#[rtic::app(device = lm3s6965, dispatchers = [SSI0, GPIOA], 
    compiler_passes = [hardware])]
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
    
    #[__rtic_task_module(has_context = true, has_monotonic = true)]
    pub mod init {
        pub use super::__rtic_context_init_context as Context;
        pub use super::__rtic_monotonic_monotonic_struct as Monotonics;
    }

    #[task(binds = GPIOA, priority = 2)]
    fn __rtic_dispatcher_GPIOA(_: __rtic_dispatcher_GPIOA::Context) {
        // Unsafe dispatcher code
    }

    #[task(binds = SSI0, priority = 1)]
    fn __rtic_dispatcher_SSI0(_: __rtic_dispatcher_SSI0::Context) {
        // Unsafe dispatcher code
    }

    #[task(binds=UART0)]
    fn baz(_: baz::Context){
    }

    #[__rtic_task_module(has_context = true, has_monotonic = false)]
    pub mod baz {
        pub use super::__rtic_context_baz_context as Context;
        pub use super::__rtic_local_resource_baz_local_resources as LocalResources;
        pub use super::__rtic_shared_resource_baz_shared_resources as SharedResources;
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        loop {
            cortex_m::asm::nop();
        }
    }

    #[__rtic_task_module(has_context = true, has_monotonic = false)]
    pub mod idle {
        pub use super::__rtic_context_idle_context as Context;
    }

    // Discussed in each pass, the comments represents code
    #[__rtic_main]
    fn __rtic_main() {
        // populates the request queues of the software tasks
        // Assertions for send of the resource types
        // Unmasking SysTick and disable interrupt on empty queue
        #[__post_init]
        fn post_init() {
            // Sets the initial values to the resources
            // Resets the monotonic and stores it in the monotonic storage
        }
    }
}