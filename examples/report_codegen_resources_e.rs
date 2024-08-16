#![feature(prelude_import)]
//! examples/lock-free.rs
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
    /// #monotonics
    /// #user_imports
    use cortex_m_semihosting::{debug, hprintln};
    /// #user_code
    /// #user
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        foo::spawn().unwrap();
        (
            Shared {
                r_shared: 0,
                r_lock_free: 0,
                r_only_shared: 0,
            },
            Local { r_local: 0 },
            init::Monotonics(),
        )
    }
    /// #user_hardware_tasks
    #[allow(non_snake_case)]
    fn foo(c: foo::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        bar::spawn().unwrap();
        *c.shared.counter += 1;
        let counter = *c.shared.counter;
        ::cortex_m_semihosting::export::hstdout_fmt(
                format_args!("  foo = {0}\n", counter),
            )
            .unwrap();
    }
    /// #user_software_tasks
    /// #root
    struct Shared {
        r_shared: i32,
        r_lock_free: i32,
        r_only_shared: i32,
    }
    struct Local {
        r_local: i32,
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
    /// #mod_shared_resources
    mod shared_resources {
        use rtic::export::Priority;
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct r_shared_that_needs_to_be_locked<'a> {
            priority: &'a Priority,
        }
        impl<'a> r_shared_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a Priority) -> Self {
                r_shared_that_needs_to_be_locked {
                    priority,
                }
            }
            #[inline(always)]
            pub unsafe fn priority(&self) -> &Priority {
                self.priority
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct r_only_shared_that_needs_to_be_locked<'a> {
            priority: &'a Priority,
        }
        impl<'a> r_only_shared_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a Priority) -> Self {
                r_only_shared_that_needs_to_be_locked {
                    priority,
                }
            }
            #[inline(always)]
            pub unsafe fn priority(&self) -> &Priority {
                self.priority
            }
        }
    }
    /// #mod_local_resources
    /// #root_hardware_tasks
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `foo` has access to
    pub struct __rtic_internal_fooLocalResources<'a> {
        pub r_local: &'a mut i32,
        pub r_late_local: &'a mut i32,
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `foo` has access to
    pub struct __rtic_internal_fooSharedResources<'a> {
        pub r_shared: shared_resources::r_shared_that_needs_to_be_locked<'a>,
        pub r_lock_free: &'a mut i32,
        pub r_only_shared: &'a i32,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_foo_Context<'a> {
        /// Local Resources this task has access to
        pub local: foo::LocalResources<'a>,
        /// Shared Resources this task has access to
        pub shared: foo::SharedResources<'a>,
    }
    impl<'a> __rtic_internal_foo_Context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_foo_Context {
                local: foo::LocalResources::new(),
                shared: foo::SharedResources::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod foo {
        #[doc(inline)]
        pub use super::__rtic_internal_fooLocalResources as LocalResources;
        #[doc(inline)]
        pub use super::__rtic_internal_fooSharedResources as SharedResources;
        pub use super::__rtic_internal_foo_Context as Context;
    }
    /// #user_software_tasks
    /// app module
    /// #mod_app_shared_resources
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic0"]
    static __rtic_internal_shared_resource_r_shared: rtic::RacyCell<
        core::mem::MaybeUninit<i32>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    impl<'a> rtic::Mutex for shared_resources::r_shared_that_needs_to_be_locked<'a> {
        type T = i32;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut i32) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            /// Priority ceiling
            const CEILING: u8 = 1u8;
            unsafe {
                rtic::export::lock(
                    __rtic_internal_shared_resource_r_shared.get_mut() as *mut _,
                    self.priority(),
                    CEILING,
                    lm3s6965::NVIC_PRIO_BITS,
                    &__rtic_internal_MASKS,
                    f,
                )
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic1"]
    static __rtic_internal_shared_resource_r_lock_free: rtic::RacyCell<
        core::mem::MaybeUninit<i32>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic2"]
    static __rtic_internal_shared_resource_r_only_shared: rtic::RacyCell<
        core::mem::MaybeUninit<i32>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    impl<'a> rtic::Mutex for shared_resources::r_only_shared_that_needs_to_be_locked<'a> {
        type T = i32;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut i32) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            /// Priority ceiling
            const CEILING: u8 = 1u8;
            unsafe {
                rtic::export::lock(
                    __rtic_internal_shared_resource_r_only_shared.get_mut() as *mut _,
                    self.priority(),
                    CEILING,
                    lm3s6965::NVIC_PRIO_BITS,
                    &__rtic_internal_MASKS,
                    f,
                )
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_internal_MASK_CHUNKS: usize = rtic::export::compute_mask_chunks([
        lm3s6965::Interrupt::GPIOA as u32,
    ]);
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_internal_MASKS: [rtic::export::Mask<__rtic_internal_MASK_CHUNKS>; 3] = [
        rtic::export::create_mask([lm3s6965::Interrupt::GPIOA as u32]),
        rtic::export::create_mask([]),
        rtic::export::create_mask([]),
    ];
    /// #mod_app_local_resources
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic3"]
    static __rtic_internal_local_resource_r_local: rtic::RacyCell<
        core::mem::MaybeUninit<i32>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_local_foo_r_late_local: rtic::RacyCell<i32> = rtic::RacyCell::new(
        0,
    );
    /// #mod_app_hardware_tasks
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn GPIOA() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(
            PRIORITY,
            || { foo(foo::Context::new(&rtic::export::Priority::new(PRIORITY))) },
        );
    }
    impl<'a> __rtic_internal_fooLocalResources<'a> {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_internal_fooLocalResources {
                r_local: &mut *(&mut *__rtic_internal_local_resource_r_local.get_mut())
                    .as_mut_ptr(),
                r_late_local: &mut *__rtic_internal_local_foo_r_late_local.get_mut(),
            }
        }
    }
    impl<'a> __rtic_internal_fooSharedResources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_fooSharedResources {
                r_shared: shared_resources::r_shared_that_needs_to_be_locked::new(
                    priority,
                ),
                r_lock_free: &mut *(&mut *__rtic_internal_shared_resource_r_lock_free
                    .get_mut())
                    .as_mut_ptr(),
                r_only_shared: &*(&*__rtic_internal_shared_resource_r_only_shared.get())
                    .as_ptr(),
            }
        }
    }
    /// #user_software_tasks
    /// #mod_app_dispatchers
    /// #mod_app_timer_queue
    #[doc(hidden)]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            rtic::export::assert_send::<i32>();
            const _CONST_CHECK: () = {
                if !rtic::export::have_basepri() {
                    if (lm3s6965::Interrupt::GPIOA as usize)
                        >= (__rtic_internal_MASK_CHUNKS * 32)
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
            rtic::export::interrupt::disable();
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
                .into();
            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 1u8 as usize {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Maximum priority used by interrupt vector \'GPIOA\' is more than supported by hardware",
                        ),
                    );
                };
            };
            core.NVIC
                .set_priority(
                    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOA,
                    rtic::export::logical2hw(1u8, lm3s6965::NVIC_PRIO_BITS),
                );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOA,
            );
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
                __rtic_internal_shared_resource_r_shared
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(shared_resources.r_shared));
                __rtic_internal_shared_resource_r_lock_free
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(shared_resources.r_lock_free));
                __rtic_internal_shared_resource_r_only_shared
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(shared_resources.r_only_shared));
                __rtic_internal_local_resource_r_local
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(local_resources.r_local));
                rtic::export::interrupt::enable();
            });
            loop {
                rtic::export::nop()
            }
        }
    }
}
