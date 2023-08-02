//! examples/report_resource.rs

// This example belongs to the code 

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965)]
mod app {
    use cortex_m_semihosting::debug;
    #[shared]
    struct Shared {shared_r: u32}

    #[local]
    struct Local {local_r: u32}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        ( Shared {shared_r: 0}, Local {local_r: 0},init::Monotonics() )
    }
    
    #[task(binds = UART0,  local = [local_r], shared = [shared_r])]
    fn foo(mut cx: foo::Context) {
        let local_r = cx.local.local_r;
        *local_r += 1;
        cx.shared.shared_r.lock(|locked_shared_r| {
            *locked_shared_r += 1;
        });
    }

    #[task(binds = UART1,  local = [late_local_r: u32 = 0], shared = [shared_r])]
    fn bar(mut cx: bar::Context) {
        cx.shared.shared_r.lock(|locked_shared_r| {
            *locked_shared_r += 1;
        });
        let late_local_r = cx.local.late_local_r;
        *late_local_r += 1;
    }

    #[idle]
    fn idle(_: idle::Context) -> !{
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        loop {
            cortex_m::asm::nop();
        }
    }
}

