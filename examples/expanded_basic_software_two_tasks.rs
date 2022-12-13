#![feature(prelude_import,fmt_internals,core_panic,const_fmt_arguments_new)]
//! examples/init.rs
//#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]
#[prelude_import]
//use core::prelude::rust_2021::*;
//#[macro_use]
extern crate core;
//#[macro_use]
//extern crate compiler_builtins;

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
        bar::spawn().unwrap();
        foo::spawn().unwrap();
        ::cortex_m_semihosting::export::hstdout_str("init\n").unwrap();
        (Shared {}, Local {}, init::Monotonics())
    }
    /// #user_hardware_tasks
    /// #user_software_tasks
    #[allow(non_snake_case)]
    fn foo(_: foo::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("foo\n").unwrap();
        debug::exit(debug::EXIT_SUCCESS);
    }
    #[allow(non_snake_case)]
    fn bar(_: bar::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("bar\n").unwrap();
    }
    #[allow(non_snake_case)]
    fn do_not_run(_: do_not_run::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("don\'t run\n").unwrap();
    }
    /// #root
    struct Shared {}
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
    /// #mod_local_resources
    /// #root_hardware_tasks
    /// #user_software_tasks
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_foo_Context {}
    impl __rtic_internal_foo_Context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_internal_foo_Context {}
        }
    }
    /// Spawns the task directly
    pub fn __rtic_internal_foo_spawn() -> Result<(), ()> {
        let input = ();
        unsafe {
            if let Some(index)
                = rtic::export::interrupt::free(|_| {
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
                rtic::pend(lm3s6965::interrupt::SSI0);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    #[allow(non_snake_case)]
    ///Software task
    pub mod foo {
        pub use super::__rtic_internal_foo_Context as Context;
        pub use super::__rtic_internal_foo_spawn as spawn;
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_bar_Context {}
    impl __rtic_internal_bar_Context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_internal_bar_Context {}
        }
    }
    /// Spawns the task directly
    pub fn __rtic_internal_bar_spawn() -> Result<(), ()> {
        let input = ();
        unsafe {
            if let Some(index)
                = rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_internal_bar_FQ.get_mut()).dequeue()
                }) {
                (&mut *__rtic_internal_bar_INPUTS.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_internal_P1_RQ.get_mut())
                        .enqueue_unchecked((P1_T::bar, index));
                });
                rtic::pend(lm3s6965::interrupt::SSI0);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    #[allow(non_snake_case)]
    ///Software task
    pub mod bar {
        pub use super::__rtic_internal_bar_Context as Context;
        pub use super::__rtic_internal_bar_spawn as spawn;
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_do_not_run_Context {}
    impl __rtic_internal_do_not_run_Context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_internal_do_not_run_Context {
            }
        }
    }
    /// Spawns the task directly
    pub fn __rtic_internal_do_not_run_spawn() -> Result<(), ()> {
        let input = ();
        unsafe {
            if let Some(index)
                = rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_internal_do_not_run_FQ.get_mut()).dequeue()
                }) {
                (&mut *__rtic_internal_do_not_run_INPUTS.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_internal_P1_RQ.get_mut())
                        .enqueue_unchecked((P1_T::do_not_run, index));
                });
                rtic::pend(lm3s6965::interrupt::SSI0);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    #[allow(non_snake_case)]
    ///Software task
    pub mod do_not_run {
        pub use super::__rtic_internal_do_not_run_Context as Context;
        pub use super::__rtic_internal_do_not_run_spawn as spawn;
    }
    /// app module
    /// #mod_app_shared_resources
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_internal_MASK_CHUNKS: usize = rtic::export::compute_mask_chunks([
        lm3s6965::Interrupt::SSI0 as u32,
    ]);
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_internal_MASKS: [rtic::export::Mask<__rtic_internal_MASK_CHUNKS>; 3] = [
        rtic::export::create_mask([lm3s6965::Interrupt::SSI0 as u32]),
        rtic::export::create_mask([]),
        rtic::export::create_mask([]),
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
    #[link_section = ".uninit.rtic0"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_foo_INPUTS: rtic::RacyCell<[core::mem::MaybeUninit<()>; 1]> = rtic::RacyCell::new([
        core::mem::MaybeUninit::uninit(),
    ]);
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_bar_FQ: rtic::RacyCell<rtic::export::SCFQ<2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[link_section = ".uninit.rtic1"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_bar_INPUTS: rtic::RacyCell<[core::mem::MaybeUninit<()>; 1]> = rtic::RacyCell::new([
        core::mem::MaybeUninit::uninit(),
    ]);
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_do_not_run_FQ: rtic::RacyCell<rtic::export::SCFQ<2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[link_section = ".uninit.rtic2"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_do_not_run_INPUTS: rtic::RacyCell<
        [core::mem::MaybeUninit<()>; 1],
    > = rtic::RacyCell::new([core::mem::MaybeUninit::uninit()]);
    /// #mod_app_dispatchers
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum P1_T {
        bar,
        do_not_run,
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
    static __rtic_internal_P1_RQ: rtic::RacyCell<rtic::export::SCRQ<P1_T, 4>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[allow(non_snake_case)]
    ///Interrupt handler to dispatch tasks at priority 1
    #[no_mangle]
    unsafe fn SSI0() {
        /// The priority of this interrupt handler
        const PRIORITY: u8 = 1u8;
        rtic::export::run(
            PRIORITY,
            || {
                while let Some((task, index))
                    = (&mut *__rtic_internal_P1_RQ.get_mut()).split().1.dequeue()
                {
                    match task {
                        P1_T::bar => {
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
                        P1_T::do_not_run => {
                            let () = (&*__rtic_internal_do_not_run_INPUTS.get())
                                .get_unchecked(usize::from(index))
                                .as_ptr()
                                .read();
                            (&mut *__rtic_internal_do_not_run_FQ.get_mut())
                                .split()
                                .0
                                .enqueue_unchecked(index);
                            let priority = &rtic::export::Priority::new(PRIORITY);
                            do_not_run(do_not_run::Context::new(priority))
                        }
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
    /// #mod_app_timer_queue
    #[doc(hidden)]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
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
                    (&mut *__rtic_internal_do_not_run_FQ.get_mut()).enqueue_unchecked(i)
                });
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
                .into();
            let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0;
            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 1u8 as usize {
                ::core::panicking::panic_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[
                            "Maximum priority used by interrupt vector \'SSI0\' is more than supported by hardware",
                        ],
                        &[],
                    ),
                );
            };
            core.NVIC
                .set_priority(
                    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0,
                    rtic::export::logical2hw(1u8, lm3s6965::NVIC_PRIO_BITS),
                );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0,
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
                rtic::export::interrupt::enable();
            });
            loop {
                rtic::export::nop()
            }
        }
    }
}
