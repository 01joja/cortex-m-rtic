use proc_macro2::{Span,TokenStream as TokenStream2};
use quote::quote;
use rtic_syntax::{ast::App, Context, analyze::Priority, ast::SoftwareTask};

use std::str::FromStr;
use std::collections::HashMap;

use crate::{
    analyze::Analysis,
    check::Extra,
    codegen::util,
};


use syn::{Ident, LitInt, Path};

const RTIC_SW_TASK: &str = "__rtic_software_task";
const RTIC_DISPATCHER: &str = "__rtic_dispatcher";


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
    extra: &Extra,
) -> (
    // Dispatchers
    Vec<TokenStream2>,
    // Software_tasks - Code generated for the software tasks
    Vec<TokenStream2>,
    // fn init_software - allocates memory for the function queues
    TokenStream2){

    // hashmap, key: priority, value: vector of tasks.
    let priority_to_tasks = sort_tasks_after_priority(&app);

    //TODO - better names and structure.

    let device = &extra.device;
    let mut software_tasks = vec![];
    let mut dispatchers = vec![];
    let mut interrupts = vec![];

    interrupts.extend(&app.args.extern_interrupts);
    
    let mut init_tasks = vec![];

    for (priority,tasks) in priority_to_tasks{
        let interrupt = interrupts.pop().unwrap().0; 
        

        let mut match_spawn_software_task = vec![];
        let mut same_prio_tasks_vector = vec![];

        let same_prio_tasks = Ident::new(&format!("__{}_with_priority_{}",RTIC_SW_TASK, &priority), Span::call_site());
        
        let name_queue = format!("dispatcher_queue_{}",priority);
        let dispatcher_queue = Ident::new(&format!("{}_{}", RTIC_DISPATCHER, name_queue), Span::call_site());

        let capacity_usize = tasks.len() + 1;

        for (name, task) in tasks{
            
            // The dispatcher needs to be able to access all tasks of it's
            // priority.
            same_prio_tasks_vector.push(quote!(#name,));

            let (allocate_software_task_queue,
                bind_spawn_to_software_task,
                software_task) = 
                    generate_software_task(
                        name,
                        task,
                        &same_prio_tasks,
                        &dispatcher_queue,
                        device,
                        interrupt
                    );

            init_tasks.push(allocate_software_task_queue);
            match_spawn_software_task.push(bind_spawn_to_software_task);
            software_tasks.push(software_task);
        }
        
        let priority_lit = LitInt::new(&format!("{}",&priority),Span::call_site());
        let interrupt_name = util::internal_task_ident(&interrupt, "");
        let interrupt_name_unsafe = util::internal_task_ident(&interrupt, "unsafe");
        
        let capacity = LitInt::new(&format!("{}",capacity_usize), Span::call_site());
        let dispatcher_request_queue = quote!(rtic::export::SCRQ<#same_prio_tasks,#capacity>);

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
                                    #(#match_spawn_software_task)*
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
            pub enum #same_prio_tasks {
                #(#same_prio_tasks_vector)*
            }

            /// Implements rtic clone
            #[automatically_derived]
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for #same_prio_tasks {
                #[inline]
                fn clone(&self) -> #same_prio_tasks {
                    *self
                }
            }
            
            /// Implements rtic copy
            #[automatically_derived]
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for #same_prio_tasks {}
            
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

    let init_software = quote!{
        fn init_software(){
            unsafe{
                #(#init_tasks)*
            }
        }
    };

    (dispatchers, software_tasks, init_software)
}

/// Generates following code for the software task: 
/// initialization of the 
fn generate_software_task(
    name: &Ident, 
    task: &SoftwareTask,
    same_prio_tasks: &Ident,
    dispatcher_queue: &Ident,
    device: &Path,
    interrupt: &Ident
    ) -> (
    // initialization
    TokenStream2,
    // 
    TokenStream2,
    TokenStream2
    ){
    let attrs = &task.attrs;
    let cfgs = &task.cfgs;
    let stmts  = &task.stmts;

    // Create free queues and inputs / instants buffers
    let function_queue =  Ident::new(&format!("{}_{}_function_queue",RTIC_SW_TASK, name), Span::call_site());

    // Allocates memory for the software functions queues during
    // initialization.
    let allocate_software_task_queue = quote!{
        (0..1u8).for_each(|i| {
            (&mut *#function_queue.get_mut()).enqueue_unchecked(i)
        });
    };

    // Binds foo::spawn().unwrap(); to function foo in the dispatcher
    // match statement.
    let bind_spawn_to_software_task = quote!{
        #same_prio_tasks::#name =>{
            // let () = message passing?
            (&mut *#function_queue.get_mut()).split().0.enqueue_unchecked(index);
            let priority = &rtic::export::Priority::new(PRIORITY);
            #name(#name::Context::new(priority))
        }
    };

    // This transforms the software task to a function that the dispatcher
    // can call.
    let software_task_as_function = quote!{
        /// Software task as a function
        #(#attrs)*
        #(#cfgs)*
        #[allow(non_snake_case)]
        fn #name(_: #name::Context){
            use rtic::Mutex as _;
            use rtic::mutex::prelude::*;
            #(#stmts)*
        }  
    };

    let context_name = Ident::new(&format!("{}_{}_context",RTIC_SW_TASK, name), Span::call_site());
    let spawn_name = Ident::new(&format!("{}_{}_spawn",RTIC_SW_TASK, name), Span::call_site());
    
    let capacity = task.args.capacity as usize + 1;
    let capacity_literal = LitInt::new(&capacity.to_string(), Span::call_site());

    // Removed mk_uninit because monotonic is not implemented yet
    #[allow(clippy::redundant_closure)]
    let function_queue_capacity = quote!(rtic::export::SCFQ<#capacity_literal>);

    // The software overhead needed for the task.
    let task_overhead = quote!{

        /// Queue version of a free-list that keeps track of empty slots in
        /// the following buffers
        #[allow(non_camel_case_types)]
        #[allow(non_upper_case_globals)]
        #[doc(hidden)]
        static #function_queue: rtic::RacyCell<#function_queue_capacity> = rtic::RacyCell::new(
            rtic::export::Queue::new(),
        );

        /// Binds internal task overhead to the user defined task.
        pub mod #name {
            pub use super::#context_name as Context;
            pub use super::#spawn_name as spawn;
        }

        /// internal task context (only priority for now)
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
                    (&mut *#function_queue.get_mut()).dequeue()}) 
                    {  

                        //__rtic_internal_(whatever_task)_INPUTS

                        rtic::export::interrupt::free(|_| {
                            (&mut *#dispatcher_queue.get_mut())
                            .enqueue_unchecked((#same_prio_tasks::#name, index));
                        });
            
                        rtic::pend(#device::interrupt::#interrupt);
                        Ok(())
                }else{
                    Err(input)
                }
            }   
        }
    };

    // The function and overhead is the software task
    let software_task = quote!{
        #software_task_as_function
        #task_overhead
    };


    return (
        allocate_software_task_queue,
        bind_spawn_to_software_task,
        software_task)
}



/// Returns a HashMap where the key is a priority and
/// value is vector of all software tasks with that priority.
fn sort_tasks_after_priority(app: &App) -> HashMap<u8, Vec<(&Ident, &SoftwareTask)>>{
    let mut priority_to_tasks: HashMap<u8, Vec<(&Ident, &SoftwareTask)>> = HashMap::new();
    
    for task in &app.software_tasks{
        let priority = &task.1.args.priority;
        
        match priority_to_tasks.get_mut(priority){
            Some(vec) => {
                // push to vector in the hashmap
                vec.push(task);
            },
            None => {
                // new entry, create new vector with task.
                // let mut vec = vec![task];
                let vec = vec![task];
                priority_to_tasks.insert(priority.clone(), vec);
            },
        };
    }

    return priority_to_tasks;
}

