#![feature(prelude_import)]
//! examples/spawn.rs
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
    #[local]
    struct Local {}
    #[shared]
    struct Shared {}
    use cortex_m_semihosting::debug;
    #[init()]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        bar::spawn(1).unwrap();
        foo::spawn().unwrap();
        bar::spawn(2).unwrap();
        (Shared {}, Local {}, init::Monotonics())
    }
    #[allow(unsafe_code)]
    #[task(binds = SSI0, priority = 1)]
    fn __rtic_dispatcher_SSI0(_: __rtic_dispatcher_SSI0::Context) {
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
                                foo()
                            }
                            __rtic_software_tasks_prio_1::bar => {
                                let (_0,) = (&*__rtic_sw_task_bar_message_list.get())
                                    .get_unchecked(usize::from(index))
                                    .as_ptr()
                                    .read();
                                (&mut *__rtic_sw_task_bar_request_queue.get_mut())
                                    .split()
                                    .0
                                    .enqueue_unchecked(index);
                                let priority = &rtic::export::Priority::new(PRIORITY);
                                bar(_0)
                            }
                        }
                    }
                },
            );
        }
    }
    /// Context needed to pass local and shared resources to their respective task.
    /// All software tasks belonging to prio X
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum __rtic_software_tasks_prio_1 {
        foo,
        bar,
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
        rtic::export::SCRQ<__rtic_software_tasks_prio_1, 4>,
    > = rtic::RacyCell::new(rtic::export::Queue::new());
    /// Software task as a function
    #[allow(non_snake_case)]
    fn foo() {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
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
                rtic::pend(lm3s6965::interrupt::SSI0);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    /// Software task as a function
    #[allow(non_snake_case)]
    fn bar(x: u32) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        if x == 2 {
            debug::exit(debug::EXIT_SUCCESS);
        }
    }
    /// Queue version of a free-list that keeps track of empty slots in
    /// the following buffers
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    pub static __rtic_sw_task_bar_request_queue: rtic::RacyCell<rtic::export::SCFQ<3>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    /// Binds internal task overhead to the user defined task.
    /// Also makes it possible for other passes to modify the
    /// function_queue and inputs. (Monotonic pass needs them)
    pub mod bar {
        pub use super::__rtic_software_tasks_prio_1 as __internal_dispatcher_task_name;
        pub use super::__rtic_dispatcher_request_queue_prio_1 as __internal_PRIO_REQUEST_Q;
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
        [core::mem::MaybeUninit<(u32,)>; 2],
    > = rtic::RacyCell::new([
        core::mem::MaybeUninit::uninit(),
        core::mem::MaybeUninit::uninit(),
    ]);
    /// internal task context (only priority for now)
    pub struct __rtic_sw_task_bar_context {}
    impl __rtic_sw_task_bar_context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_sw_task_bar_context {}
        }
    }
    /// internal spawn function for task
    pub fn __rtic_sw_task_bar_spawn(_0: u32) -> Result<(), (u32,)> {
        let input = (_0,);
        unsafe {
            if let Some(index) = rtic::export::interrupt::free(|_| {
                (&mut *__rtic_sw_task_bar_request_queue.get_mut()).dequeue()
            }) {
                (&mut *__rtic_sw_task_bar_message_list.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                rtic::export::interrupt::free(|_| {
                    (&mut *__rtic_dispatcher_request_queue_prio_1.get_mut())
                        .enqueue_unchecked((__rtic_software_tasks_prio_1::bar, index));
                });
                rtic::pend(lm3s6965::interrupt::SSI0);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    #[__rtic_main]
    fn __rtic_main() {
        (0..1u8)
            .for_each(|i| {
                (&mut *__rtic_sw_task_foo_request_queue.get_mut()).enqueue_unchecked(i)
            });
        (0..2u8)
            .for_each(|i| {
                (&mut *__rtic_sw_task_bar_request_queue.get_mut()).enqueue_unchecked(i)
            });
        #[__post_init]
        fn post_init() {}
    }
}
