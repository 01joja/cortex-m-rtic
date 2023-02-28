
#![feature(prelude_import, core_panic, rustc_private,const_fmt_arguments_new)]
//! examples/message_passing.rs
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
use panic_semihosting as _;
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use lm3s6965 as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    /// #user_imports
    use cortex_m_semihosting::{debug, hprintln};
    /// #user_code
    /// Context needed to pass local and shared resources to their respective task.
    /// All software tasks belonging to prio X
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum __rtic_dispatcher_for_priority_1 {
        foo,
    }
    /// Implements rtic clone
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for __rtic_dispatcher_for_priority_1 {
        #[inline]
        fn clone(&self) -> __rtic_dispatcher_for_priority_1 {
            *self
        }
    }
    /// Implements rtic copy
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for __rtic_dispatcher_for_priority_1 {}
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    static __rtic_dispatcher_request_queue_1: rtic::RacyCell<
        rtic::export::SCRQ<__rtic_dispatcher_for_priority_1, 4>,
    > = rtic::RacyCell::new(rtic::export::Queue::new());
    /// Software task as a function
    #[allow(non_snake_case)]
    fn foo(_c: foo::Context, x: i32, y: u32) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_fmt(format_args!("foo {0}, {1}\n", x, y))
            .unwrap();
        if x == 2 {
            debug::exit(debug::EXIT_SUCCESS);
        }
    }
    /// Queue version of a free-list that keeps track of empty slots in
    /// the following buffers
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_sw_task_foo_function_queue: rtic::RacyCell<rtic::export::SCFQ<4>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    /// Binds internal task overhead to the user defined task.
    pub mod foo {
        pub use super::__rtic_sw_task_foo_context as Context;
        pub use super::__rtic_sw_task_foo_spawn as spawn;
    }
    /// Queue that holds messages for the message passing
    #[allow(unsafe_code)]
    #[link_section = ".uninit.rtic_foo"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_sw_task_foo_input_queue: rtic::RacyCell<
        [core::mem::MaybeUninit<(i32, u32)>; 3],
    > = rtic::RacyCell::new([
        core::mem::MaybeUninit::uninit(),
        core::mem::MaybeUninit::uninit(),
        core::mem::MaybeUninit::uninit(),
    ]);
    /// internal task context (only priority for now)
    pub struct __rtic_sw_task_foo_context {}
    #[allow(unsafe_code)]
    impl __rtic_sw_task_foo_context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_sw_task_foo_context {}
        }
    }
    /// internal spawn function for task
    #[allow(unsafe_code)]
    pub fn __rtic_sw_task_foo_spawn(_0: i32, _1: u32) -> Result<(), (i32, u32)> {
        let input = (_0, _1);
        unsafe {
            if let Some(index)
                = rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_sw_task_foo_function_queue.get_mut()).dequeue()
                }) {
                (&mut *__rtic_sw_task_foo_input_queue.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_dispatcher_request_queue_1.get_mut())
                        .enqueue_unchecked((
                            __rtic_dispatcher_for_priority_1::foo,
                            index,
                        ));
                });
                rtic::pend(lm3s6965::interrupt::SSI0);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    /// #user_init
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        foo::spawn(1, 1).unwrap();
        foo::spawn(1, 2).unwrap();
        foo::spawn(2, 3).unwrap();
        if !foo::spawn(1, 4).is_err() {
            ::core::panicking::panic("assertion failed: foo::spawn(1, 4).is_err()")
        }
        (Shared {}, Local {}, init::Monotonics())
    }
    /// #user_idle
    /// #user_hardware_tasks
    #[allow(unsafe_code)]
    #[allow(non_snake_case)]
    fn __rtic_dispatcher_SSI0() {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        unsafe {
            const PRIORITY: u8 = 1u8;
            rtic::export::run(
                PRIORITY,
                || {
                    while let Some((task, index))
                        = (&mut *__rtic_dispatcher_request_queue_1.get_mut())
                            .split()
                            .1
                            .dequeue()
                    {
                        match task {
                            __rtic_dispatcher_for_priority_1::foo => {
                                let (_0, _1) = (&*__rtic_sw_task_foo_input_queue.get())
                                    .get_unchecked(usize::from(index))
                                    .as_ptr()
                                    .read();
                                (&mut *__rtic_sw_task_foo_function_queue.get_mut())
                                    .split()
                                    .0
                                    .enqueue_unchecked(index);
                                let priority = &rtic::export::Priority::new(PRIORITY);
                                foo(foo::Context::new(priority), _0, _1)
                            }
                        }
                    }
                },
            );
        }
    }
    /// #root_init
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
    #[allow(unsafe_code)]
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
    /// #root_hardware_tasks
    /// #mod_app_idle
    /// #mod_app_hardware_tasks
    #[allow(non_snake_case)]
    #[no_mangle]
    #[allow(unsafe_code)]
    unsafe fn SSI0() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(PRIORITY, || { __rtic_dispatcher_SSI0() });
    }
    /// #main
    #[doc(hidden)]
    mod rtic_ext {
        use super::*;
        #[allow(unsafe_code)]
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            rtic::export::interrupt::disable();
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
                .into();
            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 1u8 as usize {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'SSI0\' is more than supported by hardware"
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
            (0..3u8)
                .for_each(|i| {
                    (&mut *__rtic_sw_task_foo_function_queue.get_mut())
                        .enqueue_unchecked(i)
                });
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
