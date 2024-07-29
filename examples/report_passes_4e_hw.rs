#![feature(prelude_import)]
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
    /// #user_imports
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;
    pub use rtic::Monotonic as _;
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
    pub struct __rtic_context_idle_context {}
    impl __rtic_context_idle_context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
            __rtic_context_idle_context {}
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_context_baz_context<'a> {
        pub local: baz::LocalResources<'a>,
        pub shared: baz::SharedResources<'a>,
    }
    impl<'a> __rtic_context_baz_context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_context_baz_context {
                local: baz::LocalResources::new(),
                shared: baz::SharedResources::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_context_foo_context<'a> {
        pub local: foo::LocalResources<'a>,
        pub shared: foo::SharedResources<'a>,
    }
    impl<'a> __rtic_context_foo_context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_context_foo_context {
                local: foo::LocalResources::new(),
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
    ///Local resources `baz` has access to
    pub struct __rtic_local_resource_baz_local_resources<'a> {
        pub late_local: &'a mut u16,
    }
    impl<'a> __rtic_local_resource_baz_local_resources<'a> {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_local_resource_baz_local_resources {
                late_local: &mut *__rtic_local_resource_baz_late_local.get_mut(),
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `baz` has access to
    pub struct __rtic_shared_resource_baz_shared_resources<'a> {
        pub shared_lock_free: shared_resources::shared_lock_free_that_needs_to_be_locked<
            'a,
        >,
    }
    impl<'a> __rtic_shared_resource_baz_shared_resources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_shared_resource_baz_shared_resources {
                shared_lock_free: shared_resources::shared_lock_free_that_needs_to_be_locked::new(
                    priority,
                ),
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `foo` has access to
    pub struct __rtic_local_resource_foo_local_resources<'a> {
        pub local_r: &'a mut i8,
    }
    impl<'a> __rtic_local_resource_foo_local_resources<'a> {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_local_resource_foo_local_resources {
                local_r: &mut *(&mut *__rtic_local_resource_local_r.get_mut())
                    .as_mut_ptr(),
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `foo` has access to
    pub struct __rtic_shared_resource_foo_shared_resources<'a> {
        pub shared_r: shared_resources::shared_r_that_needs_to_be_locked<'a>,
    }
    impl<'a> __rtic_shared_resource_foo_shared_resources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_shared_resource_foo_shared_resources {
                shared_r: shared_resources::shared_r_that_needs_to_be_locked::new(
                    priority,
                ),
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `bar` has access to
    pub struct __rtic_shared_resource_bar_shared_resources<'a> {
        pub shared_r: shared_resources::shared_r_that_needs_to_be_locked<'a>,
        pub shared_lock_free: shared_resources::shared_lock_free_that_needs_to_be_locked<
            'a,
        >,
    }
    impl<'a> __rtic_shared_resource_bar_shared_resources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_shared_resource_bar_shared_resources {
                shared_r: shared_resources::shared_r_that_needs_to_be_locked::new(
                    priority,
                ),
                shared_lock_free: shared_resources::shared_lock_free_that_needs_to_be_locked::new(
                    priority,
                ),
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic_r_local_r"]
    static __rtic_local_resource_local_r: rtic::RacyCell<core::mem::MaybeUninit<i8>> = rtic::RacyCell::new(
        core::mem::MaybeUninit::uninit(),
    );
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_local_resource_baz_late_local: rtic::RacyCell<u16> = rtic::RacyCell::new(
        0,
    );
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic_r_shared_r"]
    static __rtic_shared_resource_shared_r: rtic::RacyCell<
        core::mem::MaybeUninit<i16>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    impl<'a> rtic::Mutex for shared_resources::shared_r_that_needs_to_be_locked<'a> {
        type T = i16;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut i16) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            /// Priority ceiling
            const CEILING: u8 = 2u8;
            unsafe {
                rtic::export::lock(
                    __rtic_shared_resource_shared_r.get_mut() as *mut _,
                    self.priority(),
                    CEILING,
                    lm3s6965::NVIC_PRIO_BITS,
                    &__rtic_shared_resources_MASKS,
                    f,
                )
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic_r_shared_lock_free"]
    static __rtic_shared_resource_shared_lock_free: rtic::RacyCell<
        core::mem::MaybeUninit<u8>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    impl<'a> rtic::Mutex
    for shared_resources::shared_lock_free_that_needs_to_be_locked<'a> {
        type T = u8;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut u8) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            /// Priority ceiling
            const CEILING: u8 = 1u8;
            unsafe {
                rtic::export::lock(
                    __rtic_shared_resource_shared_lock_free.get_mut() as *mut _,
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
        lm3s6965::Interrupt::GPIOA as u32,
        lm3s6965::Interrupt::SSI0 as u32,
        lm3s6965::Interrupt::UART0 as u32,
    ]);
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_shared_resources_MASKS: [rtic::export::Mask<
        __rtic_shared_resources_MASK_CHUNKS,
    >; 3] = [
        rtic::export::create_mask([
            lm3s6965::Interrupt::GPIOA as u32,
            lm3s6965::Interrupt::UART0 as u32,
        ]),
        rtic::export::create_mask([lm3s6965::Interrupt::SSI0 as u32]),
        rtic::export::create_mask([]),
    ];
    mod shared_resources {
        use rtic::export::Priority;
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct shared_r_that_needs_to_be_locked<'a> {
            priority: &'a Priority,
        }
        impl<'a> shared_r_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a Priority) -> Self {
                shared_r_that_needs_to_be_locked {
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
        pub struct shared_lock_free_that_needs_to_be_locked<'a> {
            priority: &'a Priority,
        }
        impl<'a> shared_lock_free_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a Priority) -> Self {
                shared_lock_free_that_needs_to_be_locked {
                    priority,
                }
            }
            #[inline(always)]
            pub unsafe fn priority(&self) -> &Priority {
                self.priority
            }
        }
    }
    type MyMono = Systick<100>;
    /// Monotonics used by the system
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_monotonic_monotonic_struct(pub Systick<100>);
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
    /// Context needed to pass local and shared resources to their respective task.
    /// All software tasks belonging to prio X
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum __rtic_software_tasks_prio_2 {
        foo,
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
    /// Context needed to pass local and shared resources to their respective task.
    /// All software tasks belonging to prio X
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum __rtic_software_tasks_prio_1 {
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
        rtic::export::SCRQ<__rtic_software_tasks_prio_1, 2>,
    > = rtic::RacyCell::new(rtic::export::Queue::new());
    /// Software task as a function
    #[allow(non_snake_case)]
    fn foo(_: foo::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("foo\n").ok();
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
        pub use super::__rtic_local_resource_foo_local_resources as LocalResources;
        pub use super::__rtic_shared_resource_foo_shared_resources as SharedResources;
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        #[allow(non_snake_case)]
        pub mod MyMono {
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_after as spawn_after;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_at as spawn_at;
            pub use super::super::__rtic_monotonic_MyMono_foo_spawn_handler as SpawnHandle;
        }
        pub use super::__rtic_software_tasks_prio_2 as __internal_dispatcher_task_name;
        pub use super::__rtic_dispatcher_request_queue_prio_2 as __internal_PRIO_REQUEST_Q;
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
                    (&mut *__rtic_dispatcher_request_queue_prio_2.get_mut())
                        .enqueue_unchecked((__rtic_software_tasks_prio_2::foo, index));
                });
                rtic::pend(lm3s6965::interrupt::GPIOA);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    /// Software task as a function
    #[allow(non_snake_case)]
    fn bar(_: bar::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
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
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        #[allow(non_snake_case)]
        pub mod MyMono {
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_after as spawn_after;
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_at as spawn_at;
            pub use super::super::__rtic_monotonic_MyMono_bar_spawn_handler as SpawnHandle;
        }
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
    /// #user_init
    #[inline(always)]
    #[allow(non_snake_case)]
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
    /// #module_init
    struct Shared {
        shared_r: i16,
        shared_lock_free: u8,
    }
    struct Local {
        local_r: i8,
    }
    mod init {
        pub use super::__rtic_context_init_context as Context;
        pub use super::__rtic_monotonic_monotonic_struct as Monotonics;
    }
    /// #user_idle
    ///Idle function
    #[allow(non_snake_case)]
    #[allow(non_snake_case)]
    #[allow(non_snake_case)]
    #[allow(non_snake_case)]
    fn idle(_: idle::Context) -> ! {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        debug::exit(debug::EXIT_SUCCESS);
        loop {
            cortex_m::asm::nop();
        }
    }
    /// #module_idle
    ///module_idle
    #[allow(non_snake_case)]
    ///idle loop
    pub mod idle {
        pub use super::__rtic_context_idle_context as Context;
    }
    /// #user_hardware_tasks
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
                            __rtic_software_tasks_prio_2::foo => {
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
    fn __rtic_dispatcher_SSI0() {
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
                            __rtic_software_tasks_prio_1::bar => {
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
    #[allow(non_snake_case)]
    fn baz(_: baz::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
    }
    /// #modules_hardware_tasks
    pub mod baz {
        pub use super::__rtic_context_baz_context as Context;
        pub use super::__rtic_local_resource_baz_local_resources as LocalResources;
        pub use super::__rtic_shared_resource_baz_shared_resources as SharedResources;
    }
    /// #interrupts_handlers
    #[allow(non_snake_case)]
    #[no_mangle]
    #[allow(unsafe_code)]
    unsafe fn GPIOA() {
        const PRIORITY: u8 = 2u8;
        rtic::export::run(PRIORITY, || { __rtic_dispatcher_GPIOA() });
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    #[allow(unsafe_code)]
    unsafe fn SSI0() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(PRIORITY, || { __rtic_dispatcher_SSI0() });
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn UART0() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(
            PRIORITY,
            || { baz(baz::Context::new(&rtic::export::Priority::new(PRIORITY))) },
        );
    }
    /// #main
    #[doc(hidden)]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            rtic::export::interrupt::disable();
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
                .into();
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
            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 1u8 as usize {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Maximum priority used by interrupt vector \'SSI0\' is more than supported by hardware",
                        ),
                    );
                };
            };
            core.NVIC
                .set_priority(
                    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0,
                    rtic::export::logical2hw(1u8, lm3s6965::NVIC_PRIO_BITS),
                );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0,
            );
            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 1u8 as usize {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Maximum priority used by interrupt vector \'UART0\' is more than supported by hardware",
                        ),
                    );
                };
            };
            core.NVIC
                .set_priority(
                    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART0,
                    rtic::export::logical2hw(1u8, lm3s6965::NVIC_PRIO_BITS),
                );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART0,
            );
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_sw_task_foo_request_queue.get_mut())
                        .enqueue_unchecked(i)
                });
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_sw_task_bar_request_queue.get_mut())
                        .enqueue_unchecked(i)
                });
            rtic::export::assert_send::<i16>();
            rtic::export::assert_send::<u8>();
            rtic::export::assert_send::<i8>();
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
                } else {}
            };
            let _ = _CONST_CHECK;
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
                __rtic_shared_resource_shared_lock_free
                    .get_mut()
                    .write(
                        core::mem::MaybeUninit::new(shared_resources.shared_lock_free),
                    );
                __rtic_local_resource_local_r
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(local_resources.local_r));
                __rtic_shared_resource_shared_r
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(shared_resources.shared_r));
                monotonics.0.reset();
                __rtic_monotonic_STORAGE_MyMono.get_mut().write(Some(monotonics.0));
                rtic::export::interrupt::enable();
            });
            idle(
                ///Idle context
                idle::Context::new(&rtic::export::Priority::new(0)),
            )
        }
    }
}
