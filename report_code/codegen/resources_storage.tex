// Any resource except late locals
static __rtic_internal_any_resource: rtic::RacyCell<
    core::mem::MaybeUninit<i32>,
> = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());

// late local resource bound to foo
static __rtic_internal_local_foo_r_late_local: rtic::RacyCell<i32> = rtic::RacyCell::new(
    0, // initial value defined in the task
);

// post_init: (see "Main function")
let (shared_resources, local_resources, mut monotonics) = init(
    init::Context::new(core.into()),
);
// initialises a stared resource with values defined in init
__rtic_internal_any_shared_resource
    .get_mut()
    .write(core::mem::MaybeUninit::new(shared_resources.any_shared_resource));
// initialises a stared resource with values defined in init
    __rtic_internal_any_local_resource
    .get_mut()
    .write(core::mem::MaybeUninit::new(local_resources.any_local_resource));