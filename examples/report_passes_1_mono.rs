#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

/* 
Label [lis:before_passes]  
Caption: 
After the monotonics pass the monotonics has been handled and thus removed.
*/

#[rtic::app(device = lm3s6965, dispatchers = [SSI0, GPIOA], 
    compiler_passes = [resources,software,hardware])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;

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
        hprintln!("init").ok();
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
    
    #[__rtic_task_module(has_monotonic = true)]
    pub mod init {
        pub use super::__rtic_monotonic_monotonic_struct as Monotonics;
    }

    #[task(capacity=1, priority=2, shared=[shared_r], local=[local_r])]
    fn foo(_: foo::Context) {
    }

    #[__rtic_task_module(has_monotonic = true)]
    pub mod foo {
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        #[allow(non_snake_case)]
        pub mod MyMono {
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_after as spawn_after;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_at as spawn_at;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_handler as SpawnHandle;
        }
    }

    #[task(capacity=2, priority=1, shared=[ shared_r, shared_lock_free])]
    fn bar(_: bar::Context, _x: i32, _y: i32) {
    }

    #[__rtic_task_module(has_monotonic = true)]
    pub mod bar {
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        #[allow(non_snake_case)]
        pub mod MyMono {
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_after as spawn_after;
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_at as spawn_at;
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_handler as SpawnHandle;
        }
    }

    #[task(binds=UART0, shared=[shared_lock_free], local=[late_local: u16 = 0])]
    fn baz(_: baz::Context){
    }

    #[idle(local = [x: u32 = 0])]
    fn idle(cx: idle::Context) -> ! {
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator

        loop {
            cortex_m::asm::nop();
        }
    }
}
