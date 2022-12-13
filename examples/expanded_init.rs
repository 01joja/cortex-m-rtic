#![feature(prelude_import,fmt_internals,core_panic,const_fmt_arguments_new)]
//! examples/init.rs
//#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]
#[prelude_import]
//use core::prelude::rust_2021::*;
//#[macro_use]
extern crate core;
//#[macro_use]
//extern crate compiler_builtins;

use panic_semihosting as _;
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use lm3s6965 as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    use cortex_m_semihosting::{debug, hprintln};
    /// User code from within the module
    /// User code end
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let _core: cortex_m::Peripherals = cx.core;
        let _device: lm3s6965::Peripherals = cx.device;
        let _x: &'static mut u32 = cx.local.x;
        let _cs_token: bare_metal::CriticalSection = cx.cs;
        hej();
        ::cortex_m_semihosting::export::hstdout_str("init\n").unwrap();
        debug::exit(debug::EXIT_SUCCESS);
        (Shared {}, Local {}, init::Monotonics())
    }


    fn hej(){
        hprintln!("hej").unwrap();
    }
    ///
    ///#user_hardware_tasks
    /// ||||
    /// \/\/
    /// /\/\
    /// ||||
    /// #user_hardware_tasks
    ///
    ///
    /// #user_software_tasks
    /// ||||
    /// \/\/
    /// /\/\
    /// ||||
    /// #user_software_tasks
    ///
    struct Shared {}
    struct Local {}
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `init` has access to
    pub struct __rtic_internal_initLocalResources {
        pub x: &'static mut u32,
    }
    /// Monotonics used by the system
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_Monotonics();
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_init_Context<'a> {
        /// Core (Cortex-M) peripherals
        pub core: rtic::export::Peripherals,
        /// Device peripherals
        pub device: lm3s6965::Peripherals,
        /// Critical section token for init
        pub cs: rtic::export::CriticalSection<'a>,
        /// Local Resources this task has access to
        pub local: init::LocalResources,
    }
    impl<'a> __rtic_internal_init_Context<'a> {
        #[inline(always)]
        pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
            __rtic_internal_init_Context {
                device: lm3s6965::Peripherals::steal(),
                cs: rtic::export::CriticalSection::new(),
                core,
                local: init::LocalResources::new(),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Initialization function
    pub mod init {
        #[doc(inline)]
        pub use super::__rtic_internal_initLocalResources as LocalResources;
        pub use super::__rtic_internal_Monotonics as Monotonics;
        pub use super::__rtic_internal_init_Context as Context;
    }
    ///
    /// #user_software_tasks
    /// ||||
    /// \/\/
    /// /\/\
    /// ||||
    /// #user_software_tasks
    ///
    /// app module
    impl __rtic_internal_initLocalResources {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_internal_initLocalResources {
                x: &mut *__rtic_internal_local_init_x.get_mut(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_internal_MASK_CHUNKS: usize = rtic::export::compute_mask_chunks([]);
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    const __rtic_internal_MASKS: [rtic::export::Mask<__rtic_internal_MASK_CHUNKS>; 3] = [
        rtic::export::create_mask([]),
        rtic::export::create_mask([]),
        rtic::export::create_mask([]),
    ];
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_local_init_x: rtic::RacyCell<u32> = rtic::RacyCell::new(0);
    ///
    /// #user_software_tasks
    /// ||||
    /// \/\/
    /// /\/\
    /// ||||
    /// #user_software_tasks
    ///
    #[doc(hidden)]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            const _CONST_CHECK: () = { if !rtic::export::have_basepri() {} else {} };
            let _ = _CONST_CHECK;
            rtic::export::interrupt::disable();
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
                .into();
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
                rtic::export::interrupt::enable();
            });
            loop {
                rtic::export::nop()
            }
        }
    }
}
