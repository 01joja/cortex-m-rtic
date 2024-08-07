// Context:
pub mod foo {
    #[doc(inline)]
    pub use super::__rtic_internal_fooLocalResources as LocalResources;
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

pub fn __rtic_internal_foo_MyMono_spawn_after(
    duration: <MyMono as rtic::Monotonic>::Duration,
) -> Result<foo::MyMono::SpawnHandle, ()> {
    let instant = monotonics::MyMono::now();
    __rtic_internal_foo_MyMono_spawn_at(instant + duration)
}

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

static __rtic_internal_TQ_MyMono: rtic::RacyCell<
rtic::export::TimerQueue<Systick<100>, SCHED_T, 2>,
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
                (&mut *__rtic_internal_P1_RQ.get_mut())
                    .split()
                    .0
                    .enqueue_unchecked((P1_T::foo, index))
            });
            rtic::pend(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0,
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

// pre init

rtic::export::assert_monotonic::<Systick<100>>();
if !<Systick<100> as rtic::Monotonic>::DISABLE_INTERRUPT_ON_EMPTY_QUEUE {
    core::mem::transmute::<_, rtic::export::SYST>(()).enable_interrupt();
}


// post init

monotonics.0.reset();
__rtic_internal_MONOTONIC_STORAGE_MyMono
    .get_mut()
    .write(Some(monotonics.0));
