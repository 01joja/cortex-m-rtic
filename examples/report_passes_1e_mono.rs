
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;


#[rtic::app(device = lm3s6965, dispatchers = [SSI0, GPIOA], compiler_passes = [resources])]
mod app {
    #[local]
    struct Local {
        local_r: i8,
    }
    #[shared]
    struct Shared {
        shared_r: i16,
        shared_lock_free: u8,
    }
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;
    type MyMono = Systick<100>;
    #[init()]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;
        let mono = Systick::new(systick, 12_000_000);
        (
            Shared {
                shared_r: 0,
                shared_lock_free: 0,
            },
            Local { local_r: 0 },
            init::Monotonics(mono),
        )
    }
    /// Monotonics used by the system
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_monotonic_monotonic_struct(pub Systick<100>);
    #[__rtic_task_module(has_monotonic = true)]
    pub mod init {
        pub use super::__rtic_monotonic_monotonic_struct as Monotonics;
    }
    #[allow(non_snake_case)]
    #[idle()]
    fn idle(_: idle::Context) -> ! {
        debug::exit(debug::EXIT_SUCCESS);
        loop {
            cortex_m::asm::nop();
        }
    }
    #[task(
        binds = UART0,
        priority = 1,
        local = [late_local:u16 = 0,
        ],
        shared = [shared_lock_free,
        ]
    )]
    fn baz(_: baz::Context) {}
    #[task(priority = 2, local = [local_r], shared = [shared_r])]
    fn foo(_: foo::Context) {
        ::cortex_m_semihosting::export::hstdout_str("foo\n").ok();
        debug::exit(debug::EXIT_SUCCESS);
    }
    #[task(priority = 1, shared = [shared_r, shared_lock_free])]
    fn bar(_: bar::Context) {}
    #[__rtic_task_module(has_monotonic = true)]
    pub mod foo {
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        #[allow(non_snake_case)]
        pub mod MyMono {
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_after as spawn_after;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_at as spawn_at;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_handler as SpawnHandle;
        }
    }
    
    #[__rtic_task_module(has_monotonic = true)]
    pub mod bar {
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        #[allow(non_snake_case)]
        pub mod MyMono {
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_after as spawn_after;
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_at as spawn_at;
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_handler as SpawnHandle;
        }
    }
    pub use rtic::Monotonic as _;
    /// Holds static methods for each monotonic.
    pub mod monotonics {
        pub use MyMono::now;
        ///This module holds the static implementation for `MyMono::now()`
        #[allow(non_snake_case)]
        pub mod MyMono {
            /// Read the current time from this monotonic
            pub fn now() -> <super::super::MyMono as rtic::Monotonic>::Instant {
                rtic::export::interrupt::free(|_| {
                    use rtic::Monotonic as _;
                    if let Some(m) = unsafe {
                        &mut *super::super::__rtic_monotonic_STORAGE_MyMono.get_mut()
                    } {
                        m.now()
                    } else {
                        <super::super::MyMono as rtic::Monotonic>::zero()
                    }
                })
            }
        }
    }
    #[link_section = ".uninit.rtic_MyMono_foo"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_monotonic_foo_MyMono_INSTANTS: rtic::RacyCell<
        [core::mem::MaybeUninit<<Systick<100> as rtic::Monotonic>::Instant>; 1],
    > = rtic::RacyCell::new([core::mem::MaybeUninit::uninit()]);
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_monotonic_MyMono_foo_spawn_handler {
        #[doc(hidden)]
        marker: u32,
    }
    impl core::fmt::Debug for __rtic_monotonic_MyMono_foo_spawn_handler {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("MyMono::SpawnHandle").finish()
        }
    }
    impl __rtic_monotonic_MyMono_foo_spawn_handler {
        pub fn cancel(self) -> Result<(), ()> {
            rtic::export::interrupt::free(|_| unsafe {
                let timer_queue = &mut *__rtic_monotonic_MyMono_timer_q.get_mut();
                if let Some((_task, index)) = timer_queue.cancel_marker(self.marker) {
                    let msg = (&*foo::__internal_message_list.get())
                        .get_unchecked(usize::from(index))
                        .as_ptr()
                        .read();
                    (&mut *foo::__internal_function_queue.get_mut())
                        .split()
                        .0
                        .enqueue_unchecked(index);
                    Ok(msg)
                } else {
                    Err(())
                }
            })
        }
        #[inline]
        pub fn reschedule_after(
            self,
            duration: <MyMono as rtic::Monotonic>::Duration,
        ) -> Result<Self, ()> {
            self.reschedule_at(monotonics::MyMono::now() + duration)
        }
        pub fn reschedule_at(
            self,
            instant: <MyMono as rtic::Monotonic>::Instant,
        ) -> Result<Self, ()> {
            rtic::export::interrupt::free(|_| unsafe {
                let marker = __rtic_monotonic_TIMER_QUEUE_MARKER.get().read();
                __rtic_monotonic_TIMER_QUEUE_MARKER
                    .get_mut()
                    .write(marker.wrapping_add(1));
                let timer_queue = (&mut *__rtic_monotonic_MyMono_timer_q.get_mut());
                timer_queue
                    .update_marker(
                        self.marker,
                        marker,
                        instant,
                        || rtic::export::SCB::set_pendst(),
                    )
                    .map(|_| foo::MyMono::SpawnHandle { marker })
            })
        }
    }
    /// Spawns the task after a set duration relative to the current time
    ///
    /// This will use the time `Instant::new(0)` as baseline if called in `#[init]`,
    /// so if you use a non-resetable timer use `spawn_at` when in `#[init]`
    #[allow(non_snake_case)]
    pub fn __rtic_monotonic_MyMono_foo_spawn_after(
        duration: <MyMono as rtic::Monotonic>::Duration,
    ) -> Result<foo::MyMono::SpawnHandle, ()> {
        let instant = monotonics::MyMono::now();
        __rtic_monotonic_MyMono_foo_spawn_at(instant + duration)
    }
    /// Spawns the task at a fixed time instant.
    /// Needs access to the software tasks function and input queue.
    #[allow(non_snake_case)]
    pub fn __rtic_monotonic_MyMono_foo_spawn_at(
        instant: <MyMono as rtic::Monotonic>::Instant,
    ) -> Result<foo::MyMono::SpawnHandle, ()> {
        unsafe {
            let input = ();
            if let Some(index) = rtic::export::interrupt::free(|_| {
                (&mut *foo::__internal_function_queue.get_mut()).dequeue()
            }) {
                (&mut *foo::__internal_message_list.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                (&mut *__rtic_monotonic_foo_MyMono_INSTANTS.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(instant);
                rtic::export::interrupt::free(|_| {
                    let marker = __rtic_monotonic_TIMER_QUEUE_MARKER.get().read();
                    let nr = rtic::export::NotReady {
                        instant,
                        index,
                        task: __rtic_monotonic_schedule_tasks::foo,
                        marker,
                    };
                    __rtic_monotonic_TIMER_QUEUE_MARKER
                        .get_mut()
                        .write(
                            __rtic_monotonic_TIMER_QUEUE_MARKER
                                .get()
                                .read()
                                .wrapping_add(1),
                        );
                    let timer_queue = &mut *__rtic_monotonic_MyMono_timer_q.get_mut();
                    timer_queue
                        .enqueue_unchecked(
                            nr,
                            || {
                                core::mem::transmute::<_, rtic::export::SYST>(())
                                    .enable_interrupt()
                            },
                            || rtic::export::SCB::set_pendst(),
                            (&mut *__rtic_monotonic_STORAGE_MyMono.get_mut()).as_mut(),
                        );
                    Ok(foo::MyMono::SpawnHandle { marker })
                })
            } else {
                Err(input)
            }
        }
    }
    #[link_section = ".uninit.rtic_MyMono_bar"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_monotonic_bar_MyMono_INSTANTS: rtic::RacyCell<
        [core::mem::MaybeUninit<<Systick<100> as rtic::Monotonic>::Instant>; 1],
    > = rtic::RacyCell::new([core::mem::MaybeUninit::uninit()]);
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_monotonic_MyMono_bar_spawn_handler {
        #[doc(hidden)]
        marker: u32,
    }
    impl core::fmt::Debug for __rtic_monotonic_MyMono_bar_spawn_handler {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("MyMono::SpawnHandle").finish()
        }
    }
    impl __rtic_monotonic_MyMono_bar_spawn_handler {
        pub fn cancel(self) -> Result<(), ()> {
            rtic::export::interrupt::free(|_| unsafe {
                let timer_queue = &mut *__rtic_monotonic_MyMono_timer_q.get_mut();
                if let Some((_task, index)) = timer_queue.cancel_marker(self.marker) {
                    let msg = (&*bar::__internal_message_list.get())
                        .get_unchecked(usize::from(index))
                        .as_ptr()
                        .read();
                    (&mut *bar::__internal_function_queue.get_mut())
                        .split()
                        .0
                        .enqueue_unchecked(index);
                    Ok(msg)
                } else {
                    Err(())
                }
            })
        }
        #[inline]
        pub fn reschedule_after(
            self,
            duration: <MyMono as rtic::Monotonic>::Duration,
        ) -> Result<Self, ()> {
            self.reschedule_at(monotonics::MyMono::now() + duration)
        }
        pub fn reschedule_at(
            self,
            instant: <MyMono as rtic::Monotonic>::Instant,
        ) -> Result<Self, ()> {
            rtic::export::interrupt::free(|_| unsafe {
                let marker = __rtic_monotonic_TIMER_QUEUE_MARKER.get().read();
                __rtic_monotonic_TIMER_QUEUE_MARKER
                    .get_mut()
                    .write(marker.wrapping_add(1));
                let timer_queue = (&mut *__rtic_monotonic_MyMono_timer_q.get_mut());
                timer_queue
                    .update_marker(
                        self.marker,
                        marker,
                        instant,
                        || rtic::export::SCB::set_pendst(),
                    )
                    .map(|_| bar::MyMono::SpawnHandle { marker })
            })
        }
    }
    /// Spawns the task after a set duration relative to the current time
    ///
    /// This will use the time `Instant::new(0)` as baseline if called in `#[init]`,
    /// so if you use a non-resetable timer use `spawn_at` when in `#[init]`
    #[allow(non_snake_case)]
    pub fn __rtic_monotonic_MyMono_bar_spawn_after(
        duration: <MyMono as rtic::Monotonic>::Duration,
    ) -> Result<bar::MyMono::SpawnHandle, ()> {
        let instant = monotonics::MyMono::now();
        __rtic_monotonic_MyMono_bar_spawn_at(instant + duration)
    }
    /// Spawns the task at a fixed time instant.
    /// Needs access to the software tasks function and input queue.
    #[allow(non_snake_case)]
    pub fn __rtic_monotonic_MyMono_bar_spawn_at(
        instant: <MyMono as rtic::Monotonic>::Instant,
    ) -> Result<bar::MyMono::SpawnHandle, ()> {
        unsafe {
            let input = ();
            if let Some(index) = rtic::export::interrupt::free(|_| {
                (&mut *bar::__internal_function_queue.get_mut()).dequeue()
            }) {
                (&mut *bar::__internal_message_list.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                (&mut *__rtic_monotonic_bar_MyMono_INSTANTS.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(instant);
                rtic::export::interrupt::free(|_| {
                    let marker = __rtic_monotonic_TIMER_QUEUE_MARKER.get().read();
                    let nr = rtic::export::NotReady {
                        instant,
                        index,
                        task: __rtic_monotonic_schedule_tasks::bar,
                        marker,
                    };
                    __rtic_monotonic_TIMER_QUEUE_MARKER
                        .get_mut()
                        .write(
                            __rtic_monotonic_TIMER_QUEUE_MARKER
                                .get()
                                .read()
                                .wrapping_add(1),
                        );
                    let timer_queue = &mut *__rtic_monotonic_MyMono_timer_q.get_mut();
                    timer_queue
                        .enqueue_unchecked(
                            nr,
                            || {
                                core::mem::transmute::<_, rtic::export::SYST>(())
                                    .enable_interrupt()
                            },
                            || rtic::export::SCB::set_pendst(),
                            (&mut *__rtic_monotonic_STORAGE_MyMono.get_mut()).as_mut(),
                        );
                    Ok(bar::MyMono::SpawnHandle { marker })
                })
            } else {
                Err(input)
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    static __rtic_monotonic_TIMER_QUEUE_MARKER: rtic::RacyCell<u32> = rtic::RacyCell::new(
        0,
    );
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub enum __rtic_monotonic_schedule_tasks {
        foo,
        bar,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for __rtic_monotonic_schedule_tasks {
        #[inline]
        fn clone(&self) -> __rtic_monotonic_schedule_tasks {
            *self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for __rtic_monotonic_schedule_tasks {}
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    static __rtic_monotonic_MyMono_timer_q: rtic::RacyCell<
        rtic::export::TimerQueue<Systick<100>, __rtic_monotonic_schedule_tasks, 2>,
    > = rtic::RacyCell::new(
        rtic::export::TimerQueue(rtic::export::SortedLinkedList::new_u16()),
    );
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    static __rtic_monotonic_STORAGE_MyMono: rtic::RacyCell<Option<Systick<100>>> = rtic::RacyCell::new(
        None,
    );
    #[no_mangle]
    #[allow(non_snake_case)]
    unsafe fn SysTick() {
        while let Some((task, index)) = rtic::export::interrupt::free(|_| {
            if let Some(mono) = (&mut *__rtic_monotonic_STORAGE_MyMono.get_mut())
                .as_mut()
            {
                (&mut *__rtic_monotonic_MyMono_timer_q.get_mut())
                    .dequeue(
                        || {
                            core::mem::transmute::<_, rtic::export::SYST>(())
                                .disable_interrupt()
                        },
                        mono,
                    )
            } else {
                core::hint::unreachable_unchecked()
            }
        }) {
            match task {
                __rtic_monotonic_schedule_tasks::foo => {
                    rtic::export::interrupt::free(|_| {
                        (&mut *foo::__internal_PRIO_REQUEST_Q.get_mut())
                            .split()
                            .0
                            .enqueue_unchecked((
                                foo::__internal_dispatcher_task_name::foo,
                                index,
                            ))
                    });
                    rtic::pend(
                        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0,
                    );
                }
                __rtic_monotonic_schedule_tasks::bar => {
                    rtic::export::interrupt::free(|_| {
                        (&mut *bar::__internal_PRIO_REQUEST_Q.get_mut())
                            .split()
                            .0
                            .enqueue_unchecked((
                                bar::__internal_dispatcher_task_name::bar,
                                index,
                            ))
                    });
                    rtic::pend(
                        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOA,
                    );
                }
            }
        }
        rtic::export::interrupt::free(|_| {
            if let Some(mono) = (&mut *__rtic_monotonic_STORAGE_MyMono.get_mut())
                .as_mut()
            {
                mono.on_interrupt();
            }
        });
    }
    #[__rtic_main]
    fn __rtic_main() {
        const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS)
            < (1 << lm3s6965::NVIC_PRIO_BITS) as usize
        {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by monotonic \'MyMono\' is more than supported by hardware",
                    ),
                );
            };
        };
        core.SCB
            .set_priority(
                rtic::export::SystemHandler::SysTick,
                rtic::export::logical2hw(
                    (1 << lm3s6965::NVIC_PRIO_BITS),
                    lm3s6965::NVIC_PRIO_BITS,
                ),
            );
        if !<Systick<100> as rtic::Monotonic>::DISABLE_INTERRUPT_ON_EMPTY_QUEUE {
            core::mem::transmute::<_, rtic::export::SYST>(()).enable_interrupt();
        }
        #[__post_init]
        fn post_init() {
            monotonics.0.reset();
            __rtic_monotonic_STORAGE_MyMono.get_mut().write(Some(monotonics.0));
        }
    }
}
