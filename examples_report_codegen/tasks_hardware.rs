


// Hardware
/*
Catches the interrupt and then runs the task foo bounded to GPIOA with 
an priority of 1
*/
#[allow(non_snake_case)]
#[no_mangle]
unsafe fn GPIOA() {
    const PRIORITY: u8 = 1u8;
    rtic::export::run(PRIORITY, || { foo() });
}


// main

// generated for each used interrupt, in this case GPIOA
// if the priority of GPIOA is to big main will 
const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 1u8 as usize {
    {
        ::core::panicking::panic_fmt(
            format_args!(
                "Maximum priority used by interrupt vector \'GPIOA\' is more than supported by hardware",
            ),
        );
    };
};
core.NVIC
    .set_priority(
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOA,
        rtic::export::logical2hw(1u8, lm3s6965::NVIC_PRIO_BITS),
    );
rtic::export::NVIC::unmask(
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOA,
);


