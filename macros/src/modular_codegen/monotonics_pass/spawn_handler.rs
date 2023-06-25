use proc_macro2::{Span,TokenStream as TokenStream2};
use quote::quote;
use rtic_syntax::{ast::App, Context, analyze::Priority, ast::SoftwareTask};

use std::str::FromStr;
use std::collections::HashMap;

use crate::{
    analyze::Analysis,
    check::Extra,
    codegen::util, modular_codegen::generate_syntax,
};


use syn::{Ident, LitInt, Path};

use super::m_names;

/// Creates the spawn_after and spawn_later functions.
pub fn codegen(
    app: &App, 
    _extra: &Extra,
) -> (
    // Spawn_later, Spawn_at, cancel, reschedule_after, reschedule_at, 
    Vec<TokenStream2>,
    // Modules for software tasks
    Vec<TokenStream2>,
){
    // Needs to add the monotonics to all software tasks.
    let monotonics = &app.monotonics;
    let software_tasks = &app.software_tasks;

    let mut spawn_handlers = vec![];
    let mut modules = vec![];

    let schedule_tasks_enum = m_names::schedule_tasks();
    let interrupt_enum = m_names::interrupt();
    let timer_q_marker = m_names::timer_queue_marker();

    for (monotonic_name, monotonic) in monotonics{

        //do I need this?? internal_m_name???
        // let internal_m_name = m_names::internal_monotonic_name(monotonic_name);
        let monotonic_storage = m_names::monotonic_storage(monotonic_name);
        let timer_queue = m_names::timer_queue(monotonic_name);
        let bounded_interrupt = &monotonic.args.binds;
        let debug_struct_string = format!("{monotonic_name}::SpawnHandle");
        println!("monotonic name: {monotonic_name}");

        for (sw_name, task) in software_tasks{

            let instants = m_names::monotonic_instants(monotonic_name, sw_name);

            // formats message passing variables (code is the same in software task pass.
            // maybe should put it in some file both can have access to. But don't want
            // them to be dependent on "outside" code, sorry long rant)
            let mut task_messages: Vec<TokenStream2> = vec![];
            let mut task_messages_internal: Vec<TokenStream2> = vec![];
            let mut task_messages_names: Vec<TokenStream2> = vec![];
            let mut task_messages_types: Vec<TokenStream2> = vec![];
            let mut no_variabel = 0;
            for input in &task.inputs{
                let variable = &input.pat;
                let variable_internal = Ident::new(
                    &format!("_{no_variabel}"), Span::call_site());
                no_variabel += 1;
                let the_type = &input.ty;
                task_messages.push(quote!(#variable:#the_type,));
                task_messages_internal.push(quote!(#variable_internal:#the_type,));
                task_messages_names.push(quote!(#variable_internal,));
                task_messages_types.push(quote!(#the_type,));
            }

            let (enable_interrupt, pend) = if &*bounded_interrupt.to_string() == "SysTick" {
                (
                    quote!(core::mem::transmute::<_, rtic::export::SYST>(()).enable_interrupt()),
                    quote!(rtic::export::SCB::set_pendst()),
                )
            } else {
                let rt_error = m_names::rt_error();
                (
                    quote!(rtic::export::NVIC::unmask(#rt_error::#interrupt_enum::#bounded_interrupt)),
                    quote!(rtic::pend(#rt_error::#interrupt_enum::#bounded_interrupt)),
                )
            };

            // Fetches items from previews passes
            let mut module_items = vec![];
            if let Some(get_module) = app.pass_modules.get(sw_name){
                let items = &get_module.items;
                module_items.push(quote!(#(#items)*));
            }

            // I don't know what is meant by the "default monotonic".
            if monotonic.args.default{
                module_items.push(
                    quote!{
                        pub use #monotonic_name::spawn_after;
                        pub use #monotonic_name::spawn_at;
                        pub use #monotonic_name::SpawnHandle;
                    }
                );
            }
            
            // names
            let name_spawn_after = m_names::monotonic_spawn_after(&monotonic_name, sw_name);
            let name_spawn_at = m_names::monotonic_spawn_at(&monotonic_name, sw_name);
            let name_spawn_handler = m_names::monotonic_spawn_handler(&monotonic_name, sw_name);

            // the module for the software task.
            modules.push(
                quote!{
                    #[__rtic_pass_module(has_monotonic = true)]
                    pub mod #sw_name{
                        #(#module_items)*
                        #[allow(non_snake_case)]
                        pub mod #monotonic_name{
                            pub use super::super::#name_spawn_after as spawn_after;
                            pub use super::super::#name_spawn_at as spawn_at;
                            pub use super::super::#name_spawn_handler as SpawnHandle;
                        }
                    }
                }
            );

            let sys_tic = &monotonic.ty;
            let cap = task.args.capacity as usize;
            let capacity = generate_syntax::capacity_literal(cap);
            let unint_ritc = format!(".uninit.rtic_{monotonic_name}_{sw_name}");

            spawn_handlers.push(
                quote!{
                    #[link_section = #unint_ritc]
                    #[allow(non_camel_case_types)]
                    #[allow(non_upper_case_globals)]
                    #[doc(hidden)]
                    static #instants: rtic::RacyCell<
                    [core::mem::MaybeUninit<<#sys_tic as rtic::Monotonic>::Instant>; #capacity],
                    >  = rtic::RacyCell::new([core::mem::MaybeUninit::uninit()]);

                    // #(#cfgs)*
                    #[allow(non_snake_case)]
                    #[allow(non_camel_case_types)]
                    pub struct #name_spawn_handler {
                        #[doc(hidden)]
                        marker: u32,
                    }

                    impl core::fmt::Debug for #name_spawn_handler {
                        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                            f.debug_struct(#debug_struct_string).finish()
                        }
                    }

                    // #(#cfgs)*
                    impl #name_spawn_handler {
                        pub fn cancel(self) -> Result<(#(#task_messages_types)*), ()> {
                            rtic::export::interrupt::free(|_| unsafe {
                                let timer_queue = &mut *#timer_queue.get_mut();
                                if let Some((_task, index)) = timer_queue.cancel_marker(self.marker) {
                                    // Get the message
                                    let msg = (&*#sw_name::__internal_input_queue
                                        .get())
                                        .get_unchecked(usize::from(index))
                                        .as_ptr()
                                        .read();
                                    // Return the index to the free queue
                                    (&mut *#sw_name::__internal_function_queue.get_mut()).split().0.enqueue_unchecked(index);

                                    Ok(msg)
                                } else {
                                    Err(())
                                }
                            })
                        }

                        #[inline]
                        pub fn reschedule_after(
                            self,
                            duration: <#monotonic_name as rtic::Monotonic>::Duration
                        ) -> Result<Self, ()> {
                            self.reschedule_at(monotonics::#monotonic_name::now() + duration)
                        }

                        pub fn reschedule_at(
                            self,
                            instant: <#monotonic_name as rtic::Monotonic>::Instant
                        ) -> Result<Self, ()> {
                            rtic::export::interrupt::free(|_| unsafe {
                                let marker = #timer_q_marker.get().read();
                                #timer_q_marker.get_mut().write(marker.wrapping_add(1));

                                let timer_queue = (&mut *#timer_queue.get_mut());

                                timer_queue.update_marker(self.marker, marker, instant, || #pend).map(|_| #sw_name::#monotonic_name::SpawnHandle { marker })
                            })
                        }
                    }


                    /// Spawns the task after a set duration relative to the current time
                    ///
                    /// This will use the time `Instant::new(0)` as baseline if called in `#[init]`,
                    /// so if you use a non-resetable timer use `spawn_at` when in `#[init]`
                    #[allow(non_snake_case)]
                    pub fn #name_spawn_after(
                        duration: <#monotonic_name as rtic::Monotonic>::Duration,
                        #(#task_messages_internal)*
                    ) -> Result<#sw_name::#monotonic_name::SpawnHandle, (#(#task_messages_types)*)>
                    {
                        let instant = monotonics::#monotonic_name::now();

                        #name_spawn_at(instant + duration, #(#task_messages_names)*)
                    }

                    /// Spawns the task at a fixed time instant.
                    /// Needs access to the software tasks function and input queue. 
                    #[allow(non_snake_case)]
                    pub fn #name_spawn_at(
                        instant: <#monotonic_name as rtic::Monotonic>::Instant,
                        #(#task_messages_internal)*
                    ) -> Result<#sw_name::#monotonic_name::SpawnHandle, (#(#task_messages_types)*)> {
                        unsafe {
                            let input = (#(#task_messages_names)*);
                            if let Some(index) = rtic::export::interrupt::free(|_| 
                                (&mut *#sw_name::__internal_function_queue.get_mut()).dequeue()
                            ) {
                                (&mut *#sw_name::__internal_input_queue
                                    .get_mut())
                                    .get_unchecked_mut(usize::from(index))
                                    .as_mut_ptr()
                                    .write(input);

                                (&mut *#instants
                                    .get_mut())
                                    .get_unchecked_mut(usize::from(index))
                                    .as_mut_ptr()
                                    .write(instant);

                                rtic::export::interrupt::free(|_| {
                                    let marker = #timer_q_marker.get().read();
                                    let nr = rtic::export::NotReady {
                                        instant,
                                        index,
                                        task: #schedule_tasks_enum::#sw_name,
                                        marker,
                                    };

                                    #timer_q_marker.get_mut().write(#timer_q_marker.get().read().wrapping_add(1));

                                    let timer_queue = &mut *#timer_queue.get_mut();

                                    timer_queue.enqueue_unchecked(
                                        nr,
                                        || #enable_interrupt,
                                        || #pend,
                                        (&mut *#monotonic_storage.get_mut()).as_mut());

                                    Ok(#sw_name::#monotonic_name::SpawnHandle { marker })
                                })
                            } else {
                                Err(input)
                            }
                        }
                    }

                }
            )
        }
    }

    return (spawn_handlers,modules);
}


