//! examples/hardware.rs

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;
// use cortex_m::peripheral::{syst::SystClkSource, SYST};

#[rtic::app(device = lm3s6965)]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;
    use cortex_m::peripheral::{syst::SystClkSource, SYST};

    
    
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Pends the UART0 interrupt but its handler won't run until *after*
        // `init` returns because interrupts are disabled
        
        unsafe{
            systick = cx.core.SYST;
            systick.set_clock_source(SystClkSource::Core);
            hprintln!("{:?}",systick.cvr.read()).unwrap();
        }
        rtic::pend(Interrupt::UART0); // equivalent to NVIC::pend
        hprintln!("init").unwrap();

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(cx: idle::Context) -> ! {
        // interrupts are enabled again; the `UART0` handler runs at this point

        hprintln!("idle").unwrap();
        rtic::pend(Interrupt::UART0);

        unsafe{
            hprintln!("{:?}",systick.cvr.read()).unwrap();
        }        

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator

        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = UART0, local = [times: u32 = 0])]
    fn uart0(cx: uart0::Context) {
        // Safe access to local `static mut` variable
        *cx.local.times += 1;

        hprintln!(
            "UART0 called {} time{}",
            *cx.local.times,
            if *cx.local.times > 1 { "s" } else { "" }
        )
        .unwrap();
    }
}
