// Module that holds the priority for 
// the resources that needs to be locked
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
