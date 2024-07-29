

// main start

#[doc(hidden)]
mod rtic_ext {
    use super::*;
    #[no_mangle]
    unsafe extern "C" fn main() -> ! {
        // pre-init:
            // assertions of types and monotonics (see monotonic)
            // all types that are used as resources and sent by messages needs
            // to implement send
            rtic::export::assert_send::<i32>();

            // populate task queues (see resources)
            // access to peripherals.
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
                .into();
            // unmasking of interrupts, configuring priority and asserting priority (see hardware)
            // Enable monotonics (see monotonics)
        // Call to init
        // post-init:
            // Handle return of init (see init)
            // call to idle (see idle)
    }
}

// assertions of types used in the application and 
rtic::export::assert_send::<i32>();
rtic::export::assert_monotonic::<Systick<100>>();

// assertion of hardware interrupts

const _CONST_CHECK: () = {
    if !rtic::export::have_basepri() {
        if (lm3s6965::Interrupt::UART0 as usize)
            >= (__rtic_internal_MASK_CHUNKS * 32)
        {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "An interrupt out of range is used while in armv6 or armv8m.base",
                    ),
                );
            };
        }
    } else {}
};

// generates one of these for each interrupt used
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


// generated from report_used_for_main_expanded

#[doc(hidden)]
mod rtic_ext {
    use super::*;
    #[no_mangle]
    unsafe extern "C" fn main() -> ! {
        rtic::export::assert_send::<i32>();
        rtic::export::assert_monotonic::<Systick<100>>();
        const _CONST_CHECK: () = {
            if !rtic::export::have_basepri() {
                if (lm3s6965::Interrupt::UART0 as usize)
                    >= (__rtic_internal_MASK_CHUNKS * 32)
                {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "An interrupt out of range is used while in armv6 or armv8m.base",
                            ),
                        );
                    };
                }
            } else {}
        };
        let _ = _CONST_CHECK;
        rtic::export::interrupt::disable();
        (0..1u8)
            .for_each(|i| {
                (&mut *__rtic_internal_foo_FQ.get_mut()).enqueue_unchecked(i)
            });
        (0..2u8)
            .for_each(|i| {
                (&mut *__rtic_internal_bar_FQ.get_mut()).enqueue_unchecked(i)
            });
        let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
            .into();
        let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0;
        let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::GPIOA;
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
        const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 2u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'SSI0\' is more than supported by hardware",
                    ),
                );
            };
        };
        core.NVIC
            .set_priority(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0,
                rtic::export::logical2hw(2u8, lm3s6965::NVIC_PRIO_BITS),
            );
        rtic::export::NVIC::unmask(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0,
        );
        const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS) < 1u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'UART0\' is more than supported by hardware",
                    ),
                );
            };
        };
        core.NVIC
            .set_priority(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART0,
                rtic::export::logical2hw(1u8, lm3s6965::NVIC_PRIO_BITS),
            );
        rtic::export::NVIC::unmask(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART0,
        );
        const _: () = if (1 << lm3s6965::NVIC_PRIO_BITS)
            < (1 << lm3s6965::NVIC_PRIO_BITS) as usize
        {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by monotonic \'MyMono\' is more than supported by hardware",
                    ),
                );
            };
        };
        core.SCB
            .set_priority(
                rtic::export::SystemHandler::SysTick,
                rtic::export::logical2hw(
                    (1 << lm3s6965::NVIC_PRIO_BITS),
                    lm3s6965::NVIC_PRIO_BITS,
                ),
            );
        if !<Systick<100> as rtic::Monotonic>::DISABLE_INTERRUPT_ON_EMPTY_QUEUE {
            core::mem::transmute::<_, rtic::export::SYST>(()).enable_interrupt();
        }
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
            __rtic_internal_shared_resource_shared_r
                .get_mut()
                .write(core::mem::MaybeUninit::new(shared_resources.shared_r));
            __rtic_internal_shared_resource_shared_lock_free
                .get_mut()
                .write(
                    core::mem::MaybeUninit::new(shared_resources.shared_lock_free),
                );
            __rtic_internal_local_resource_local_r
                .get_mut()
                .write(core::mem::MaybeUninit::new(local_resources.local_r));
            monotonics.0.reset();
            __rtic_internal_MONOTONIC_STORAGE_MyMono
                .get_mut()
                .write(Some(monotonics.0));
            rtic::export::interrupt::enable();
        });
        loop {
            rtic::export::nop()
        }
    }
}

