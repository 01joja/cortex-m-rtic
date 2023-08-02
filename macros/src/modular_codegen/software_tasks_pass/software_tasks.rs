use proc_macro2::{Span,TokenStream as TokenStream2};
use quote::quote;
use rtic_syntax::{ Context, analyze::Priority, ast::{SoftwareTask,taskModule,App}};

use std::str::FromStr;
use std::collections::HashMap;

use crate::{
    analyze::Analysis,
    check::Extra,
};

use super::sw_names;


use syn::{Ident, LitInt, Path};



/// Generates following code for the software task: 
/// - Allocates the software task queue during init
/// - Match statement to current software_task in the dispatcher.
/// - Software task and the overhead.
/// - "Context", a struct that mimics context in hardware task step (feels like a hack). 
pub fn generate_software_task(
    name: &Ident, 
    task: &SoftwareTask,
    task_module: Option<&taskModule>,
    dispatcher_tasks_name: &Ident,
    dispatcher_request_queue: &Ident,
    device: &Path,
    interrupt: &Ident
    ) -> (
    // initialization
    TokenStream2,
    // match spawn
    TokenStream2,
    // the software task and overhead.
    TokenStream2,
    Vec<TokenStream2>,
    ){
    let attrs = &task.attrs;
    let cfgs = &task.cfgs;
    let stmts  = &task.stmts;

    // Creates internal names
    let request_queue =  sw_names::task_variable(name,"request_queue");
    let input_queue =  sw_names::task_variable(name,"input_queue");
        
    // Allocates memory for the software functions queues during
    // initialization.
    let capacity_u8 = &task.args.capacity;
    let allocate_software_task_queue = quote!{
        (0..#capacity_u8).for_each(|i| {
            (&mut *#request_queue.get_mut()).enqueue_unchecked(i)
        });
    };

    let mut local_resources_struct = vec![];
    let mut local_resources = vec!();
    // Don't know what and how external works. So just throws it away for now.
    for (local_resource, external) in &task.args.local_resources{
        let r_type;
        match external {
            rtic_syntax::ast::TaskLocal::External => todo!(),
            rtic_syntax::ast::TaskLocal::Declared(local) => {
                r_type = &local.ty;
            },
            _ => todo!(),
        }

        local_resources.push(quote!(#local_resource));
        local_resources_struct.push(quote!(pub #local_resource: &'a mut #r_type));
    }

    let mut local_resources_struct = vec![];
    let mut shared_resources = vec![];
    // Need to look in to access. Is it &mut and &? 
    for (shared_resource, _access) in &task.args.shared_resources{
        shared_resources.push(shared_resource.to_string().clone());
        local_resources_struct.push(quote!(pub shared_resource: ));
    }



    // formats message passing variables 
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
    
    // fetches existing items form previous passes and 
    // checks if context is needed.
    let task_context = &task.context;
    let mut module_items = None;
    let mut function_context = None;
    let mut function_call_context = None;
    if let Some(module) = task_module{
        let items = &module.items;
        module_items = Some(
            quote!{
                #(#items)*
            }
        );

        // If module doesn't have any context, we
        // cant use it in call to task or task function.
        if module.has_context{
            function_context = Some(quote!{
                #task_context: #name::Context,
            });
            function_call_context = Some(quote!{
                #name::Context::new(priority)
            });
        }
    }


    // Binds foo::spawn().unwrap(); to function foo in the dispatcher
    // match statement.
    let bind_spawn_to_software_task = quote!{
        #dispatcher_tasks_name::#name =>{
            let (#(#task_messages_names)*) = (&*#input_queue.get())
                .get_unchecked(usize::from(index))
                .as_ptr()
                .read();
            (&mut *#request_queue.get_mut())
                .split()
                .0
                .enqueue_unchecked(index);
            let priority = &rtic::export::Priority::new(PRIORITY);
            #name(#function_call_context #(#task_messages_names)*)
        }
    };

    // This transforms the software task to a function that the dispatcher
    // can call.
    let software_task_as_function = quote!{
        /// Software task as a function
        #(#attrs)*
        #(#cfgs)*
        #[allow(non_snake_case)]
        fn #name(#function_context #(#task_messages)*){
            use rtic::Mutex as _;
            use rtic::mutex::prelude::*;
            #(#stmts)*
        }  
    };

    
    let context = sw_names::task_variable(name, "context");
    let spawn_name = sw_names::task_variable(name, "spawn");
    
    let capacity = task.args.capacity as usize;
    // capacity literal needs to + 1 here
    let capacity_literal = LitInt::new(&(capacity + 1).to_string(), Span::call_site());

    // Removed mk_uninit because monotonic is not implemented yet
    #[allow(clippy::redundant_closure)]
    let function_queue_capacity = quote!(rtic::export::SCFQ<#capacity_literal>);

    // sets it to the right value again
    let capacity_literal = LitInt::new(&(capacity).to_string(), Span::call_site());

    // number of messages passes needs to be equal to capacity
    let mut vec_of_maybe_unit = vec![];
    for _ in 0..task.args.capacity{
        vec_of_maybe_unit.push(quote!(core::mem::MaybeUninit::uninit(),))
    }

    
    let link_section = sw_names::link_seciton(name);

    // The software overhead needed for the task.
    let task_overhead = quote!{

        /// Queue version of a free-list that keeps track of empty slots in
        /// the following buffers
        #[allow(non_camel_case_types)]
        #[allow(non_upper_case_globals)]
        #[doc(hidden)]
        pub static #request_queue: rtic::RacyCell<#function_queue_capacity> = rtic::RacyCell::new(
            rtic::export::Queue::new(),
        );

        

        /// Binds internal task overhead to the user defined task.
        /// Also makes it possible for other passes to modify the 
        /// function_queue and inputs. (Monotonic pass needs them)
        pub mod #name {
            #module_items
            pub use super::#dispatcher_tasks_name as __internal_dispatcher_task_name;
            pub use super::#dispatcher_request_queue as __internal_PRIO_REQUEST_Q;
            pub use super::#request_queue as __internal_function_queue;
            pub use super::#input_queue as __internal_input_queue;
            pub use super::#spawn_name as spawn;
        }

        /// Queue that holds messages for the message passing
        #[link_section = #link_section]
        #[allow(non_camel_case_types)]
        #[allow(non_upper_case_globals)]
        #[doc(hidden)]
        pub static #input_queue: rtic::RacyCell<
            [core::mem::MaybeUninit<(#(#task_messages_types)*)>; #capacity_literal],> = rtic::RacyCell::new([
            #(#vec_of_maybe_unit)*
        ]);

        /// internal task context (only priority for now)
        pub struct #context {}
        impl #context{
            #[inline(always)]
            pub unsafe fn new(priority: &rtic::export::Priority) -> Self{
                #context {}
            }
        }

        /// internal spawn function for task
        pub fn #spawn_name(#(#task_messages_internal)*) -> Result<(),(#(#task_messages_types)*)>{
            let input = (#(#task_messages_names)*);
            unsafe {
                if let Some(index) 
                    = rtic::export::interrupt::free(|_| {
                    (&mut *#request_queue.get_mut()).dequeue()}) 
                    {  
                        (&mut *#input_queue.get_mut())
                            .get_unchecked_mut(usize::from(index))
                            .as_mut_ptr()
                            .write(input);
                        rtic::export::interrupt::free(|_| {
                            (&mut *#dispatcher_request_queue.get_mut())
                            .enqueue_unchecked((#dispatcher_tasks_name::#name, index));
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

    let special_context = vec![quote!()];

    return (
        allocate_software_task_queue,
        bind_spawn_to_software_task,
        software_task,
        special_context,
    );
}
