
#![feature(prelude_import,core_panic,rustc_private,const_fmt_arguments_new)]
//! examples/locals.rs
// #![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
#[prelude_import]
#[macro_use]
extern crate core;
extern crate compiler_builtins;
use panic_semihosting as _;
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use lm3s6965 as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    /// #user_imports
    use cortex_m_semihosting::{debug};
    use lm3s6965::Interrupt;
    /// #user_code
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_context_init_context<'a> {
        /// Core (Cortex-M) peripherals
        pub core: rtic::export::Peripherals,
        /// Device peripherals
        pub device: lm3s6965::Peripherals,
        /// Critical section token for init
        pub cs: rtic::export::CriticalSection<'a>,
    }
    impl<'a> __rtic_context_init_context<'a> {
        #[inline(always)]
        pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
            __rtic_context_init_context {
                device: lm3s6965::Peripherals::steal(),
                cs: rtic::export::CriticalSection::new(),
                core,
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_context_idle_context {}
    impl __rtic_context_idle_context {
        #[inline(always)]
        pub unsafe fn new(_priority: &rtic::export::Priority) -> Self {
            __rtic_context_idle_context {}
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_context_foo_context<'a> {
        pub local: foo::LocalResources<'a>,
    }
    impl<'a> __rtic_context_foo_context<'a> {
        #[inline(always)]
        pub unsafe fn new(_priority: &'a rtic::export::Priority) -> Self {
            __rtic_context_foo_context {
                local: foo::LocalResources::new(),
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `foo` has access to
    pub struct __rtic_local_resource_foo_local_resources<'a> {
        pub local_to_foo: &'a mut i64,
    }
    impl<'a> __rtic_local_resource_foo_local_resources<'a> {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_local_resource_foo_local_resources {
                local_to_foo: &mut *(&mut *__rtic_local_resource_local_to_foo.get_mut())
                    .as_mut_ptr(),
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic_local_to_foo"]
    static __rtic_local_resource_local_to_foo: rtic::RacyCell<
        core::mem::MaybeUninit<i64>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_internal_MASK_CHUNKS: usize = rtic::export::compute_mask_chunks([
        lm3s6965::Interrupt::UART0 as u32,
    ]);
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_internal_MASKS: [rtic::export::Mask<__rtic_internal_MASK_CHUNKS>; 3] = [
        rtic::export::create_mask([lm3s6965::Interrupt::UART0 as u32]),
        rtic::export::create_mask([]),
        rtic::export::create_mask([]),
    ];
    /// #user_init
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        (Shared {}, Local { local_to_foo: 0 }, init::Monotonics())
    }
    /// #user_idle
    #[allow(non_snake_case)]
    #[allow(non_snake_case)]
    fn idle(_: idle::Context) -> ! {
        loop {
            rtic::pend(Interrupt::UART0);
            cortex_m::asm::nop();
        }
    }
    /// #user_hardware_tasks
    #[allow(non_snake_case)]
    fn foo(cx: foo::Context) {
        let local_to_foo = cx.local.local_to_foo;
        *local_to_foo += 1;
        ::cortex_m_semihosting::export::hstdout_fmt(
                format_args!("foo: local_to_foo = {0}\n", local_to_foo),
            )
            .unwrap();
        if local_to_foo > &mut 2 {
            debug::exit(debug::EXIT_SUCCESS);
        }
    }
    /// #root_init
    struct Shared {}
    struct Local {
        local_to_foo: i64,
    }
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
    #[allow(non_snake_case)]
    ///idle loop
    pub mod idle {
        pub use super::__rtic_context_idle_context as Context;
    }
    /// #root_hardware_tasks
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_idle_foo_context {}
    impl __rtic_idle_foo_context {
        #[inline(always)]
        pub unsafe fn new(_priority: &rtic::export::Priority) -> Self {
            __rtic_idle_foo_context {}
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod foo {
        pub use super::__rtic_context_foo_context as Context;
        pub use super::__rtic_local_resource_foo_local_resources as LocalResources;
    }
    /// #mod_app_init
    /// #mod_app_idle
    /// #mod_app_hardware_tasks
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn UART0() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(
            PRIORITY,
            || { foo(foo::Context::new(&rtic::export::Priority::new(PRIORITY))) },
        );
    }
    /// #main
    #[doc(hidden)]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            rtic::export::assert_send::<i64>();
            const _CONST_CHECK: () = {
                if !rtic::export::have_basepri() {
                    if (lm3s6965::Interrupt::UART0 as usize)
                        >= (__rtic_internal_MASK_CHUNKS * 32)
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "An interrupt out of range is used while in armv6 or armv8m.base"
                            ),
                        );
                    }
                } else {}
            };
            let _ = _CONST_CHECK;
            rtic::export::interrupt::disable();
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
                .into();
            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 1u8 as usize {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'UART0\' is more than supported by hardware"
                    ),
                );
            };
            core.NVIC
                .set_priority(
                    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART0,
                    rtic::export::logical2hw(1u8, lm3s6965::NVIC_PRIO_BITS),
                );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART0,
            );
            #[inline(never)]
            fn __rtic_init_resources<F>(f: F)
            where
                F: FnOnce(),
            {
                f();
            }
            __rtic_init_resources(|| {
                let (_shared_resources, _local_resources, _monotonics) = init(
                    init::Context::new(core.into()),
                );
                rtic::export::interrupt::enable();
            });
            idle(idle::Context::new(&rtic::export::Priority::new(0)))
        }
    }
}
