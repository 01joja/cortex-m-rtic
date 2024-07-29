
// init


// post init:

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

