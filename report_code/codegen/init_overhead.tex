///Initialization function
pub mod init {
    pub use super::__rtic_internal_Monotonics as Monotonics;
    pub use super::__rtic_internal_init_Context as Context;
}

pub struct __rtic_internal_Monotonics();

pub struct __rtic_internal_init_Context<'a> {
    /// Core (Cortex-M) peripherals
    pub core: rtic::export::Peripherals,
    /// Device peripherals
    pub device: lm3s6965::Peripherals,
    /// Critical section token for init
    pub cs: rtic::export::CriticalSection<'a>,
}
impl<'a> __rtic_internal_init_Context<'a> {
    #[inline(always)]
    pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
        __rtic_internal_init_Context {
            device: lm3s6965::Peripherals::steal(),
            cs: rtic::export::CriticalSection::new(),
            core,
        }
    }
}