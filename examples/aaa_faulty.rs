// #![feature(prelude_import)]
//! examples/lock.rs
// #![deny(unsafe_code)]
// #![deny(warnings)]
#![feature(panic_internals,const_fmt_arguments_new,rustc_private)]
#![no_main]
#![no_std]
// #[prelude_import]
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
    use cortex_m_semihosting::{debug, hprintln};
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
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_context_baz_context {}
    impl __rtic_context_baz_context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_context_baz_context {}
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `foo` has access to
    pub struct __rtic_shared_resource_foo_shared_resources<'a> {
        pub shared: shared_resources::shared_that_needs_to_be_locked<'a>,
    }
    impl<'a> __rtic_shared_resource_foo_shared_resources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_shared_resource_foo_shared_resources {
                shared: shared_resources::shared_that_needs_to_be_locked::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `bar` has access to
    pub struct __rtic_shared_resource_bar_shared_resources<'a> {
        pub shared: shared_resources::shared_that_needs_to_be_locked<'a>,
    }
    impl<'a> __rtic_shared_resource_bar_shared_resources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_shared_resource_bar_shared_resources {
                shared: shared_resources::shared_that_needs_to_be_locked::new(priority),
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic_r_shared"]
    static __rtic_shared_resource_shared: rtic::RacyCell<core::mem::MaybeUninit<u32>> = rtic::RacyCell::new(
        core::mem::MaybeUninit::uninit(),
    );
    impl<'a> rtic::Mutex for shared_resources::shared_that_needs_to_be_locked<'a> {
        type T = u32;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut u32) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            ::cortex_m_semihosting::export::hstdout_fmt(
                    format_args!("hej!\n"),
                )
                .unwrap();
            /// Priority ceiling
            const CEILING: u8 = 2u8;
            unsafe {
                rtic::export::lock(
                    __rtic_shared_resource_shared.get_mut() as *mut _,
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
        lm3s6965::Interrupt::GPIOC as u32,
        lm3s6965::Interrupt::GPIOB as u32,
        lm3s6965::Interrupt::GPIOA as u32,
    ]);
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_shared_resources_MASKS: [rtic::export::Mask<
        __rtic_shared_resources_MASK_CHUNKS,
    >; 3] = [
        rtic::export::create_mask([lm3s6965::Interrupt::GPIOC as u32]),
        rtic::export::create_mask([lm3s6965::Interrupt::GPIOB as u32]),
        rtic::export::create_mask([lm3s6965::Interrupt::GPIOA as u32]),
    ];
    mod shared_resources {
        use rtic::export::Priority;
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct shared_that_needs_to_be_locked<'a> {
            priority: &'a Priority,
        }
        impl<'a> shared_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a Priority) -> Self {
                shared_that_needs_to_be_locked {
                    priority,
                }
            }
            #[inline(always)]
            pub unsafe fn priority(&self) -> &Priority {
                self.priority
            }
        }
    }
    /// Context needed to pass local and shared resources to their respective task.
    /// All software tasks belonging to prio X
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum __rtic_software_tasks_prio_1 {
        foo,
    }
    /// Implements rtic clone
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for __rtic_software_tasks_prio_1 {
        #[inline]
        fn clone(&self) -> __rtic_software_tasks_prio_1 {
            *self
        }
    }
    /// Implements rtic copy
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for __rtic_software_tasks_prio_1 {}
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    pub static __rtic_dispatcher_request_queue_prio_1: rtic::RacyCell<
        rtic::export::SCRQ<__rtic_software_tasks_prio_1, 2>,
    > = rtic::RacyCell::new(rtic::export::Queue::new());
    /// Context needed to pass local and shared resources to their respective task.
    /// All software tasks belonging to prio X
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum __rtic_software_tasks_prio_3 {
        baz,
    }
    /// Implements rtic clone
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for __rtic_software_tasks_prio_3 {
        #[inline]
        fn clone(&self) -> __rtic_software_tasks_prio_3 {
            *self
        }
    }
    /// Implements rtic copy
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for __rtic_software_tasks_prio_3 {}
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    pub static __rtic_dispatcher_request_queue_prio_3: rtic::RacyCell<
        rtic::export::SCRQ<__rtic_software_tasks_prio_3, 2>,
    > = rtic::RacyCell::new(rtic::export::Queue::new());
    /// Context needed to pass local and shared resources to their respective task.
    /// All software tasks belonging to prio X
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum __rtic_software_tasks_prio_2 {
        bar,
    }
    /// Implements rtic clone
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for __rtic_software_tasks_prio_2 {
        #[inline]
        fn clone(&self) -> __rtic_software_tasks_prio_2 {
            *self
        }
    }
    /// Implements rtic copy
    #[automatically_derived]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for __rtic_software_tasks_prio_2 {}
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    pub static __rtic_dispatcher_request_queue_prio_2: rtic::RacyCell<
        rtic::export::SCRQ<__rtic_software_tasks_prio_2, 2>,
    > = rtic::RacyCell::new(rtic::export::Queue::new());
    /// Software task as a function
    #[allow(non_snake_case)]
    fn foo(mut c: foo::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("A\n").unwrap();
        c.shared
            .shared
            .lock(|shared| {
                *shared += 1;
                bar::spawn().unwrap();
                ::cortex_m_semihosting::export::hstdout_fmt(
                        format_args!("B - shared = {0}\n", *shared),
                    )
                    .unwrap();
                baz::spawn().unwrap();
            });
        ::cortex_m_semihosting::export::hstdout_str("E\n").unwrap();
        debug::exit(debug::EXIT_SUCCESS);
    }
    /// Queue version of a free-list that keeps track of empty slots in
    /// the following buffers
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    pub static __rtic_sw_task_foo_request_queue: rtic::RacyCell<rtic::export::SCFQ<2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    /// Binds internal task overhead to the user defined task.
    /// Also makes it possible for other passes to modify the
    /// function_queue and inputs. (Monotonic pass needs them)
    pub mod foo {
        pub use super::__rtic_context_foo_context as Context;
        pub use super::__rtic_shared_resource_foo_shared_resources as SharedResources;
        pub use super::__rtic_software_tasks_prio_1 as __internal_dispatcher_task_name;
        pub use super::__rtic_dispatcher_request_queue_prio_1 as __internal_PRIO_REQUEST_Q;
        pub use super::__rtic_sw_task_foo_request_queue as __internal_function_queue;
        pub use super::__rtic_sw_task_foo_message_list as __internal_message_list;
        pub use super::__rtic_sw_task_foo_spawn as spawn;
    }
    /// Queue that holds messages for the message passing
    #[link_section = ".uninit.rtic_sw_foo"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    pub static __rtic_sw_task_foo_message_list: rtic::RacyCell<
        [core::mem::MaybeUninit<()>; 1],
    > = rtic::RacyCell::new([core::mem::MaybeUninit::uninit()]);
    /// internal task context (only priority for now)
    pub struct __rtic_sw_task_foo_context {}
    impl __rtic_sw_task_foo_context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_sw_task_foo_context {}
        }
    }
    /// internal spawn function for task
    pub fn __rtic_sw_task_foo_spawn() -> Result<(), ()> {
        let input = ();
        unsafe {
            if let Some(index) = rtic::export::interrupt::free(|_| {
                (&mut *__rtic_sw_task_foo_request_queue.get_mut()).dequeue()
            }) {
                (&mut *__rtic_sw_task_foo_message_list.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_dispatcher_request_queue_prio_1.get_mut())
                        .enqueue_unchecked((__rtic_software_tasks_prio_1::foo, index));
                });
                rtic::pend(lm3s6965::interrupt::GPIOC);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    /// Software task as a function
    #[allow(non_snake_case)]
    fn baz(_: baz::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("C\n").unwrap();
    }
    /// Queue version of a free-list that keeps track of empty slots in
    /// the following buffers
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    pub static __rtic_sw_task_baz_request_queue: rtic::RacyCell<rtic::export::SCFQ<2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    /// Binds internal task overhead to the user defined task.
    /// Also makes it possible for other passes to modify the
    /// function_queue and inputs. (Monotonic pass needs them)
    pub mod baz {
        pub use super::__rtic_context_baz_context as Context;
        pub use super::__rtic_software_tasks_prio_3 as __internal_dispatcher_task_name;
        pub use super::__rtic_dispatcher_request_queue_prio_3 as __internal_PRIO_REQUEST_Q;
        pub use super::__rtic_sw_task_baz_request_queue as __internal_function_queue;
        pub use super::__rtic_sw_task_baz_message_list as __internal_message_list;
        pub use super::__rtic_sw_task_baz_spawn as spawn;
    }
    /// Queue that holds messages for the message passing
    #[link_section = ".uninit.rtic_sw_baz"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    pub static __rtic_sw_task_baz_message_list: rtic::RacyCell<
        [core::mem::MaybeUninit<()>; 1],
    > = rtic::RacyCell::new([core::mem::MaybeUninit::uninit()]);
    /// internal task context (only priority for now)
    pub struct __rtic_sw_task_baz_context {}
    impl __rtic_sw_task_baz_context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_sw_task_baz_context {}
        }
    }
    /// internal spawn function for task
    pub fn __rtic_sw_task_baz_spawn() -> Result<(), ()> {
        let input = ();
        unsafe {
            if let Some(index) = rtic::export::interrupt::free(|_| {
                (&mut *__rtic_sw_task_baz_request_queue.get_mut()).dequeue()
            }) {
                (&mut *__rtic_sw_task_baz_message_list.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_dispatcher_request_queue_prio_3.get_mut())
                        .enqueue_unchecked((__rtic_software_tasks_prio_3::baz, index));
                });
                rtic::pend(lm3s6965::interrupt::GPIOB);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    /// Software task as a function
    #[allow(non_snake_case)]
    fn bar(mut c: bar::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        let shared = c
            .shared
            .shared
            .lock(|shared| {
                *shared += 1;
                *shared
            });
        ::cortex_m_semihosting::export::hstdout_fmt(
                format_args!("D - shared = {0}\n", shared),
            )
            .unwrap();
    }
    /// Queue version of a free-list that keeps track of empty slots in
    /// the following buffers
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    pub static __rtic_sw_task_bar_request_queue: rtic::RacyCell<rtic::export::SCFQ<2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    /// Binds internal task overhead to the user defined task.
    /// Also makes it possible for other passes to modify the
    /// function_queue and inputs. (Monotonic pass needs them)
    pub mod bar {
        pub use super::__rtic_context_bar_context as Context;
        pub use super::__rtic_shared_resource_bar_shared_resources as SharedResources;
        pub use super::__rtic_software_tasks_prio_2 as __internal_dispatcher_task_name;
        pub use super::__rtic_dispatcher_request_queue_prio_2 as __internal_PRIO_REQUEST_Q;
        pub use super::__rtic_sw_task_bar_request_queue as __internal_function_queue;
        pub use super::__rtic_sw_task_bar_message_list as __internal_message_list;
        pub use super::__rtic_sw_task_bar_spawn as spawn;
    }
    /// Queue that holds messages for the message passing
    #[link_section = ".uninit.rtic_sw_bar"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    pub static __rtic_sw_task_bar_message_list: rtic::RacyCell<
        [core::mem::MaybeUninit<()>; 1],
    > = rtic::RacyCell::new([core::mem::MaybeUninit::uninit()]);
    /// internal task context (only priority for now)
    pub struct __rtic_sw_task_bar_context {}
    impl __rtic_sw_task_bar_context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_sw_task_bar_context {}
        }
    }
    /// internal spawn function for task
    pub fn __rtic_sw_task_bar_spawn() -> Result<(), ()> {
        let input = ();
        unsafe {
            if let Some(index) = rtic::export::interrupt::free(|_| {
                (&mut *__rtic_sw_task_bar_request_queue.get_mut()).dequeue()
            }) {
                (&mut *__rtic_sw_task_bar_message_list.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_dispatcher_request_queue_prio_2.get_mut())
                        .enqueue_unchecked((__rtic_software_tasks_prio_2::bar, index));
                });
                rtic::pend(lm3s6965::interrupt::GPIOA);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    /// #user_init
    /// user_init
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        foo::spawn().unwrap();
        (Shared { shared: 0 }, Local {}, init::Monotonics())
    }
    /// #module_init
    struct Shared {
        shared: u32,
    }
    struct Local {}
    mod init {
        pub use super::__rtic_context_init_context as Context;
        pub use super::__rtic_internal_Monotonics as Monotonics;
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_Monotonics();
    /// #user_idle
    /// #module_idle
    /// #user_hardware_tasks
    #[allow(unsafe_code)]
    #[allow(non_snake_case)]
    fn __rtic_dispatcher_GPIOC() {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        unsafe {
            const PRIORITY: u8 = 1u8;
            rtic::export::run(
                PRIORITY,
                || {
                    while let Some((task, index)) = (&mut *__rtic_dispatcher_request_queue_prio_1
                        .get_mut())
                        .split()
                        .1
                        .dequeue()
                    {
                        match task {
                            __rtic_software_tasks_prio_1::foo => {
                                let () = (&*__rtic_sw_task_foo_message_list.get())
                                    .get_unchecked(usize::from(index))
                                    .as_ptr()
                                    .read();
                                (&mut *__rtic_sw_task_foo_request_queue.get_mut())
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
    }
    #[allow(unsafe_code)]
    #[allow(non_snake_case)]
    fn __rtic_dispatcher_GPIOB() {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        unsafe {
            const PRIORITY: u8 = 3u8;
            rtic::export::run(
                PRIORITY,
                || {
                    while let Some((task, index)) = (&mut *__rtic_dispatcher_request_queue_prio_3
                        .get_mut())
                        .split()
                        .1
                        .dequeue()
                    {
                        match task {
                            __rtic_software_tasks_prio_3::baz => {
                                let () = (&*__rtic_sw_task_baz_message_list.get())
                                    .get_unchecked(usize::from(index))
                                    .as_ptr()
                                    .read();
                                (&mut *__rtic_sw_task_baz_request_queue.get_mut())
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
    }
    #[allow(unsafe_code)]
    #[allow(non_snake_case)]
    fn __rtic_dispatcher_GPIOA() {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        unsafe {
            const PRIORITY: u8 = 2u8;
            rtic::export::run(
                PRIORITY,
                || {
                    while let Some((task, index)) = (&mut *__rtic_dispatcher_request_queue_prio_2
                        .get_mut())
                        .split()
                        .1
                        .dequeue()
                    {
                        match task {
                            __rtic_software_tasks_prio_2::bar => {
                                let () = (&*__rtic_sw_task_bar_message_list.get())
                                    .get_unchecked(usize::from(index))
                                    .as_ptr()
                                    .read();
                                (&mut *__rtic_sw_task_bar_request_queue.get_mut())
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
    }
    /// #modules_hardware_tasks
    /// #interrupts_handlers
    #[allow(non_snake_case)]
    #[no_mangle]
    #[allow(unsafe_code)]
    unsafe fn GPIOC() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(PRIORITY, || { __rtic_dispatcher_GPIOC() });
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    #[allow(unsafe_code)]
    unsafe fn GPIOB() {
        const PRIORITY: u8 = 3u8;
        rtic::export::run(PRIORITY, || { __rtic_dispatcher_GPIOB() });
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    #[allow(unsafe_code)]
    unsafe fn GPIOA() {
        const PRIORITY: u8 = 2u8;
        rtic::export::run(PRIORITY, || { __rtic_dispatcher_GPIOA() });
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


            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 3u8 as usize {
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
                    rtic::export::logical2hw(3u8, lm3s6965::NVIC_PRIO_BITS),
                );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOB,
            );


            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 2u8 as usize {
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
                    rtic::export::logical2hw(2u8, lm3s6965::NVIC_PRIO_BITS),
                );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOA,
            );

            
            ///pre_init end
            fn tmp() {}
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_sw_task_foo_request_queue.get_mut())
                        .enqueue_unchecked(i)
                });
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_sw_task_baz_request_queue.get_mut())
                        .enqueue_unchecked(i)
                });
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_sw_task_bar_request_queue.get_mut())
                        .enqueue_unchecked(i)
                });
            rtic::export::assert_send::<u32>();
            const _CONST_CHECK: () = { if !rtic::export::have_basepri() {} else {} };
            let _ = _CONST_CHECK;
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
                __rtic_shared_resource_shared
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(shared_resources.shared));
                rtic::export::interrupt::enable();
            });
            /// main_init.rs
            fn the_end() {}
            loop {
                rtic::export::nop()
            }
        }
    }
}
