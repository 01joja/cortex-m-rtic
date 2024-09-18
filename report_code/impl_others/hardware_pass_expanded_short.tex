pub mod app {
    // Always includes the device crate which contains the vector table
    use lm3s6965 as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    // #user_imports
    use cortex_m_semihosting::debug;
    use lm3s6965::Interrupt;
    
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtic::pend(Interrupt::UART0);
        (Shared {}, Local {}, init::Monotonics())
    }
    
    // Idle function
    fn idle() -> ! {
        use rtic::Mutex as _;        // added by rtic
        use rtic::mutex::prelude::*; // added by rtic
        // idle task instructions
    }

    // hardware task foo
    fn foo() {
        use rtic::Mutex as _;        // added by rtic
        use rtic::mutex::prelude::*; // added by rtic
        // foo task instructions 
    }
    
    // hardware task module
    pub mod foo {
        pub use super::foo as example;
    }

    // resources that are just forwarded.
    struct Shared {}
    struct Local {}

    // monotonics
    pub struct __rtic_internal_Monotonics();

    // context
    pub struct __rtic_internal_init_Context<'a> {
        // code for init context, see init
    }
    
    // init
    pub mod init {
        pub use super::__rtic_internal_Monotonics as Monotonics; // needs special for monotonic
        pub use super::__rtic_internal_init_Context as Context;  // always has to have context
    }

    // hardware interrupt
    unsafe fn UART0() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(PRIORITY, || { foo() });
    }

    // main
    #[doc(hidden)]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            rtic::export::interrupt::disable();
            /// enable hardware interrutps
            let _example_pre_init = "example pre init"; // from pre init
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
                let _example_post_init = "example post init"; // from post init
                rtic::export::interrupt::enable();
            });
            // call to idle
            idle()
        }
    }
}
