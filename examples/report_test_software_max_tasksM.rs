//! examples/spawn.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, 
    dispatchers = [UART0,UART1,UART2,SSI0,I2C0,GPIOA,GPIOB,GPIOC], 
    compiler_passes = [standard]
    )]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        // Used to show that it dispatches in the right order and depending on priority
        one_one::spawn().unwrap();
        one_two::spawn().unwrap();
        one_three::spawn().unwrap();
        bar::spawn().unwrap();
        foo::spawn().unwrap();
        two::spawn().unwrap();
        three_one::spawn().unwrap();
        three_two::spawn().unwrap();
        three_three::spawn().unwrap();
        four::spawn().unwrap();
        five::spawn().unwrap();
        six::spawn().unwrap();
        seven::spawn().unwrap();
        eight::spawn().unwrap();


        hprintln!("init, I spawned all tasks once").unwrap();

        (Shared {}, Local {}, init::Monotonics())
    }

    #[task(priority = 1)]
    fn foo(_: foo::Context) {
        hprintln!("foo").unwrap();

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    #[task(priority = 1)]
    fn bar(_: bar::Context) {
        hprintln!("bar").unwrap();
    }
    #[task(priority = 2)]
    fn two(_: two::Context) {
        hprintln!("two").unwrap();
    }

    #[task(priority = 3)]
    fn three_one(_: three_one::Context) {
        hprintln!("three_one").unwrap();
    }
    
    #[task(priority = 1)]
    fn one_three(_: one_three::Context) {
        hprintln!("one_three").unwrap();
    }

    #[task(priority = 4)]
    fn four(_: four::Context) {
        hprintln!("four").unwrap();
    }
    
    #[task(priority = 3)]
    fn three_two(_: three_two::Context) {
        hprintln!("three_two").unwrap();
    }

    #[task(priority = 5)]
    fn five(_: five::Context) {
        hprintln!("five").unwrap();
    }
    
    #[task(priority = 1)]
    fn one_two(_: one_two::Context) {
        hprintln!("one_two").unwrap();
    }

    #[task(priority = 6)]
    fn six(_: six::Context) {
        hprintln!("six").unwrap();
    }

    #[task(priority = 3)]
    fn three_three(_: three_three::Context) {
        hprintln!("three_three").unwrap();
    }

    #[task(priority = 7)]
    fn seven(_: seven::Context) {
        hprintln!("seven").unwrap();
    }

    #[task(priority = 8)]
    fn eight(_: eight::Context) {
        hprintln!("eight").unwrap();
    }

    #[task(priority = 1)]
    fn one_one(_: one_one::Context) {
        hprintln!("one_one").unwrap();
    }


}
