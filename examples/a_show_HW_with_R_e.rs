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
mod app {
    /// contexts
    #[__rtic_task_module(has_context = true, has_monotonic = false)]
    pub mod init {
        pub use super::__rtic_context_init_context as Context;
    }
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
    #[__rtic_task_module(has_context = true, has_monotonic = false)]
    pub mod foo {
        pub use super::__rtic_context_foo_context as Context;
        pub use super::__rtic_shared_resource_foo_shared_resources as SharedResources;
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_context_foo_context<'a> {
        pub shared: foo::SharedResources<'a>,
    }
    impl<'a> __rtic_context_foo_context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_context_foo_context {
                shared: foo::SharedResources::new(priority),
            }
        }
    }
    #[__rtic_task_module(has_context = true, has_monotonic = false)]
    pub mod bar {
        pub use super::__rtic_context_bar_context as Context;
        pub use super::__rtic_shared_resource_bar_shared_resources as SharedResources;
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_context_bar_context<'a> {
        pub shared: bar::SharedResources<'a>,
    }
    impl<'a> __rtic_context_bar_context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_context_bar_context {
                shared: bar::SharedResources::new(priority),
            }
        }
    }
    /// structs
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `foo` has access to
    pub struct __rtic_shared_resource_foo_shared_resources<'a> {
        pub resource: shared_resources::resource_that_needs_to_be_locked<'a>,
    }
    impl<'a> __rtic_shared_resource_foo_shared_resources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_shared_resource_foo_shared_resources {
                resource: shared_resources::resource_that_needs_to_be_locked::new(
                    priority,
                ),
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `bar` has access to
    pub struct __rtic_shared_resource_bar_shared_resources<'a> {
        pub resource: shared_resources::resource_that_needs_to_be_locked<'a>,
    }
    impl<'a> __rtic_shared_resource_bar_shared_resources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_shared_resource_bar_shared_resources {
                resource: shared_resources::resource_that_needs_to_be_locked::new(
                    priority,
                ),
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic_r_resource"]
    static __rtic_shared_resource_resource: rtic::RacyCell<
        core::mem::MaybeUninit<u32>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    impl<'a> rtic::Mutex for shared_resources::resource_that_needs_to_be_locked<'a> {
        type T = u32;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut u32) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            /// Priority ceiling
            const CEILING: u8 = 3u8;
            unsafe {
                rtic::export::lock(
                    __rtic_shared_resource_resource.get_mut() as *mut _,
                    self.priority(),
                    CEILING,
                    lm3s6965::NVIC_PRIO_BITS,
                    &__rtic_shared_resources_MASKS,
                    f,
                )
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_shared_resources_MASK_CHUNKS: usize = rtic::export::compute_mask_chunks([
        lm3s6965::Interrupt::UART0 as u32,
        lm3s6965::Interrupt::UART1 as u32,
    ]);
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_shared_resources_MASKS: [rtic::export::Mask<
        __rtic_shared_resources_MASK_CHUNKS,
    >; 3] = [
        rtic::export::create_mask([lm3s6965::Interrupt::UART0 as u32]),
        rtic::export::create_mask([]),
        rtic::export::create_mask([lm3s6965::Interrupt::UART1 as u32]),
    ];
    mod shared_resources {
        use rtic::export::Priority;
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct resource_that_needs_to_be_locked<'a> {
            priority: &'a Priority,
        }
        impl<'a> resource_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a Priority) -> Self {
                resource_that_needs_to_be_locked {
                    priority,
                }
            }
            #[inline(always)]
            pub unsafe fn priority(&self) -> &Priority {
                self.priority
            }
        }
    }
    #[local]
    struct Local {}
    #[shared]
    struct Shared {
        resource: u32,
    }
    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;
    #[init()]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtic::pend(Interrupt::UART0);
        ::cortex_m_semihosting::export::hstdout_str("init\n").unwrap();
        (Shared { resource: 0 }, Local {}, init::Monotonics())
    }
    #[task(binds = UART0, priority = 1)]
    fn foo(mut c: foo::Context) {
        rtic::pend(Interrupt::UART1);
        c.shared
            .resource
            .lock(|resource| {
                rtic::pend(Interrupt::UART1);
                *resource += 1;
                ::cortex_m_semihosting::export::hstdout_fmt(
                        format_args!("foo {0}\n", *resource),
                    )
                    .unwrap();
            });
        debug::exit(debug::EXIT_SUCCESS);
    }
    #[task(binds = UART1, priority = 3)]
    fn bar(mut c: bar::Context) {
        c.shared
            .resource
            .lock(|resource| {
                *resource += 1;
                ::cortex_m_semihosting::export::hstdout_fmt(
                        format_args!("bar {0}\n", *resource),
                    )
                    .unwrap();
            });
    }
    #[__rtic_main]
    fn __rtic_main() {
        rtic::export::assert_send::<u32>();
        const _CONST_CHECK: () = {
            if !rtic::export::have_basepri() {
                if (lm3s6965::Interrupt::UART0 as usize)
                    >= (__rtic_shared_resources_MASK_CHUNKS * 32)
                {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "An interrupt out of range is used while in armv6 or armv8m.base",
                            ),
                        );
                    };
                }
                if (lm3s6965::Interrupt::UART1 as usize)
                    >= (__rtic_shared_resources_MASK_CHUNKS * 32)
                {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "An interrupt out of range is used while in armv6 or armv8m.base",
                            ),
                        );
                    };
                }
            } else {}
        };
        let _ = _CONST_CHECK;
        #[__post_init]
        fn post_init() {
            __rtic_shared_resource_resource
                .get_mut()
                .write(core::mem::MaybeUninit::new(shared_resources.resource));
        }
    }
}
