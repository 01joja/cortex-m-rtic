use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{ast::App, Context, analyze::Priority};

use std::str::FromStr;
use std::collections::HashMap;

use crate::{
    analyze::Analysis,
    check::Extra,
    codegen::util,
};


/// Creates dispatchers for each priority and binds each dispatcher to given 
/// interrupt. The dispatchers will work as hardware tasks.
/// 
/// Every software task is bund to corresponding dispatcher with same prio.
/// The dispatcher will then take care of running the right task.
/// 
/// The overhead that makes calls like "foo::spawn().unwrap();" possible. 
/// 
/// TODO: local and shared resources. Resource passing
pub fn codegen(
    app: &App, 
    _analysis: &Analysis,
    extra: &Extra,
) -> (
    // Dispatchers
    Vec<TokenStream2>,
    // Software_tasks - the original code that the user have 
    // written.
    Vec<TokenStream2>,
    // Overhead created for the Software_tasks. Like
    // spawn and context
    Vec<TokenStream2>,
    // A function that initializes all things in init.
    TokenStream2){

    // Key: priority
    // Value: Vec<software_tasks>
    // Sorts tasks in to vectors of tasks that has the same prio
    let mut tasks_priority = HashMap::new();
    
    
    for task in &app.software_tasks{
        let priority = &task.1.args.priority;
        let tasks = match tasks_priority.get(priority){
            None => {vec![task]}
            Some(t) => {
                let mut task = vec![task];
                task.extend(t);
                task
            }
        };
        tasks_priority.insert(priority, tasks);
    }

    let device = &extra.device;
    let mut software_tasks = vec![];
    let mut tasks_overhead = vec![];
    let mut dispatchers = vec![];
    let mut interrupts = vec![];
    interrupts.extend(&app.args.extern_interrupts);
    
    let mut init_tasks = vec![];

    for (priority,tasks) in tasks_priority{
        let interrupt = interrupts.pop().unwrap().0; 
        let priority_lit = util::priority_literal(priority);

        let mut call_to_software_task = vec![];
        let mut same_prio_tasks = vec![];

        //TODO - better names
        let max = util::capacity_literal(tasks.len() + 1);
        let enum_name_task_same_prio = util::enum_name(priority);
        let dispatcher_request_queue = 
            quote!(rtic::export::SCRQ<#enum_name_task_same_prio, #max>);

        
        let name_queue = format!("dispatcher_queue_{}",priority);
        let dispatcher_queue = util::mark_internal_name(name_queue.as_str());

        for (name, task)in tasks{
            let attrs = &task.attrs;
            let cfgs = &task.cfgs;
            let stmts  = &task.stmts;

            // Create free queues and inputs / instants buffers
            let task_f_queue = util::fq_ident(name);

            call_to_software_task.push(quote!{
                #enum_name_task_same_prio::#name =>{
                    // let () = message passing?
                    (&mut *#task_f_queue.get_mut()).split().0.enqueue_unchecked(index);
                    let priority = &rtic::export::Priority::new(PRIORITY);
                    #name(#name::Context::new(priority))
                }
            });

            // This transforms the software task to a function that the dispatcher
            // can call.
            software_tasks.push(quote!{
                #(#attrs)*
                #(#cfgs)*
                #[allow(non_snake_case)]
                fn #name(_: #name::Context){
                    use rtic::Mutex as _;
                    use rtic::mutex::prelude::*;
                    #(#stmts)*
                }  
            });
                
            let context_name = util::internal_task_ident(name, "Context");
            let spawn_name = util::internal_task_ident(name, "spawn");

            
            let cap = task.args.capacity;
            let cap_lit = util::capacity_literal(cap as usize);
            let cap_lit_p1 = util::capacity_literal(cap as usize + 1);
            
            

            // Removed mk_uninit because monotonic is not implemented yet
            // Dont really know what rtic::export do.
            #[allow(clippy::redundant_closure)]
            let task_f_queue_ty = quote!(rtic::export::SCFQ<#cap_lit_p1>);

            // The dispatcher needs to be able to access all tasks of it's
            // priority.
            same_prio_tasks.push(quote!(#name,));

            // The software overhead needed for the task
            tasks_overhead.push(quote!{

                /// Queue version of a free-list that keeps track of empty slots in
                /// the following buffers
                #[allow(non_camel_case_types)]
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                static #task_f_queue: rtic::RacyCell<#task_f_queue_ty> = rtic::RacyCell::new(
                    rtic::export::Queue::new(),
                );

                /// Binds internal task overhead to the user defined task.
                pub mod #name {
                    pub use super::#context_name as Context;
                    pub use super::#spawn_name as spawn;
                }

                /// internal task context
                pub struct #context_name {}
                impl #context_name{
                    #[inline(always)]
                    pub unsafe fn new(priority: &rtic::export::Priority) -> Self{
                        #context_name {}
                    }
                }

                /// internal spawn function for task
                pub fn #spawn_name() -> Result<(),()>{
                    let input = ();
                    unsafe {
                        if let Some(index) 
                            = rtic::export::interrupt::free(|_| {
                            (&mut *#task_f_queue.get_mut()).dequeue()}) 
                            {  

                                //__rtic_internal_(whatever_task)_INPUTS

                                rtic::export::interrupt::free(|_| {
                                    (&mut *#dispatcher_queue.get_mut())
                                    .enqueue_unchecked((#enum_name_task_same_prio::#name, index));
                                });
                    
                                rtic::pend(#device::interrupt::#interrupt);
                                Ok(())
                        }else{
                            Err(input)
                        }
                    }   
                }
            });

            // Feels like a hack but I would need to change the parsers if I want this
            // to work in a better way.
            init_tasks.push(quote!{
                (0..1u8).for_each(|i| {
                    (&mut *#task_f_queue.get_mut()).enqueue_unchecked(i)
                });
            });
        }

        
        let dispatcher_prio = format!("dispatcher_{:}",priority);
        let interrupt_name = util::internal_task_ident(&interrupt, "");
        let interrupt_name_unsafe = util::internal_task_ident(&interrupt, "unsafe");
        



        // Dispatches all software_tasks of priority of a certain priority.
        // The dispatcher is expressed as a hardware task that runs the
        // different software tasks of same prio as functions.
        dispatchers.push(quote!(
            /// The real dispatcher
            #[task(binds = #interrupt, priority = #priority_lit)]
            fn #interrupt_name(_: #interrupt_name::Context){
                #interrupt_name_unsafe();
            }

            fn #interrupt_name_unsafe(){
                unsafe{
                    const PRIORITY: u8 = #priority;
                    rtic::export::run(
                        PRIORITY,
                            || {
                            while let Some((task, index))
                                = (&mut *#dispatcher_queue.get_mut()).split().1.dequeue()
                            {
                                match task {
                                    #(#call_to_software_task)*
                                }
                            }
                        },
                    );
                    }
            }

            /// All software tasks belonging to prio X
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            pub enum #enum_name_task_same_prio {
                #(#same_prio_tasks)*
            }

            /// Implements rtic clone
            #[automatically_derived]
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for #enum_name_task_same_prio {
                #[inline]
                fn clone(&self) -> #enum_name_task_same_prio {
                    *self
                }
            }
            
            /// Implements rtic copy
            #[automatically_derived]
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for #enum_name_task_same_prio {}
            
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            #[allow(non_upper_case_globals)]
            static #dispatcher_queue: rtic::RacyCell<#dispatcher_request_queue> = rtic::RacyCell::new(
                rtic::export::Queue::new(),
            );
        ));

    }

    if dispatchers.len() == 0{
        dispatchers.push(quote!());
    }
    if software_tasks.len() == 0{
        software_tasks.push(quote!());
    }
    if tasks_overhead.len() == 0{
        tasks_overhead.push(quote!());
    }

    let init_software = quote!{
        fn init_software(){
            unsafe{
                #(#init_tasks)*
            }
        }
    };

    (dispatchers, software_tasks, tasks_overhead, init_software)
}


