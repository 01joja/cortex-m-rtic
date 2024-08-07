

// modules:
pub mod bar {
    pub use super::__rtic_internal_barLocalResources as LocalResources;
    pub use super::__rtic_internal_bar_Context as Context;
    pub use super::__rtic_internal_bar_spawn as spawn;
}

pub mod foo {
    pub use super::__rtic_internal_fooLocalResources as LocalResources;
    pub use super::__rtic_internal_foo_Context as Context;
    pub use super::__rtic_internal_foo_spawn as spawn;
}

// task code:
pub struct __rtic_internal_fooLocalResources<'a> {
    pub local_foo: &'a mut i32,
}

pub struct __rtic_internal_foo_Context<'a> {
    /// Local Resources this task has access to
    pub local: foo::LocalResources<'a>,
}
impl<'a> __rtic_internal_foo_Context<'a> {
    #[inline(always)]
    pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
        __rtic_internal_foo_Context {
            local: foo::LocalResources::new(),
        }
    }
}

///Local resources `bar` has access to
pub struct __rtic_internal_barLocalResources<'a> {
    pub local_bar: &'a mut i32,
}
/// Execution context
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct __rtic_internal_bar_Context<'a> {
    /// Local Resources this task has access to
    pub local: bar::LocalResources<'a>,
}
impl<'a> __rtic_internal_bar_Context<'a> {
    #[inline(always)]
    pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
        __rtic_internal_bar_Context {
            local: bar::LocalResources::new(),
        }
    }
}

// generated for each local resource:

static __rtic_internal_local_resource_local_to_idle: rtic::RacyCell<
    core::mem::MaybeUninit<i64>,
> = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());

impl<'a> __rtic_internal_fooLocalResources<'a> {
    #[inline(always)]
    pub unsafe fn new() -> Self {
        __rtic_internal_fooLocalResources {
            local_foo: &mut *(&mut *__rtic_internal_local_resource_local_foo
                .get_mut())
                .as_mut_ptr(),
        }
    }
}

// generated for each late local resource:
static __rtic_internal_local_bar_local_bar: rtic::RacyCell<i32> = rtic::RacyCell::new(
    0,
);

impl<'a> __rtic_internal_barLocalResources<'a> {
    #[inline(always)]
    pub unsafe fn new() -> Self {
        __rtic_internal_barLocalResources {
            local_bar: &mut *__rtic_internal_local_bar_local_bar.get_mut(),
        }
    }
}

// post init:
    __rtic_internal_local_resource_local_foo
    .get_mut()
    .write(core::mem::MaybeUninit::new(local_resources.local_foo));


