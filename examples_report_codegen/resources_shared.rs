

// Shared resources

///Software task
pub mod foo {
    #[doc(inline)]
    pub use super::__rtic_internal_fooSharedResources as SharedResources;
    pub use super::__rtic_internal_foo_Context as Context;
    pub use super::__rtic_internal_foo_spawn as spawn;
}

///Shared resources `foo` has access to
pub struct __rtic_internal_fooSharedResources<'a> {
    pub a_shared: shared_resources::a_shared_that_needs_to_be_locked<'a>,
}
/// Execution context
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct __rtic_internal_foo_Context<'a> {
    /// Shared Resources this task has access to
    pub shared: foo::SharedResources<'a>,
}
impl<'a> __rtic_internal_foo_Context<'a> {
    #[inline(always)]
    pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
        __rtic_internal_foo_Context {
            shared: foo::SharedResources::new(priority),
        }
    }
}

// Generated for each shared resource

// Implementations for shared resources
mod shared_resources {
    use rtic::export::Priority;
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct a_shared_that_needs_to_be_locked<'a> {
        priority: &'a Priority,
    }
    impl<'a> a_shared_that_needs_to_be_locked<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a Priority) -> Self {
            a_shared_that_needs_to_be_locked {
                priority,
            }
        }
        #[inline(always)]
        pub unsafe fn priority(&self) -> &Priority {
            self.priority
        }
    }
}

// Storage for a_shared, 
static __rtic_internal_shared_resource_a_shared: rtic::RacyCell<
core::mem::MaybeUninit<u32>,
> = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());

// Lock for a_shared
impl<'a> rtic::Mutex for shared_resources::a_shared_that_needs_to_be_locked<'a> {
    type T = u32;
    #[inline(always)]
    fn lock<RTIC_INTERNAL_R>(
        &mut self,
        f: impl FnOnce(&mut u32) -> RTIC_INTERNAL_R,
    ) -> RTIC_INTERNAL_R {
        const CEILING: u8 = 2u8; // Priority ceiling, 
        unsafe {
            rtic::export::lock(
                __rtic_internal_shared_resource_a_shared.get_mut() as *mut _,
                self.priority(),
                CEILING,
                lm3s6965::NVIC_PRIO_BITS,
                &__rtic_internal_MASKS,
                f,
            )
        }
    }
}



// post init
__rtic_internal_shared_resource_a_shared
    .get_mut()
    .write(core::mem::MaybeUninit::new(shared_resources.a_shared));

// Lock free

pub struct __rtic_internal_barSharedResources<'a> {
    pub counter: &'a mut u64,
}

static __rtic_internal_shared_resource_counter: rtic::RacyCell<
core::mem::MaybeUninit<u64>,
> = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());

impl<'a> __rtic_internal_fooSharedResources<'a> {
    #[inline(always)]
    pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
        __rtic_internal_fooSharedResources {
            counter: &mut *(&mut *__rtic_internal_shared_resource_counter.get_mut())
                .as_mut_ptr(),
        }
    }
}

// post init

__rtic_internal_shared_resource_counter
.get_mut()
.write(core::mem::MaybeUninit::new(shared_resources.counter));
rtic::export::interrupt::enable();