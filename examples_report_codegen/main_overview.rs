

// main start
/* 
Label [lis:codegen_main_overview]  
Caption: 
Minimal overview of what is put where in the main function
*/

#[doc(hidden)]
mod rtic_ext {
    use super::*;
    #[no_mangle]
    unsafe extern "C" fn main() -> ! {
        // pre-init:
            // assertions of types and monotonics
            // enable interrupts (see hardware)
            // populate task queues (see resources)
            // unmasking of interrupts, configuring priority and asserting priority
            // Enable monotonics
        // Call to init
        // post-init:
            // Handle return of init
            // call to idle 
    }
}
