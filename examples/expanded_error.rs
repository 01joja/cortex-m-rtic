#![feature(prelude_import)]
//! examples/locals.rs
// #![deny(unsafe_code)]
// #![deny(warnings)]
#![feature(const_fmt_arguments_new)]
#![feature(rustc_private)]
#![feature(core_panic)]
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
    /// #user_imports
    use cortex_m_semihosting::{debug, hprintln};

    pub struct FooLocal<'a> {
        pub local_to_foo: &'a mut i64,
    }

    pub struct FooContext<'a> {
        pub local: FooLocal<'a>,
    }

    pub struct BarLocal<'a> {
        pub local_to_bar: &'a mut i64,
    }

    pub struct BarContext<'a> {
        pub local: BarLocal<'a>,
    }

    // struct foo_resources {
    //     local: Local,
    //     shared: Shared
    // }

    // ///Local resources `__rtic_dispatcher_UART0` has access to
    // pub struct __rtic_internal___rtic_dispatcher_UART0LocalResources<'a> {
    //     pub local_to_foo: &'a mut i64,
    //     pub local_to_bar: &'a mut i64,
    // }

    /// #user_code
    fn __rtic_dispatcher_UART0_unsafe(context: __rtic_dispatcher_UART0::Context) {
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
                                let () = (&*__rtic_sw_task_foo_input_queue.get())
                                    .get_unchecked(usize::from(index))
                                    .as_ptr()
                                    .read();
                                (&mut *__rtic_sw_task_foo_function_queue.get_mut())
                                    .split()
                                    .0
                                    .enqueue_unchecked(index);
                                let foo_context = FooContext{
                                    local: FooLocal{
                                        local_to_foo: context.local.local_to_foo
                                    }
                                };
                                foo(foo_context)
                            }
                            __rtic_dispatcher_for_priority_1::bar => {
                                let () = (&*__rtic_sw_task_bar_input_queue.get())
                                    .get_unchecked(usize::from(index))
                                    .as_ptr()
                                    .read();
                                (&mut *__rtic_sw_task_bar_function_queue.get_mut())
                                    .split()
                                    .0
                                    .enqueue_unchecked(index);
                                let bar_context = BarContext{
                                    local: BarLocal{
                                        local_to_bar: context.local.local_to_bar
                                    }
                                };
                                bar(bar_context)
                            }
                        }
                    }
                },
            );
        }
    }
    /// All software tasks belonging to prio X
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum __rtic_dispatcher_for_priority_1 {
        foo,
        bar,
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
        rtic::export::SCRQ<__rtic_dispatcher_for_priority_1, 3>,
    > = rtic::RacyCell::new(rtic::export::Queue::new());
    /// #software_tasks
    /// Software task as a function
    #[allow(non_snake_case)]
    fn foo(cx: FooContext){
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        let local_to_foo = cx.local.local_to_foo;
        *local_to_foo += 1;
        ::cortex_m_semihosting::export::hstdout_fmt(
                format_args!("foo: local_to_foo = {0}\n", local_to_foo),
            )
            .unwrap();
    }
    /// Queue version of a free-list that keeps track of empty slots in
    /// the following buffers
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_sw_task_foo_function_queue: rtic::RacyCell<rtic::export::SCFQ<2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    /// Binds internal task overhead to the user defined task.
    pub mod foo {
        pub use super::__rtic_sw_task_foo_spawn as spawn;
    }
    /// Queue that holds messages for the message passing
    #[link_section = ".uninit.rtic_foo"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_sw_task_foo_input_queue: rtic::RacyCell<
        [core::mem::MaybeUninit<()>; 1],
    > = rtic::RacyCell::new([core::mem::MaybeUninit::uninit()]);
    /// internal spawn function for task
    pub fn __rtic_sw_task_foo_spawn() -> Result<(), ()> {
        let input = ();
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
                rtic::pend(lm3s6965::interrupt::UART0);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    /// Software task as a function
    #[allow(non_snake_case)]
    fn bar(cx:  BarContext){
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        foo::spawn().unwrap();
        let local_to_bar = cx.local.local_to_bar;
        *local_to_bar += 1;
        ::cortex_m_semihosting::export::hstdout_fmt(
                format_args!("bar: local_to_bar = {0}\n", local_to_bar),
            )
            .unwrap();
    }
    /// Queue version of a free-list that keeps track of empty slots in
    /// the following buffers
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_sw_task_bar_function_queue: rtic::RacyCell<rtic::export::SCFQ<2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    /// Binds internal task overhead to the user defined task.
    pub mod bar {
        pub use super::__rtic_sw_task_bar_spawn as spawn;
    }
    /// Queue that holds messages for the message passing
    #[link_section = ".uninit.rtic_bar"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_sw_task_bar_input_queue: rtic::RacyCell<
        [core::mem::MaybeUninit<()>; 1],
    > = rtic::RacyCell::new([core::mem::MaybeUninit::uninit()]);
    /// internal spawn function for task
    pub fn __rtic_sw_task_bar_spawn() -> Result<(), ()> {
        let input = ();
        unsafe {
            if let Some(index)
                = rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_sw_task_bar_function_queue.get_mut()).dequeue()
                }) {
                (&mut *__rtic_sw_task_bar_input_queue.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_dispatcher_request_queue_1.get_mut())
                        .enqueue_unchecked((
                            __rtic_dispatcher_for_priority_1::bar,
                            index,
                        ));
                });
                rtic::pend(lm3s6965::interrupt::UART0);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    /// #init_software
    fn init_software() {
        unsafe {
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_sw_task_foo_function_queue.get_mut())
                        .enqueue_unchecked(i)
                });
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_sw_task_bar_function_queue.get_mut())
                        .enqueue_unchecked(i)
                });
        }
    }
    /// #user_init
    /// #user_init
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        init_software();
        foo::spawn().unwrap();
        bar::spawn().unwrap();
        (
            Shared {},
            Local {
                local_to_foo: 0,
                local_to_bar: 0,
                local_to_idle: 0,
            },
            init::Monotonics(),
        )
    }
    /// #user_idle
    /// #user_idle
    #[allow(non_snake_case)]
    #[allow(non_snake_case)]
    fn idle(cx: idle::Context) -> ! {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        let local_to_idle = cx.local.local_to_idle;
        *local_to_idle += 1;
        ::cortex_m_semihosting::export::hstdout_fmt(
                format_args!("idle: local_to_idle = {0}\n", local_to_idle),
            )
            .unwrap();
        debug::exit(debug::EXIT_SUCCESS);
        loop {
            cortex_m::asm::nop();
        }
    }
    /// #user_hardware_tasks
    #[allow(non_snake_case)]
    fn __rtic_dispatcher_UART0(context: __rtic_dispatcher_UART0::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        __rtic_dispatcher_UART0_unsafe(context);
    }
    /// #root_init
    struct Shared {}
    struct Local {
        local_to_foo: i64,
        local_to_bar: i64,
        local_to_idle: i64,
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
    #[allow(non_camel_case_types)]
    ///Local resources `idle` has access to
    pub struct __rtic_internal_idleLocalResources {
        pub local_to_idle: &'static mut i64,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_idle_idle_context {
        /// Local Resources this task has access to
        pub local: idle::LocalResources,
    }
    impl __rtic_idle_idle_context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_idle_idle_context {
                local: idle::LocalResources::new(),
            }
        }
    }
    #[allow(non_snake_case)]
    ///idle loop
    pub mod idle {
        pub use super::__rtic_idle_idle_context as Context;
        #[doc(inline)]
        pub use super::__rtic_internal_idleLocalResources as LocalResources;
    }
    /// #mod_shared_resources
    /// #mod_local_resources
    /// #root_hardware_tasks
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `__rtic_dispatcher_UART0` has access to
    pub struct __rtic_internal___rtic_dispatcher_UART0LocalResources<'a> {
        pub local_to_foo: &'a mut i64,
        pub local_to_bar: &'a mut i64,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_idle___rtic_dispatcher_UART0_context<'a> {
        /// Local Resources this task has access to
        pub local: __rtic_dispatcher_UART0::LocalResources<'a>,
    }
    impl<'a> __rtic_idle___rtic_dispatcher_UART0_context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_idle___rtic_dispatcher_UART0_context {
                local: __rtic_dispatcher_UART0::LocalResources::new(),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod __rtic_dispatcher_UART0 {
        #[doc(inline)]
        pub use super::__rtic_internal___rtic_dispatcher_UART0LocalResources as LocalResources;
        pub use super::__rtic_idle___rtic_dispatcher_UART0_context as Context;
    }
    /// #mod_app_init
    /// #mod_app_idle
    impl __rtic_internal_idleLocalResources {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_internal_idleLocalResources {
                local_to_idle: &mut *(&mut *__rtic_internal_local_resource_local_to_idle
                    .get_mut())
                    .as_mut_ptr(),
            }
        }
    }
    /// #mod_app_shared_resources
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
    /// #mod_app_local_resources
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic0"]
    static __rtic_internal_local_resource_local_to_foo: rtic::RacyCell<
        core::mem::MaybeUninit<i64>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic1"]
    static __rtic_internal_local_resource_local_to_bar: rtic::RacyCell<
        core::mem::MaybeUninit<i64>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic2"]
    static __rtic_internal_local_resource_local_to_idle: rtic::RacyCell<
        core::mem::MaybeUninit<i64>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    /// #mod_app_hardware_tasks
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn UART0() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(
            PRIORITY,
            || {
                __rtic_dispatcher_UART0(
                    __rtic_dispatcher_UART0::Context::new(
                        &rtic::export::Priority::new(PRIORITY),
                    ),
                )
            },
        );
    }
    impl<'a> __rtic_internal___rtic_dispatcher_UART0LocalResources<'a> {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_internal___rtic_dispatcher_UART0LocalResources {
                local_to_foo: &mut *(&mut *__rtic_internal_local_resource_local_to_foo
                    .get_mut())
                    .as_mut_ptr(),
                local_to_bar: &mut *(&mut *__rtic_internal_local_resource_local_to_bar
                    .get_mut())
                    .as_mut_ptr(),
            }
        }
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
                let (shared_resources, local_resources, mut monotonics) = init(
                    init::Context::new(core.into()),
                );
                __rtic_internal_local_resource_local_to_foo
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(local_resources.local_to_foo));
                __rtic_internal_local_resource_local_to_bar
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(local_resources.local_to_bar));
                __rtic_internal_local_resource_local_to_idle
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(local_resources.local_to_idle));
                rtic::export::interrupt::enable();
            });
            idle(idle::Context::new(&rtic::export::Priority::new(0)))
        }
    }
}
