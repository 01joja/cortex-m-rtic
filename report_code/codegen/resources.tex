/// needs to be locked
mod shared_resources {
    use rtic::export::Priority;
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct r_shared_that_needs_to_be_locked<'a> {
        priority: &'a Priority,
    }
    impl<'a> r_shared_that_needs_to_be_locked<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a Priority) -> Self {
            r_shared_that_needs_to_be_locked {
                priority,
            }
        }
        #[inline(always)]
        pub unsafe fn priority(&self) -> &Priority {
            self.priority
        }
    }
}

///Local resources `foo` has access to
pub struct __rtic_internal_fooLocalResources<'a> {
    pub r_local: &'a mut i32,
    pub local_bar: &'a mut i32,
}
impl<'a> __rtic_internal_fooLocalResources<'a> {
    #[inline(always)]
    pub unsafe fn new() -> Self {
        __rtic_internal_fooLocalResources {
            r_local: &mut *(&mut *__rtic_internal_local_resource_r_local.get_mut())
                .as_mut_ptr(),
            local_bar: &mut *__rtic_internal_local_foo_local_bar.get_mut(),
        }
    }
}

///Shared resources `foo` has access to
pub struct __rtic_internal_fooSharedResources<'a> {
    pub r_shared: shared_resources::r_shared_that_needs_to_be_locked<'a>,
    pub r_lock_free: &'a mut i32,
}
impl<'a> __rtic_internal_fooSharedResources<'a> {
    #[inline(always)]
    pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
        __rtic_internal_fooSharedResources {
            r_shared: shared_resources::r_shared_that_needs_to_be_locked::new(
                priority,
            ),
            r_lock_free: &mut *(&mut *__rtic_internal_shared_resource_r_lock_free
                .get_mut())
                .as_mut_ptr(),
        }
    }
}

/// Context
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

// module (API)
pub mod foo {
    #[doc(inline)]
    pub use super::__rtic_internal_fooLocalResources as LocalResources;
    #[doc(inline)]
    pub use super::__rtic_internal_fooSharedResources as SharedResources;
    pub use super::__rtic_internal_foo_Context as Context;
}

// Storage of shared resource and its lock implementation
static __rtic_internal_shared_resource_r_shared: rtic::RacyCell<
    core::mem::MaybeUninit<i32>,
> = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
impl<'a> rtic::Mutex for shared_resources::r_shared_that_needs_to_be_locked<'a> {
    type T = i32;
    #[inline(always)]
    fn lock<RTIC_INTERNAL_R>(
        &mut self,
        f: impl FnOnce(&mut i32) -> RTIC_INTERNAL_R,
    ) -> RTIC_INTERNAL_R {
        /// Priority ceiling
        const CEILING: u8 = 1u8;
        unsafe {
            rtic::export::lock(
                __rtic_internal_shared_resource_r_shared.get_mut() as *mut _,
                self.priority(),
                CEILING,
                lm3s6965::NVIC_PRIO_BITS,
                &__rtic_internal_MASKS,
                f,
            )
        }
    }
}

// Lock free resource
static __rtic_internal_shared_resource_r_lock_free: rtic::RacyCell<
    core::mem::MaybeUninit<i32>,
> = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());

// local resource
static __rtic_internal_local_resource_r_local: rtic::RacyCell<
    core::mem::MaybeUninit<i32>,
> = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());

// late local resource
static __rtic_internal_local_foo_local_bar: rtic::RacyCell<i32> = rtic::RacyCell::new(
    0,
);


