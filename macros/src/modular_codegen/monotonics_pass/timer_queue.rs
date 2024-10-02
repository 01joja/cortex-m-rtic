use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::ast::App;

use crate::modular_codegen::generate_syntax;
use super::m_names;

use crate::{analyze::Analysis, check::Extra, codegen::util};

/// Generates timer queues and timer queue handlers
#[allow(clippy::too_many_lines)]
pub fn codegen(app: &App, analysis: &Analysis, _extra: &Extra) -> Vec<TokenStream2> {
    let mut items = vec![];

    if !app.monotonics.is_empty() {
        // Generate the marker counter used to track for `cancel` and `reschedule`
        let tq_marker = m_names::timer_queue_marker();
        items.push(quote!(
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            #[allow(non_upper_case_globals)]
            static #tq_marker: rtic::RacyCell<u32> = rtic::RacyCell::new(0);
        ));

        let t = m_names::schedule_tasks();

        // Enumeration of `schedule`-able tasks
        {
            let variants = app
                .software_tasks
                .iter()
                .map(|(name, task)| {
                    let cfgs = &task.cfgs;

                    quote!(
                        #(#cfgs)*
                        #name
                    )
                })
                .collect::<Vec<_>>();

            // For future use
            // let doc = "Tasks that can be scheduled".to_string();
            items.push(quote!(
                #[doc(hidden)]
                #[allow(non_camel_case_types)]
                #[derive(Clone, Copy)]
                pub enum #t {
                    #(#variants,)*
                }
            ));
        }
    }

    for (monotonic_name, monotonic) in &app.monotonics {
        let tq = m_names::timer_queue(monotonic_name);
        let schedule_task = m_names::schedule_tasks();
        let sys_tick = &monotonic.ty;
        let m_ident = m_names::monotonic_storage(monotonic_name);

        // Static variables and resource proxy
        {   
            // adds the capacity for all softwaretasks together
            let cap: usize = app
                .software_tasks
                .iter()
                .map(|(_name, task)| task.args.capacity as usize)
                .sum();
            let capacity = generate_syntax::capacity_literal(cap);
            let tq_ty = quote!(rtic::export::TimerQueue<#sys_tick, #schedule_task, #capacity>);

            items.push(quote!(
                #[doc(hidden)]
                #[allow(non_camel_case_types)]
                #[allow(non_upper_case_globals)]
                static #tq: rtic::RacyCell<#tq_ty> =
                    rtic::RacyCell::new(rtic::export::TimerQueue(rtic::export::SortedLinkedList::new_u16()));
            ));

            let mono = m_names::monotonic_storage(monotonic_name);
            
            items.push(quote!(
                #[doc(hidden)]
                #[allow(non_camel_case_types)]
                #[allow(non_upper_case_globals)]
                static #mono: rtic::RacyCell<Option<#sys_tick>> = rtic::RacyCell::new(None);
            ));
        }

        // Timer queue handler
        {
            let enum_ = m_names::interrupt();
            let rt_err = generate_syntax::rt_error();

            let arms = app
                .software_tasks
                .iter()
                .map(|(name, task)| {
                    let cfgs = &task.cfgs;
                    let priority = task.args.priority;
                    let interrupt = &analysis.interrupts.get(&priority).expect("RTIC-ICE: interrupt not found").0;

                    let pend = {
                        quote!(
                            rtic::pend(#rt_err::interrupt::#interrupt);
                        )
                    };

                    quote!(
                        #(#cfgs)*
                        #schedule_task::#name => {
                            rtic::export::interrupt::free(|_| (&mut *#name::__internal_PRIO_REQUEST_Q.get_mut()).split().0.enqueue_unchecked((#name::__internal_dispatcher_task_name::#name, index)));

                            #pend
                        }
                    )
                })
                .collect::<Vec<_>>();

            let bound_interrupt = &monotonic.args.binds;
            let disable_isr = if &*bound_interrupt.to_string() == "SysTick" {
                quote!(core::mem::transmute::<_, rtic::export::SYST>(()).disable_interrupt())
            } else {
                quote!(rtic::export::NVIC::mask(#rt_err::#enum_::#bound_interrupt))
            };

            items.push(quote!(
                #[no_mangle]
                #[allow(non_snake_case)]
                unsafe fn #bound_interrupt() {
                    while let Some((task, index)) = rtic::export::interrupt::free(|_|
                        if let Some(mono) = (&mut *#m_ident.get_mut()).as_mut() {
                            (&mut *#tq.get_mut()).dequeue(|| #disable_isr, mono)
                        } else {
                            // We can only use the timer queue if `init` has returned, and it
                            // writes the `Some(monotonic)` we are accessing here.
                            core::hint::unreachable_unchecked()
                        })
                    {
                        match task {
                            #(#arms)*
                        }
                    }

                    rtic::export::interrupt::free(|_| if let Some(mono) = (&mut *#m_ident.get_mut()).as_mut() {
                        mono.on_interrupt();
                    });
                }
            ));
        }
    }

    items
}
