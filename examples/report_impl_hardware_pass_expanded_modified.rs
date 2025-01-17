#![feature(prelude_import)]
//! examples/hardware.rs
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use panic_semihosting as _;
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use lm3s6965 as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    /// #user_imports
    use cortex_m_semihosting::debug;
    use lm3s6965::Interrupt;
    /// #user_code
    /// #user_init
    /// user_init
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtic::pend(Interrupt::UART0);
        (Shared {}, Local {}, init::Monotonics())
    }
    /// #user_idle
    ///Idle function
    #[allow(non_snake_case)]
    fn idle() -> ! {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        loop {
            cortex_m::asm::nop();
            debug::exit(debug::EXIT_SUCCESS);
        }
    }
    /// #user_hardware_tasks
    #[allow(non_snake_case)]
    fn foo() {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
    }
    /// #root_init
    struct Shared {}
    struct Local {}
    /// module_init
    /// Monotonics used by the system
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_Monotonics();
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_init_Context<'a> {
        /// Core (Cortex-M) peripherals
        pub core: rtic::export::Peripherals,
        /// Device peripherals
        pub device: lm3s6965::Peripherals,
        /// Critical section token for init
        pub cs: rtic::export::CriticalSection<'a>,
    }
    impl<'a> __rtic_internal_init_Context<'a> {
        #[inline(always)]
        pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
            __rtic_internal_init_Context {
                device: lm3s6965::Peripherals::steal(),
                cs: rtic::export::CriticalSection::new(),
                core,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Initialization function
    pub mod init {
        pub use super::__rtic_internal_Monotonics as Monotonics;
        pub use super::__rtic_internal_init_Context as Context;
    }
    /// #root_idle
    /// #modules_hardware_tasks
    pub mod foo {
        pub use super::foo as example;
    }
    /// #mod_app_hardware_tasks
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn UART0() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(PRIORITY, || { foo() });
    }
    /// #main
    #[doc(hidden)]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            /// main_init.rs
            fn the_start() {}
            ///pre_init start
            fn tmp_start() {}
            rtic::export::interrupt::disable();
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
                .into();
            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 1u8 as usize {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Maximum priority used by interrupt vector \'UART0\' is more than supported by hardware",
                        ),
                    );
                };
            };
            core.NVIC
                .set_priority(
                    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART0,
                    rtic::export::logical2hw(1u8, lm3s6965::NVIC_PRIO_BITS),
                );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART0,
            );
            ///pre_init end
            fn tmp() {}
            let _example_pre_init = "example pre init";
            let _example_pre_init2 = "example pre init 2";
            #[inline(never)]
            fn __rtic_init_resources<F>(f: F)
            where
                F: FnOnce(),
            {
                f();
            }
            __rtic_init_resources(|| {
                let (shared_resources, local_resources, mut monotonics) = init(
                    init::Context::new(core.into()),
                );
                let _example_post_init = "example post init";
                rtic::export::interrupt::enable();
            });
            /// main_init.rs
            fn the_end() {}
            idle()
        }
    }
}
