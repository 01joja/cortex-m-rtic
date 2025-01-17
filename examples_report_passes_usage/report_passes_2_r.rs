//! examples/message_passing.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/* 
Label [lis:passes_2_r]  
Caption: 
The overhead for the resources has been generated, but the #[shared] and #[local] is still there.
The parser doesn't allow for applications without them so they have to stay but are empty.
The contexts are also generated.
*/

#[rtic::app(device = lm3s6965, dispatchers = [SSI0, GPIOA], compiler_passes = [software, hardware])]
mod app {
    #[shared] // *Should have been removed*
    struct Shared {
        shared_r: i16,
        #[lock_free]
        shared_lock_free: u8,
    }

    #[local] // *Should have been removed*
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
    
    #[__rtic_task_module(has_context = true, has_monotonic = true)]
    pub mod init {
        pub use super::__rtic_context_init_context as Context;
        pub use super::__rtic_monotonic_monotonic_struct as Monotonics;
    }

    #[task(capacity=1, priority=2)] // *Removed in next pass*
    fn foo(_: foo::Context) {
    }
    
    #[__rtic_task_module(has_context = true, has_monotonic = true)] // *Removed in next pass*
    pub mod foo {
        pub use super::__rtic_context_foo_context as Context; // *New*
        pub use super::__rtic_local_resource_foo_local_resources as LocalResources; // *New*
        pub use super::__rtic_shared_resource_foo_shared_resources as SharedResources; // *New*
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        pub mod MyMono {
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_after as spawn_after;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_at as spawn_at;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_handler as SpawnHandle;
        }
    }

    #[task(capacity=1, priority=1)] // *Removed in next pass*
    fn bar(_: bar::Context) {
    }
    
    #[__rtic_task_module(has_context = true, has_monotonic = true)] // *Removed in next pass*
    pub mod bar {
        pub use super::__rtic_context_bar_context as Context; // *New*
        pub use super::__rtic_shared_resource_bar_shared_resources as SharedResources; // *New*
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        pub mod MyMono {
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_after as spawn_after;
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_at as spawn_at;
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_handler as SpawnHandle;
        }
    }

    #[task(binds=UART0)]
    fn baz(_: baz::Context){
    }
    
    #[__rtic_task_module(has_context = true, has_monotonic = false)]
    pub mod baz {
        pub use super::__rtic_context_baz_context as Context; // *New*
        pub use super::__rtic_local_resource_baz_local_resources as LocalResources; // *New*
        pub use super::__rtic_shared_resource_baz_shared_resources as SharedResources; // *New*
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
        // Unmasking SysTick and disable interrupt on empty queue
        #[__post_init]
        fn post_init() {
            // Store the initial values ib the resources storages // *New*
            // Resets the monotonic and stores it in the monotonic storage
        }
    }
}