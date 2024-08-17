#![feature(prelude_import)]
//! examples/message_passing.rs
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
                        &mut *super::super::__rtic_internal_MONOTONIC_STORAGE_MyMono
                            .get_mut()
                    } {
                        m.now()
                    } else {
                        <super::super::MyMono as rtic::Monotonic>::zero()
                    }
                })
            }
        }
    }
    /// #user_imports
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;
    /// #user_code
    type MyMono = Systick<100>;
    /// #user
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;
        let mono = Systick::new(systick, 12_000_000);
        foo::spawn_after(1.secs()).unwrap();
        ::cortex_m_semihosting::export::hstdout_str("init\n").ok();
        (
            Shared {
                shared_r: 0,
                shared_lock_free: 0,
            },
            Local { local_r: 0 },
            init::Monotonics(mono),
        )
    }
    /// #user_hardware_tasks
    #[allow(non_snake_case)]
    fn baz(_: baz::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
    }
    /// #user_software_tasks
    #[allow(non_snake_case)]
    fn foo(_: foo::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("foo\n").ok();
        debug::exit(debug::EXIT_SUCCESS);
    }
    #[allow(non_snake_case)]
    fn bar(_: bar::Context, _x: i32, _y: i32) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
    }
    /// #root
    struct Shared {
        shared_r: i16,
        shared_lock_free: u8,
    }
    struct Local {
        local_r: i8,
    }
    /// Monotonics used by the system
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_Monotonics(pub Systick<100>);
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
    }
    /// #mod_local_resources
    /// #root_hardware_tasks
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `baz` has access to
    pub struct __rtic_internal_bazLocalResources<'a> {
        pub late_local: &'a mut u16,
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `baz` has access to
    pub struct __rtic_internal_bazSharedResources<'a> {
        pub shared_lock_free: &'a mut u8,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_baz_Context<'a> {
        /// Local Resources this task has access to
        pub local: baz::LocalResources<'a>,
        /// Shared Resources this task has access to
        pub shared: baz::SharedResources<'a>,
    }
    impl<'a> __rtic_internal_baz_Context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_baz_Context {
                local: baz::LocalResources::new(),
                shared: baz::SharedResources::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod baz {
        #[doc(inline)]
        pub use super::__rtic_internal_bazLocalResources as LocalResources;
        #[doc(inline)]
        pub use super::__rtic_internal_bazSharedResources as SharedResources;
        pub use super::__rtic_internal_baz_Context as Context;
    }
    /// #user_software_tasks
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `foo` has access to
    pub struct __rtic_internal_fooLocalResources<'a> {
        pub local_r: &'a mut i8,
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `foo` has access to
    pub struct __rtic_internal_fooSharedResources<'a> {
        pub shared_r: shared_resources::shared_r_that_needs_to_be_locked<'a>,
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
                    (&mut *__rtic_internal_P2_RQ.get_mut())
                        .enqueue_unchecked((P2_T::foo, index));
                });
                rtic::pend(lm3s6965::interrupt::SSI0);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_foo_MyMono_SpawnHandle {
        #[doc(hidden)]
        marker: u32,
    }
    impl core::fmt::Debug for __rtic_internal_foo_MyMono_SpawnHandle {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("MyMono::SpawnHandle").finish()
        }
    }
    impl __rtic_internal_foo_MyMono_SpawnHandle {
        pub fn cancel(self) -> Result<(), ()> {
            rtic::export::interrupt::free(|_| unsafe {
                let tq = &mut *__rtic_internal_TQ_MyMono.get_mut();
                if let Some((_task, index)) = tq.cancel_marker(self.marker) {
                    let msg = (&*__rtic_internal_foo_INPUTS.get())
                        .get_unchecked(usize::from(index))
                        .as_ptr()
                        .read();
                    (&mut *__rtic_internal_foo_FQ.get_mut())
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
                let marker = __rtic_internal_TIMER_QUEUE_MARKER.get().read();
                __rtic_internal_TIMER_QUEUE_MARKER
                    .get_mut()
                    .write(marker.wrapping_add(1));
                let tq = (&mut *__rtic_internal_TQ_MyMono.get_mut());
                tq.update_marker(
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
    pub fn __rtic_internal_foo_MyMono_spawn_after(
        duration: <MyMono as rtic::Monotonic>::Duration,
    ) -> Result<foo::MyMono::SpawnHandle, ()> {
        let instant = monotonics::MyMono::now();
        __rtic_internal_foo_MyMono_spawn_at(instant + duration)
    }
    /// Spawns the task at a fixed time instant
    #[allow(non_snake_case)]
    pub fn __rtic_internal_foo_MyMono_spawn_at(
        instant: <MyMono as rtic::Monotonic>::Instant,
    ) -> Result<foo::MyMono::SpawnHandle, ()> {
        unsafe {
            let input = ();
            if let Some(index) = rtic::export::interrupt::free(|_| {
                (&mut *__rtic_internal_foo_FQ.get_mut()).dequeue()
            }) {
                (&mut *__rtic_internal_foo_INPUTS.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                (&mut *__rtic_internal_foo_MyMono_INSTANTS.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(instant);
                rtic::export::interrupt::free(|_| {
                    let marker = __rtic_internal_TIMER_QUEUE_MARKER.get().read();
                    let nr = rtic::export::NotReady {
                        instant,
                        index,
                        task: SCHED_T::foo,
                        marker,
                    };
                    __rtic_internal_TIMER_QUEUE_MARKER
                        .get_mut()
                        .write(
                            __rtic_internal_TIMER_QUEUE_MARKER
                                .get()
                                .read()
                                .wrapping_add(1),
                        );
                    let tq = &mut *__rtic_internal_TQ_MyMono.get_mut();
                    tq.enqueue_unchecked(
                        nr,
                        || {
                            core::mem::transmute::<_, rtic::export::SYST>(())
                                .enable_interrupt()
                        },
                        || rtic::export::SCB::set_pendst(),
                        (&mut *__rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut())
                            .as_mut(),
                    );
                    Ok(foo::MyMono::SpawnHandle { marker })
                })
            } else {
                Err(input)
            }
        }
    }
    #[allow(non_snake_case)]
    ///Software task
    pub mod foo {
        #[doc(inline)]
        pub use super::__rtic_internal_fooLocalResources as LocalResources;
        #[doc(inline)]
        pub use super::__rtic_internal_fooSharedResources as SharedResources;
        pub use super::__rtic_internal_foo_Context as Context;
        pub use super::__rtic_internal_foo_spawn as spawn;
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        pub mod MyMono {
            pub use super::super::__rtic_internal_foo_MyMono_spawn_after as spawn_after;
            pub use super::super::__rtic_internal_foo_MyMono_spawn_at as spawn_at;
            pub use super::super::__rtic_internal_foo_MyMono_SpawnHandle as SpawnHandle;
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `bar` has access to
    pub struct __rtic_internal_barSharedResources<'a> {
        pub shared_r: shared_resources::shared_r_that_needs_to_be_locked<'a>,
        pub shared_lock_free: &'a mut u8,
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
    pub fn __rtic_internal_bar_spawn(_0: i32, _1: i32) -> Result<(), (i32, i32)> {
        let input = (_0, _1);
        unsafe {
            if let Some(index) = rtic::export::interrupt::free(|_| {
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
                rtic::pend(lm3s6965::interrupt::GPIOA);
                Ok(())
            } else {
                Err(input)
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_bar_MyMono_SpawnHandle {
        #[doc(hidden)]
        marker: u32,
    }
    impl core::fmt::Debug for __rtic_internal_bar_MyMono_SpawnHandle {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("MyMono::SpawnHandle").finish()
        }
    }
    impl __rtic_internal_bar_MyMono_SpawnHandle {
        pub fn cancel(self) -> Result<(i32, i32), ()> {
            rtic::export::interrupt::free(|_| unsafe {
                let tq = &mut *__rtic_internal_TQ_MyMono.get_mut();
                if let Some((_task, index)) = tq.cancel_marker(self.marker) {
                    let msg = (&*__rtic_internal_bar_INPUTS.get())
                        .get_unchecked(usize::from(index))
                        .as_ptr()
                        .read();
                    (&mut *__rtic_internal_bar_FQ.get_mut())
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
                let marker = __rtic_internal_TIMER_QUEUE_MARKER.get().read();
                __rtic_internal_TIMER_QUEUE_MARKER
                    .get_mut()
                    .write(marker.wrapping_add(1));
                let tq = (&mut *__rtic_internal_TQ_MyMono.get_mut());
                tq.update_marker(
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
    pub fn __rtic_internal_bar_MyMono_spawn_after(
        duration: <MyMono as rtic::Monotonic>::Duration,
        _0: i32,
        _1: i32,
    ) -> Result<bar::MyMono::SpawnHandle, (i32, i32)> {
        let instant = monotonics::MyMono::now();
        __rtic_internal_bar_MyMono_spawn_at(instant + duration, _0, _1)
    }
    /// Spawns the task at a fixed time instant
    #[allow(non_snake_case)]
    pub fn __rtic_internal_bar_MyMono_spawn_at(
        instant: <MyMono as rtic::Monotonic>::Instant,
        _0: i32,
        _1: i32,
    ) -> Result<bar::MyMono::SpawnHandle, (i32, i32)> {
        unsafe {
            let input = (_0, _1);
            if let Some(index) = rtic::export::interrupt::free(|_| {
                (&mut *__rtic_internal_bar_FQ.get_mut()).dequeue()
            }) {
                (&mut *__rtic_internal_bar_INPUTS.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(input);
                (&mut *__rtic_internal_bar_MyMono_INSTANTS.get_mut())
                    .get_unchecked_mut(usize::from(index))
                    .as_mut_ptr()
                    .write(instant);
                rtic::export::interrupt::free(|_| {
                    let marker = __rtic_internal_TIMER_QUEUE_MARKER.get().read();
                    let nr = rtic::export::NotReady {
                        instant,
                        index,
                        task: SCHED_T::bar,
                        marker,
                    };
                    __rtic_internal_TIMER_QUEUE_MARKER
                        .get_mut()
                        .write(
                            __rtic_internal_TIMER_QUEUE_MARKER
                                .get()
                                .read()
                                .wrapping_add(1),
                        );
                    let tq = &mut *__rtic_internal_TQ_MyMono.get_mut();
                    tq.enqueue_unchecked(
                        nr,
                        || {
                            core::mem::transmute::<_, rtic::export::SYST>(())
                                .enable_interrupt()
                        },
                        || rtic::export::SCB::set_pendst(),
                        (&mut *__rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut())
                            .as_mut(),
                    );
                    Ok(bar::MyMono::SpawnHandle { marker })
                })
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
        pub use MyMono::spawn_after;
        pub use MyMono::spawn_at;
        pub use MyMono::SpawnHandle;
        pub mod MyMono {
            pub use super::super::__rtic_internal_bar_MyMono_spawn_after as spawn_after;
            pub use super::super::__rtic_internal_bar_MyMono_spawn_at as spawn_at;
            pub use super::super::__rtic_internal_bar_MyMono_SpawnHandle as SpawnHandle;
        }
    }
    /// app module
    /// #mod_app_shared_resources
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic0"]
    static __rtic_internal_shared_resource_shared_r: rtic::RacyCell<
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
                    __rtic_internal_shared_resource_shared_r.get_mut() as *mut _,
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
    static __rtic_internal_shared_resource_shared_lock_free: rtic::RacyCell<
        core::mem::MaybeUninit<u8>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_internal_MASK_CHUNKS: usize = rtic::export::compute_mask_chunks([
        lm3s6965::Interrupt::GPIOA as u32,
        lm3s6965::Interrupt::SSI0 as u32,
        lm3s6965::Interrupt::UART0 as u32,
    ]);
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_internal_MASKS: [rtic::export::Mask<__rtic_internal_MASK_CHUNKS>; 3] = [
        rtic::export::create_mask([
            lm3s6965::Interrupt::GPIOA as u32,
            lm3s6965::Interrupt::UART0 as u32,
        ]),
        rtic::export::create_mask([lm3s6965::Interrupt::SSI0 as u32]),
        rtic::export::create_mask([]),
    ];
    /// #mod_app_local_resources
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic2"]
    static __rtic_internal_local_resource_local_r: rtic::RacyCell<
        core::mem::MaybeUninit<i8>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_local_baz_late_local: rtic::RacyCell<u16> = rtic::RacyCell::new(
        0,
    );
    /// #mod_app_hardware_tasks
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn UART0() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(
            PRIORITY,
            || { baz(baz::Context::new(&rtic::export::Priority::new(PRIORITY))) },
        );
    }
    impl<'a> __rtic_internal_bazLocalResources<'a> {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_internal_bazLocalResources {
                late_local: &mut *__rtic_internal_local_baz_late_local.get_mut(),
            }
        }
    }
    impl<'a> __rtic_internal_bazSharedResources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_bazSharedResources {
                shared_lock_free: &mut *(&mut *__rtic_internal_shared_resource_shared_lock_free
                    .get_mut())
                    .as_mut_ptr(),
            }
        }
    }
    /// #user_software_tasks
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_foo_FQ: rtic::RacyCell<rtic::export::SCFQ<2>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[link_section = ".uninit.rtic3"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_foo_MyMono_INSTANTS: rtic::RacyCell<
        [core::mem::MaybeUninit<<Systick<100> as rtic::Monotonic>::Instant>; 1],
    > = rtic::RacyCell::new([core::mem::MaybeUninit::uninit()]);
    #[link_section = ".uninit.rtic4"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_foo_INPUTS: rtic::RacyCell<[core::mem::MaybeUninit<()>; 1]> = rtic::RacyCell::new([
        core::mem::MaybeUninit::uninit(),
    ]);
    impl<'a> __rtic_internal_fooLocalResources<'a> {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_internal_fooLocalResources {
                local_r: &mut *(&mut *__rtic_internal_local_resource_local_r.get_mut())
                    .as_mut_ptr(),
            }
        }
    }
    impl<'a> __rtic_internal_fooSharedResources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_fooSharedResources {
                shared_r: shared_resources::shared_r_that_needs_to_be_locked::new(
                    priority,
                ),
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_bar_FQ: rtic::RacyCell<rtic::export::SCFQ<3>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[link_section = ".uninit.rtic5"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_bar_MyMono_INSTANTS: rtic::RacyCell<
        [core::mem::MaybeUninit<<Systick<100> as rtic::Monotonic>::Instant>; 2],
    > = rtic::RacyCell::new([
        core::mem::MaybeUninit::uninit(),
        core::mem::MaybeUninit::uninit(),
    ]);
    #[link_section = ".uninit.rtic6"]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_bar_INPUTS: rtic::RacyCell<
        [core::mem::MaybeUninit<(i32, i32)>; 2],
    > = rtic::RacyCell::new([
        core::mem::MaybeUninit::uninit(),
        core::mem::MaybeUninit::uninit(),
    ]);
    impl<'a> __rtic_internal_barSharedResources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_barSharedResources {
                shared_r: shared_resources::shared_r_that_needs_to_be_locked::new(
                    priority,
                ),
                shared_lock_free: &mut *(&mut *__rtic_internal_shared_resource_shared_lock_free
                    .get_mut())
                    .as_mut_ptr(),
            }
        }
    }
    /// #mod_app_dispatchers
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub enum P1_T {
        bar,
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
    static __rtic_internal_P1_RQ: rtic::RacyCell<rtic::export::SCRQ<P1_T, 3>> = rtic::RacyCell::new(
        rtic::export::Queue::new(),
    );
    #[allow(non_snake_case)]
    ///Interrupt handler to dispatch tasks at priority 1
    #[no_mangle]
    unsafe fn GPIOA() {
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
                        P1_T::bar => {
                            let (_0, _1) = (&*__rtic_internal_bar_INPUTS.get())
                                .get_unchecked(usize::from(index))
                                .as_ptr()
                                .read();
                            (&mut *__rtic_internal_bar_FQ.get_mut())
                                .split()
                                .0
                                .enqueue_unchecked(index);
                            let priority = &rtic::export::Priority::new(PRIORITY);
                            bar(bar::Context::new(priority), _0, _1)
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
        foo,
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
    unsafe fn SSI0() {
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
                        P2_T::foo => {
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
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    static __rtic_internal_TIMER_QUEUE_MARKER: rtic::RacyCell<u32> = rtic::RacyCell::new(
        0,
    );
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub enum SCHED_T {
        foo,
        bar,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for SCHED_T {
        #[inline]
        fn clone(&self) -> SCHED_T {
            *self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for SCHED_T {}
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    static __rtic_internal_TQ_MyMono: rtic::RacyCell<
        rtic::export::TimerQueue<Systick<100>, SCHED_T, 3>,
    > = rtic::RacyCell::new(
        rtic::export::TimerQueue(rtic::export::SortedLinkedList::new_u16()),
    );
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    static __rtic_internal_MONOTONIC_STORAGE_MyMono: rtic::RacyCell<
        Option<Systick<100>>,
    > = rtic::RacyCell::new(None);
    #[no_mangle]
    #[allow(non_snake_case)]
    unsafe fn SysTick() {
        while let Some((task, index)) = rtic::export::interrupt::free(|_| {
            if let Some(mono) = (&mut *__rtic_internal_MONOTONIC_STORAGE_MyMono
                .get_mut())
                .as_mut()
            {
                (&mut *__rtic_internal_TQ_MyMono.get_mut())
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
                SCHED_T::foo => {
                    rtic::export::interrupt::free(|_| {
                        (&mut *__rtic_internal_P2_RQ.get_mut())
                            .split()
                            .0
                            .enqueue_unchecked((P2_T::foo, index))
                    });
                    rtic::pend(
                        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0,
                    );
                }
                SCHED_T::bar => {
                    rtic::export::interrupt::free(|_| {
                        (&mut *__rtic_internal_P1_RQ.get_mut())
                            .split()
                            .0
                            .enqueue_unchecked((P1_T::bar, index))
                    });
                    rtic::pend(
                        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOA,
                    );
                }
            }
        }
        rtic::export::interrupt::free(|_| {
            if let Some(mono) = (&mut *__rtic_internal_MONOTONIC_STORAGE_MyMono
                .get_mut())
                .as_mut()
            {
                mono.on_interrupt();
            }
        });
    }
    #[doc(hidden)]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            rtic::export::assert_send::<i16>();
            rtic::export::assert_send::<u8>();
            rtic::export::assert_send::<i8>();
            rtic::export::assert_send::<i32>();
            rtic::export::assert_monotonic::<Systick<100>>();
            const _CONST_CHECK: () = {
                if !rtic::export::have_basepri() {
                    if (lm3s6965::Interrupt::UART0 as usize)
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
            (0..1u8)
                .for_each(|i| {
                    (&mut *__rtic_internal_foo_FQ.get_mut()).enqueue_unchecked(i)
                });
            (0..2u8)
                .for_each(|i| {
                    (&mut *__rtic_internal_bar_FQ.get_mut()).enqueue_unchecked(i)
                });
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
                .into();
            let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0;
            let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOA;
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
            const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 2u8 as usize {
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
                    rtic::export::logical2hw(2u8, lm3s6965::NVIC_PRIO_BITS),
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
                __rtic_internal_shared_resource_shared_r
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(shared_resources.shared_r));
                __rtic_internal_shared_resource_shared_lock_free
                    .get_mut()
                    .write(
                        core::mem::MaybeUninit::new(shared_resources.shared_lock_free),
                    );
                __rtic_internal_local_resource_local_r
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(local_resources.local_r));
                monotonics.0.reset();
                __rtic_internal_MONOTONIC_STORAGE_MyMono
                    .get_mut()
                    .write(Some(monotonics.0));
                rtic::export::interrupt::enable();
            });
            loop {
                rtic::export::nop()
            }
        }
    }
}