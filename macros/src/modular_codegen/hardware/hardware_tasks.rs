use std::sync::Arc;

use proc_macro2::{TokenStream as TokenStream2, Span};
use quote::quote;
use rtic_syntax::{ast::{App, HardwareTask}, Context};

use syn::{Attribute, Ident, LitInt, PatType};

use crate::modular_codegen::{
    analyze::Analysis,
    check::Extra,
};

use super::{
    module,
    shared_resources_struct,
    shared_resources,
};


/// Generate support code for hardware tasks (`#[exception]`s and `#[interrupt]`s)
pub fn codegen(
    app: &App, 
    analysis: &Analysis,
    extra: &Extra,
) -> (
    // mod_app_hardware_tasks -- interrupt handlers and `${task}Resources` constructors
    Vec<TokenStream2>,
    // root_hardware_tasks -- items that must be placed in the root of the crate:
    // - `${task}Locals` structs
    // - `${task}Resources` structs
    // - `${task}` modules
    Vec<TokenStream2>,
    // user_hardware_tasks -- the `#[task]` functions written by the user
    Vec<TokenStream2>,
) {

    let mut mod_app = vec![];
    let mut root = vec![];
    let mut user_tasks = vec![];

    for (name, task) in &app.hardware_tasks{
        let _symbol = task.args.binds.clone();
        let _priority = task.args.priority;
        let _cfgs = &task.cfgs;
        let _attrs = &task.attrs;
        
        mod_app.push(config_priority(name,task));

        let mut shared_needs_lt = false;
        let mut local_needs_lt = false;

        // // `${task}Locals`
        // if !task.args.local_resources.is_empty() {
        //     let (item, constructor) = local_resources_struct::codegen_original(
        //         Context::HardwareTask(name),
        //         &mut local_needs_lt,
        //         app,
        //     );

        //     root.push(item);

        //     mod_app.push(constructor);
        // }
        

        // `${task}Resources`
        if !task.args.shared_resources.is_empty() {
            let (item, constructor) = shared_resources_struct::codegen_original(
                Context::HardwareTask(name),
                &mut shared_needs_lt,
                app,
            );

            root.push(item);

            mod_app.push(constructor);
        }

        module::codegen_original(
            "hardware",
            false,
            false,
            true,
            Context::HardwareTask(name),
            shared_needs_lt,
            local_needs_lt,
            app,
            analysis,
            extra
        );

        let has_local_resources = !task.args.local_resources.is_empty();
        let has_shared_resources = !task.args.shared_resources.is_empty();

        // Fixes the specific modules for hardware.
        let module = module_func(
            name,
            has_shared_resources,
            shared_needs_lt,
            has_local_resources,
            local_needs_lt,
            app,
            analysis,
            extra
        );
        // println!("module: \n{:?}\nmodlue2: \n{:?}", module,module2);
        root.push(module);

        user_tasks.push(user_task(name,task));

    }

    

    (mod_app, root, user_tasks)
}


// handles user_task
fn user_task(name: &Ident,task: &HardwareTask,) -> TokenStream2{

    if !task.is_extern {
        let attrs = &task.attrs;
        let context = &task.context;
        let stmts = &task.stmts;
        quote!(
            #(#attrs)*
            #[allow(non_snake_case)]
            fn #name(#context: #name::Context) {
                use rtic::Mutex as _;
                use rtic::mutex::prelude::*;

                #(#stmts)*
            }
        )
    }else{
        quote!()
    }
}

// adds a function that handles the priority of tasks.
fn config_priority(name: &Ident,task: &HardwareTask,) -> TokenStream2{
    let attrs = &task.attrs;
    let cfgs = &task.cfgs;
    let symbol = task.args.binds.clone();
    let priority = task.args.priority;
    quote!(
        #[allow(non_snake_case)]
        #[no_mangle]
        #(#attrs)*
        #(#cfgs)*
        unsafe fn #symbol() {
            const PRIORITY: u8 = #priority;
            rtic::export::run(PRIORITY, || {
                #name(
                    #name::Context::new(&rtic::export::Priority::new(PRIORITY))
                )
            });
        }
    )
}

fn module_func(
    name: &Ident,
    has_shared_resources:bool,
    shared_resources_tick:bool,
    has_local_resources:bool,
    local_resources_tick:bool, 
    app: &App, 

    _analysis: &Analysis,
    _extra: &Extra,
    ) -> TokenStream2{

    // items - items outside of the module.
    let mut items = vec![];
    // module_items - don't understand. Think it is functions in called function?.
    // it will be inside "pub mod #name"
    let mut module_items = vec![];
    // fields - builds the execution context struct.
    let mut fields: Vec<TokenStream2> = vec![];
    // values - the implementation of execution context.
    let mut values: Vec<TokenStream2> = vec![];
    // Used to copy task cfgs to the whole module
    // Don't think this will be needed here. It is only used in software.
    let task_cfgs: Vec<Attribute> = vec![];

    // Module 005
    // could be together with call to local_resources_struct, maybe...
    let mut lt = None;
    // if has_local_resources{
    //     let (module_item, field, value, lt_return) 
    //         = local_resources::codegen_module(name,local_resources_tick);
    //     module_items.push(module_item);
    //     fields.push(field);
    //     values.push(value);
    //     lt = lt_return;
    // }

    // Module 006
    if has_shared_resources{
        let (module_item, field, value, lt_return) 
            = shared_resources::codegen_module(name,shared_resources_tick);
        module_items.push(module_item);
        fields.push(field);
        values.push(value);
        lt = lt_return;
    }


    // Module 010
    let doc = "Hardware task";

    // Module 012
    let cfgs = &app.hardware_tasks[name].cfgs;

    // Module 015
    let core: Option<TokenStream2> = None;

    // Module 017
    let priority = quote!(priority: &#lt rtic::export::Priority);

    
    
    // Module 018
    let internal_context_name = Ident::new(&format!("__rtic_idle_{}_context", name), Span::call_site());
    items.push(quote!(
        #(#cfgs)*
        /// Execution context
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        pub struct #internal_context_name<#lt> {
            #(#fields,)*
        }

        #(#cfgs)*
        impl<#lt> #internal_context_name<#lt> {
            #[inline(always)]
            pub unsafe fn new(#core #priority) -> Self {
                #internal_context_name {
                    #(#values,)*
                }
            }
        }
    ));

    module_items.push(quote!(
        #(#cfgs)*
        pub use super::#internal_context_name as Context;
    ));

    // Module 020 and 021
    if items.is_empty() {
        return quote!()
    } else {
        return quote!(
            #(#items)*

            #[allow(non_snake_case)]
            #(#task_cfgs)*
            #[doc = #doc]
            pub mod #name {
                #(#module_items)*
            }
        )
    }
}
