


#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, compiler_passes = ["hardware"])]
mod app {
    /// #user_imports
    use cortex_m_semihosting::{debug, hprintln};
    /// #dispatchers
    /// The real dispatcher
    #[task(binds = SSI0, priority = 1)]
    fn __rtic_internal_SSI0_(_: __rtic_internal_SSI0_::Context) {
        __rtic_internal_SSI0_unsafe();
    }
    fn __rtic_internal_SSI0_unsafe() {
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
                                let priority = &rtic::export::Priority::new(PRIORITY);
                                foo()
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
        rtic::export::SCRQ<__rtic_dispatcher_for_priority_1, 2>,
    > = rtic::RacyCell::new(rtic::export::Queue::new());
    /// #software_tasks
    /// Software task as a function
    #[allow(non_snake_case)]
    fn foo() {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("foo\n").unwrap();
        debug::exit(debug::EXIT_SUCCESS);
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
                rtic::pend(lm3s6965::interrupt::SSI0);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    /// #user_init
    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        init_software();
        foo::spawn().unwrap();
        ::cortex_m_semihosting::export::hstdout_str("init\n").unwrap();
        (Shared {}, Local {}, init::Monotonics())
    }
    /// #init_software
    fn init_software() {
        unsafe {
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_sw_task_foo_function_queue.get_mut())
                        .enqueue_unchecked(i)
                });
        }
    }
    /// #user_idle
    /// #user_code
    /// #hardware_tasks
    /// #resources
    #[local]
    struct Local {}
    #[shared]
    struct Shared {}
}
