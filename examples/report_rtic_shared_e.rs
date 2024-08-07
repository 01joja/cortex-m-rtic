#![feature(prelude_import)]
//! examples/lock.rs
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
        (Shared { a_shared: 0 }, Local {}, init::Monotonics())
    }
    /// #user_hardware_tasks
    /// #user_software_tasks
    #[allow(non_snake_case)]
    fn foo(mut c: foo::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("A\n").unwrap();
        c.shared
            .a_shared
            .lock(|a_shared| {
                bar::spawn().unwrap();
                *a_shared += 1;
                ::cortex_m_semihosting::export::hstdout_fmt(
                        format_args!("B - shared = {0}\n", *a_shared),
                    )
                    .unwrap();
                baz::spawn().unwrap();
            });
        ::cortex_m_semihosting::export::hstdout_str("E\n").unwrap();
        debug::exit(debug::EXIT_SUCCESS);
    }
    #[allow(non_snake_case)]
    fn bar(mut c: bar::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        let shared = c
            .shared
            .a_shared
            .lock(|a_shared| {
                *a_shared += 1;
                *a_shared
            });
        ::cortex_m_semihosting::export::hstdout_fmt(
                format_args!("D - shared = {0}\n", shared),
            )
            .unwrap();
    }
    #[allow(non_snake_case)]
    fn baz(_: baz::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("C\n").unwrap();
    }
    /// #root
    struct Shared {
        a_shared: u32,
    }
    struct Local {}
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
        pub struct a_shared_that_needs_to_be_locked<'a> {
            priority: &'a Priority,
        }
        impl<'a> a_shared_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a Priority) -> Self {
                a_shared_that_needs_to_be_locked {
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
    /// #user_software_tasks
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `foo` has access to
    pub struct __rtic_internal_fooSharedResources<'a> {
        pub a_shared: shared_resources::a_shared_that_needs_to_be_locked<'a>,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_foo_Context<'a> {
        /// Shared Resources this task has access to
        pub shared: foo::SharedResources<'a>,
    }
    impl<'a> __rtic_internal_foo_Context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_foo_Context {
                shared: foo::SharedResources::new(priority),
            }
        }
    }
    /// Spawns the task directly
    pub fn __rtic_internal_foo_spawn() -> Result<(), ()> {
        let input = ();
        unsafe {
            if let Some(index) = rtic::export::interrupt::free(|_| {
                (&mut *__rtic_internal_foo_FQ.get_mut()).dequeue()
            }) {
                (&mut *__rtic_internal_foo_INPUTS.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_internal_P1_RQ.get_mut())
                        .enqueue_unchecked((P1_T::foo, index));
                });
                rtic::pend(lm3s6965::interrupt::GPIOC);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    #[allow(non_snake_case)]
    ///Software task
    pub mod foo {
        #[doc(inline)]
        pub use super::__rtic_internal_fooSharedResources as SharedResources;
        pub use super::__rtic_internal_foo_Context as Context;
        pub use super::__rtic_internal_foo_spawn as spawn;
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `bar` has access to
    pub struct __rtic_internal_barSharedResources<'a> {
        pub a_shared: shared_resources::a_shared_that_needs_to_be_locked<'a>,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_bar_Context<'a> {
        /// Shared Resources this task has access to
        pub shared: bar::SharedResources<'a>,
    }
    impl<'a> __rtic_internal_bar_Context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_bar_Context {
                shared: bar::SharedResources::new(priority),
            }
        }
    }
    /// Spawns the task directly
    pub fn __rtic_internal_bar_spawn() -> Result<(), ()> {
        let input = ();
        unsafe {
            if let Some(index) = rtic::export::interrupt::free(|_| {
                (&mut *__rtic_internal_bar_FQ.get_mut()).dequeue()
            }) {
                (&mut *__rtic_internal_bar_INPUTS.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_internal_P2_RQ.get_mut())
                        .enqueue_unchecked((P2_T::bar, index));
                });
                rtic::pend(lm3s6965::interrupt::GPIOB);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    #[allow(non_snake_case)]
    ///Software task
    pub mod bar {
        #[doc(inline)]
        pub use super::__rtic_internal_barSharedResources as SharedResources;
        pub use super::__rtic_internal_bar_Context as Context;
        pub use super::__rtic_internal_bar_spawn as spawn;
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_baz_Context {}
    impl __rtic_internal_baz_Context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_internal_baz_Context {}
        }
    }
    /// Spawns the task directly
    pub fn __rtic_internal_baz_spawn() -> Result<(), ()> {
        let input = ();
        unsafe {
            if let Some(index) = rtic::export::interrupt::free(|_| {
                (&mut *__rtic_internal_baz_FQ.get_mut()).dequeue()
            }) {
                (&mut *__rtic_internal_baz_INPUTS.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_internal_P3_RQ.get_mut())
                        .enqueue_unchecked((P3_T::baz, index));
                });
                rtic::pend(lm3s6965::interrupt::GPIOA);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    #[allow(non_snake_case)]
    ///Software task
    pub mod baz {
        pub use super::__rtic_internal_baz_Context as Context;
        pub use super::__rtic_internal_baz_spawn as spawn;
    }
    /// app module
    /// #mod_app_shared_resources
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic0"]
    static __rtic_internal_shared_resource_a_shared: rtic::RacyCell<
        core::mem::MaybeUninit<u32>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    impl<'a> rtic::Mutex for shared_resources::a_shared_that_needs_to_be_locked<'a> {
        type T = u32;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut u32) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            /// Priority ceiling
            const CEILING: u8 = 2u8;
            unsafe {
                rtic::export::lock(
                    __rtic_internal_shared_resource_a_shared.get_mut() as *mut _,
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
        lm3s6965::Interrupt::GPIOC as u32,
        lm3s6965::Interrupt::GPIOB as u32,
        lm3s6965::Interrupt::GPIOA as u32,
    ]);
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_internal_MASKS: [rtic::export::Mask<__rtic_internal_MASK_CHUNKS>; 3] = [
        rtic::export::create_mask([lm3s6965::Interrupt::GPIOC as u32]),
        rtic::export::create_mask([lm3s6965::Interrupt::GPIOB as u32]),
        rtic::export::create_mask([lm3s6965::Interrupt::GPIOA as u32]),
    ];
    /// #mod_app_local_resources
    /// #mod_app_hardware_tasks
    /// #user_software_tasks
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_foo_FQ: rtic::RacyCell<rtic::export::SCFQ<2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[link_section = ".uninit.rtic1"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_foo_INPUTS: rtic::RacyCell<[core::mem::MaybeUninit<()>; 1]> = rtic::RacyCell::new([
        core::mem::MaybeUninit::uninit(),
    ]);
    impl<'a> __rtic_internal_fooSharedResources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_fooSharedResources {
                a_shared: shared_resources::a_shared_that_needs_to_be_locked::new(
                    priority,
                ),
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_bar_FQ: rtic::RacyCell<rtic::export::SCFQ<2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[link_section = ".uninit.rtic2"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_bar_INPUTS: rtic::RacyCell<[core::mem::MaybeUninit<()>; 1]> = rtic::RacyCell::new([
        core::mem::MaybeUninit::uninit(),
    ]);
    impl<'a> __rtic_internal_barSharedResources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_barSharedResources {
                a_shared: shared_resources::a_shared_that_needs_to_be_locked::new(
                    priority,
                ),
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_baz_FQ: rtic::RacyCell<rtic::export::SCFQ<2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[link_section = ".uninit.rtic3"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_baz_INPUTS: rtic::RacyCell<[core::mem::MaybeUninit<()>; 1]> = rtic::RacyCell::new([
        core::mem::MaybeUninit::uninit(),
    ]);
    /// #mod_app_dispatchers
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum P1_T {
        foo,
    }
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for P1_T {
        #[inline]
        fn clone(&self) -> P1_T {
            *self
        }
    }
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for P1_T {}
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    static __rtic_internal_P1_RQ: rtic::RacyCell<rtic::export::SCRQ<P1_T, 2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[allow(non_snake_case)]
    ///Interrupt handler to dispatch tasks at priority 1
    #[no_mangle]
    unsafe fn GPIOC() {
        /// The priority of this interrupt handler
        const PRIORITY: u8 = 1u8;
        rtic::export::run(
            PRIORITY,
            || {
                while let Some((task, index)) = (&mut *__rtic_internal_P1_RQ.get_mut())
                    .split()
                    .1
                    .dequeue()
                {
                    match task {
                        P1_T::foo => {
                            let () = (&*__rtic_internal_foo_INPUTS.get())
                                .get_unchecked(usize::from(index))
                                .as_ptr()
                                .read();
                            (&mut *__rtic_internal_foo_FQ.get_mut())
                                .split()
                                .0
                                .enqueue_unchecked(index);
                            let priority = &rtic::export::Priority::new(PRIORITY);
                            foo(foo::Context::new(priority))
                        }
                    }
                }
            },
        );
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum P2_T {
        bar,
    }
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for P2_T {
        #[inline]
        fn clone(&self) -> P2_T {
            *self
        }
    }
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for P2_T {}
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    static __rtic_internal_P2_RQ: rtic::RacyCell<rtic::export::SCRQ<P2_T, 2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[allow(non_snake_case)]
    ///Interrupt handler to dispatch tasks at priority 2
    #[no_mangle]
    unsafe fn GPIOB() {
        /// The priority of this interrupt handler
        const PRIORITY: u8 = 2u8;
        rtic::export::run(
            PRIORITY,
            || {
                while let Some((task, index)) = (&mut *__rtic_internal_P2_RQ.get_mut())
                    .split()
                    .1
                    .dequeue()
                {
                    match task {
                        P2_T::bar => {
                            let () = (&*__rtic_internal_bar_INPUTS.get())
                                .get_unchecked(usize::from(index))
                                .as_ptr()
                                .read();
                            (&mut *__rtic_internal_bar_FQ.get_mut())
                                .split()
                                .0
                                .enqueue_unchecked(index);
                            let priority = &rtic::export::Priority::new(PRIORITY);
                            bar(bar::Context::new(priority))
                        }
                    }
                }
            },
        );
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum P3_T {
        baz,
    }
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for P3_T {
        #[inline]
        fn clone(&self) -> P3_T {
            *self
        }
    }
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for P3_T {}
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    static __rtic_internal_P3_RQ: rtic::RacyCell<rtic::export::SCRQ<P3_T, 2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[allow(non_snake_case)]
    ///Interrupt handler to dispatch tasks at priority 3
    #[no_mangle]
    unsafe fn GPIOA() {
        /// The priority of this interrupt handler
        const PRIORITY: u8 = 3u8;
        rtic::export::run(
            PRIORITY,
            || {
                while let Some((task, index)) = (&mut *__rtic_internal_P3_RQ.get_mut())
                    .split()
                    .1
                    .dequeue()
                {
                    match task {
                        P3_T::baz => {
                            let () = (&*__rtic_internal_baz_INPUTS.get())
                                .get_unchecked(usize::from(index))
                                .as_ptr()
                                .read();
                            (&mut *__rtic_internal_baz_FQ.get_mut())
                                .split()
                                .0
                                .enqueue_unchecked(index);
                            let priority = &rtic::export::Priority::new(PRIORITY);
                            baz(baz::Context::new(priority))
                        }
                    }
                }
            },
        );
    }
    /// #mod_app_timer_queue
    #[doc(hidden)]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            rtic::export::assert_send::<u32>();
            const _CONST_CHECK: () = { if !rtic::export::have_basepri() {} else {} };
            let _ = _CONST_CHECK;
            rtic::export::interrupt::disable();
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_internal_foo_FQ.get_mut()).enqueue_unchecked(i)
                });
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_internal_bar_FQ.get_mut()).enqueue_unchecked(i)
                });
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_internal_baz_FQ.get_mut()).enqueue_unchecked(i)
                });
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
                .into();
            let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOA;
            let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOB;
            let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOC;
            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 1u8 as usize {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Maximum priority used by interrupt vector \'GPIOC\' is more than supported by hardware",
                        ),
                    );
                };
            };
            core.NVIC
                .set_priority(
                    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOC,
                    rtic::export::logical2hw(1u8, lm3s6965::NVIC_PRIO_BITS),
                );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOC,
            );
            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 2u8 as usize {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Maximum priority used by interrupt vector \'GPIOB\' is more than supported by hardware",
                        ),
                    );
                };
            };
            core.NVIC
                .set_priority(
                    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOB,
                    rtic::export::logical2hw(2u8, lm3s6965::NVIC_PRIO_BITS),
                );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOB,
            );
            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 3u8 as usize {
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
                    rtic::export::logical2hw(3u8, lm3s6965::NVIC_PRIO_BITS),
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
                __rtic_internal_shared_resource_a_shared
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(shared_resources.a_shared));
                rtic::export::interrupt::enable();
            });
            loop {
                rtic::export::nop()
            }
        }
    }
}
