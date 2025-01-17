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

#[rtic::app(device = lm3s6965, dispatchers = [SSI0, GPIOA], compiler_passes = [resources, software, hardware])]
mod app {
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
            Shared { shared_r: 0, shared_lock_free: 0 }, 
            Local { local_r: 0 }, 
            init::Monotonics(mono)
        )
    }

    #[__rtic_task_module(has_monotonic = true)] // *New*
    pub mod init {
        pub use super::__rtic_monotonic_monotonic_struct as Monotonics;
    }

    #[task(capacity=1, priority=2, shared=[shared_r, &only_shared], local=[local_r])]
    fn foo(_: foo::Context) {
    }
    
    #[__rtic_task_module(has_monotonic = true)] // *New*
    pub mod foo {
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        pub mod MyMono {
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_after as spawn_after;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_at as spawn_at;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_handler as SpawnHandle;
        }
    }

    #[task(capacity=1, priority=1, shared=[shared_r, shared_lock_free])]
    fn bar(_: bar::Context) {
    }
    
    #[__rtic_task_module(has_monotonic = true)] // *New*
    pub mod foo {
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        pub mod MyMono {
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_after as spawn_after;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_at as spawn_at;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_handler as SpawnHandle;
        }
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

    // Discussed in each pass, the comments represents code
    #[__rtic_main] // *New*
    fn __rtic_main() {
        // Unmasking SysTick and disable interrupt on empty queue
        #[__post_init]
        fn post_init() {
            // Resets the monotonic and stores it in the monotonic storage
        }
    }
}