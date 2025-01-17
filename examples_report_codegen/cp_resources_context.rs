// Task foo API
pub mod foo {
    pub use super::__rtic_internal_fooLocalResources as LocalResources;
    pub use super::__rtic_internal_fooSharedResources as SharedResources;
    pub use super::__rtic_internal_foo_Context as Context;
}

// Execution context that gives access to the resources
pub struct __rtic_internal_foo_Context<'a> {
    /// Local Resources this task has access to
    pub local: foo::LocalResources<'a>,
    /// Shared Resources this task has access to
    pub shared: foo::SharedResources<'a>,
}
impl<'a> __rtic_internal_foo_Context<'a> {
    #[inline(always)]
    // used to populate the context when the task is executed
    pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
        __rtic_internal_foo_Context {
            local: foo::LocalResources::new(),
            shared: foo::SharedResources::new(priority),
        }
    }
}

// Local resources
pub struct __rtic_internal_fooLocalResources<'a> {
    pub r_local: &'a mut i32,
    pub r_late_local: &'a mut i32,
}
impl<'a> __rtic_internal_fooLocalResources<'a> {
    #[inline(always)]
    pub unsafe fn new() -> Self {
        __rtic_internal_fooLocalResources {
            // Returns a raw mutable pointer
            r_local: &mut *(&mut *__rtic_internal_local_resource_r_local.get_mut())
                .as_mut_ptr(),
            // Just returns a mutable
            r_late_local: &mut *__rtic_internal_local_foo_r_late_local.get_mut(),
        }
    }
}

///Shared resources `foo` has access to
pub struct __rtic_internal_fooSharedResources<'a> {
    pub r_shared: shared_resources::r_shared_that_needs_to_be_locked<'a>,
    pub r_lock_free: &'a mut i32,
    pub r_only_shared: &'a i32,
}
impl<'a> __rtic_internal_fooSharedResources<'a> {
    #[inline(always)]
    pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
        __rtic_internal_fooSharedResources {
            // This need to be locked and there for it has a priority
            r_shared: shared_resources::r_shared_that_needs_to_be_locked::new(
                priority,
            ),
            // This is lock free and can be modified. Returns a raw mutable pointer
            r_lock_free: &mut *(&mut *__rtic_internal_shared_resource_r_lock_free
                .get_mut())
                .as_mut_ptr(), 
            // Returns a raw pointer that is NOT mutable
            r_only_shared: &*(&*__rtic_internal_shared_resource_r_only_shared.get())
                .as_ptr(),
        }
    }
}