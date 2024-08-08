#[doc(hidden)]
mod rtic_ext {
    use super::*;
    #[no_mangle]
    unsafe extern "C" fn main() -> ! {
        // pre-init:
            // Assertions of that types implements "Send" (see resources)
            // Assertion that the monotonic type is a monotonic (see monotonic)
            rtic::export::interrupt::disable(); // Disables interrupts
            // Populate task queues (see resources)
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
                .into(); // Access the peripherals
            // Unmasking of interrupts, configuring priority and asserting priority (see hardware and software)
            // Enable monotonics (see monotonics)
        // Function that may only run once that executes init and handles resources
        fn __rtic_init_resources<F>(f: F)
        where
            F: FnOnce(),
        {
            f();
        }
        __rtic_init_resources(|| {
            // Call to the init task
            let (shared_resources, local_resources, mut monotonics) = init(
                init::Context::new(core.into()),
            );
            // post-init:
                // Handle the return of resources form init (see resources)
                // Handle the return of monotonic from init and reset the monotonic (see monotonic)
                rtic::export::interrupt::enable();
        });
        // Call to idle or loop for infinity
        idle(idle::Context::new(&rtic::export::Priority::new(0)))
        // or
        loop {
            rtic::export::nop()
        }
    }
}