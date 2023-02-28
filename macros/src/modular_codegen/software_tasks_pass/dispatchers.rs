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

use super::{software_tasks,sw_names};


use syn::{Ident, LitInt, Path};




/// Creates dispatchers for each priority and binds each dispatcher to given 
/// interrupt. The dispatchers will work as hardware tasks.
/// 
/// Every software task is bund to corresponding dispatcher with same prio.
/// The dispatcher will then take care of running the right task.
/// 
/// The overhead that makes calls like "foo::spawn().unwrap();" possible. 
/// 
/// TODO: local and shared resources
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

    let device = &extra.device;
    let mut software_tasks = vec![];
    let mut dispatchers = vec![];
    let mut interrupts = vec![];

    interrupts.extend(&app.args.extern_interrupts);
    
    let mut init_tasks = vec![];

    for (priority,tasks) in priority_to_tasks{
        let interrupt = interrupts.pop().unwrap().0; 
        
        // Named the dispatcher after the interrupt it was assigned. 
        let dispatcher_name = sw_names::dispatcher_variable( &interrupt.to_string());

        // Used to be able to access all tasks with the same dispatcher
        let mut match_spawn_software_task = vec![];
        let mut dispatcher_tasks_access = vec![];
        let dispatcher_tasks_name = sw_names::dispatcher_variable(&format!("for_priority_{priority}"));
        
        // Request queue holds the order of requested task. So they
        // can be dispatched in order.
        let dispatcher_request_queue = sw_names::dispatcher_variable(&format!("request_queue_{priority}"));

        let mut contexts = vec![];

        // Capacity for the dispatcher queue.
        // Needs to be the capacity of all capacity of all tasks
        // capacity + 1.
        let mut capacity_usize = 1;

        for (name, task) in tasks{
            
            capacity_usize += task.args.capacity;

            // The dispatcher needs to be able to access all tasks of it's
            // priority.
            dispatcher_tasks_access.push(quote!(#name,));


            // See tasks.rs
            let (allocate_software_task_queue,
                bind_spawn_to_software_task,
                software_task,
                task_context) = 
                    software_tasks::generate_software_task(
                        name,
                        task,
                        app.pass_modules.get(name),
                        &dispatcher_tasks_name,
                        &dispatcher_request_queue,
                        device,
                        interrupt,
                    );

            init_tasks.push(allocate_software_task_queue);
            match_spawn_software_task.push(bind_spawn_to_software_task);
            software_tasks.push(software_task);

            contexts.extend(task_context);
        }

        

        
        let priority_lit = LitInt::new(&format!("{}",&priority),Span::call_site());
        let capacity = LitInt::new(&format!("{}",capacity_usize), Span::call_site());
        let dispatcher_request_queue_size = quote!(rtic::export::SCRQ<#dispatcher_tasks_name,#capacity>);

        // Dispatches all software_tasks of priority of a certain priority.
        // The dispatcher is expressed as a hardware task that runs the
        // different software tasks of same prio as functions.
        dispatchers.push(quote!{
            #[allow(unsafe_code)]
            #[task(binds = #interrupt, priority = #priority_lit)]
            fn #dispatcher_name(_: #dispatcher_name::Context){
                unsafe{
                    const PRIORITY: u8 = #priority;
                    rtic::export::run(
                        PRIORITY,
                            || {
                            while let Some((task, index))
                                = (&mut *#dispatcher_request_queue.get_mut()).split().1.dequeue()
                            {
                                match task {
                                    #(#match_spawn_software_task)*
                                }
                            }
                        },
                    );
                }
            }

            /// Context needed to pass local and shared resources to their respective task.
            #(#contexts)*

            /// All software tasks belonging to prio X
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            pub enum #dispatcher_tasks_name {
                #(#dispatcher_tasks_access)*
            }

            /// Implements rtic clone
            #[automatically_derived]
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for #dispatcher_tasks_name {
                #[inline]
                fn clone(&self) -> #dispatcher_tasks_name {
                    *self
                }
            }
            
            /// Implements rtic copy
            #[automatically_derived]
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for #dispatcher_tasks_name {}
            
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            #[allow(non_upper_case_globals)]
            static #dispatcher_request_queue: rtic::RacyCell<#dispatcher_request_queue_size> = rtic::RacyCell::new(
                rtic::export::Queue::new(),
            );
        });
    }

    let init_software;

    // There are no software tasks hence no code needs to be generated
    if software_tasks.len() == 0{
        dispatchers.push(quote!());
        software_tasks.push(quote!());
        init_software = quote!();
    }
    else
    {
        init_software = quote!{
            #(#init_tasks)*
        };
    }

    (dispatchers, software_tasks, init_software)
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

