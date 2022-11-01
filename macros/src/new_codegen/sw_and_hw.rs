#![allow(unused_imports)]
#![allow(dead_code)]
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro::TokenStream;
use quote::quote;
use rtic_syntax::ast::App;
use std::fs;


use syn::{Attribute, Ident, LitInt, PatType};

use crate::{analyze::Analysis, check::Extra, codegen::app};


mod hardware_tasks;
mod util;


pub fn new_codegen(
    app: &App, 
    analysis: &Analysis,
    extra: &Extra,
) -> TokenStream2 {


    let name = &app.name;
    let device = &extra.device;
    let user_imports = &app.user_imports;
    let user_code = &app.user_code;
    let _shared = &app.shared_resources;
    let _local = &app.local_resources;
    
    // A long name, don't really understand what it dose.
    let rt_err = rt_err_ident();


    
    let init = &app.init;
    let _init_attrs = &init.attrs;
    let init_stmts = &init.stmts;
    let init_name = &init.name;
    let init_context = &init.context;
    let init_shared = &init.user_shared_struct;
    let init_local = &init.user_local_struct;

    let _sw = &app.software_tasks;


    //let mut user_tasks_names = vec![];
    //Taken form software_tasks.rs
    let mut user_tasks = vec![];
    let mut modules = vec![];
    for (name, task) in &app.software_tasks {
        let inputs = &task.inputs;
        let context = &task.context;
        let attrs = &task.attrs;
        let cfgs = &task.cfgs;
        let stmts = &task.stmts;
        user_tasks.push(quote!(
            #(#attrs)*
            #(#cfgs)*
            #[allow(non_snake_case)]
            fn #name(#context: #name::Context #(,#inputs)*) {
                use rtic::Mutex as _;
                use rtic::mutex::prelude::*;

                #(#stmts)*
            }
        ));
        let module_name = internal_task_ident(&name, "module");
        modules.push(module_name);
    }



    
    let _temp = format!("{:#?}",modules);
    //fs::write("contents/temp.rs", temp);

    //main (or mains as it is called in codegen...)
    let main = suffixed("main");
    //pre_init call to util::interrupt_ident();

    let interrupt = interrupt_ident();
    let mut interrupt_vec = vec![]; 
    for interrupt_name in app.args.extern_interrupts.keys() {
        interrupt_vec.push(quote!(let _ = #rt_err::#interrupt::#interrupt_name;));
    }

    let _nvic_prio_bits = quote!(#device::NVIC_PRIO_BITS);


    // might need to create a loop that can handle multiple internal contexts.
    
    let internal_context;

    if modules.len() > 0{
        internal_context = &modules[0];
    }else{
        let name = suffixed("foo");
        let module_name = internal_task_ident(&name, "module");
        modules.push(module_name);
        internal_context  = &modules[0];
    }

    let (mod_app, root, user_tasks) 
            = hardware_tasks::codegen(app, analysis, extra);



    let output = quote!(
        pub mod #name{
            use #device as #rt_err;
            //use #device::Interrupt;

            #(#user_imports)*


            #(#user_code)*

            fn #init_name(#init_context: #init_name::Context) -> (Shared, Local, #init_name::Monotonics) {
                #(#init_stmts)*
            }

            #(#user_tasks)*

            
            //Needs to change
            struct #init_shared {}
            struct #init_local {}
            
            /// Monotonics used by the system
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            pub struct __rtic_internal_Monotonics();
            /// Execution context
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            pub struct __rtic_internal_init_Context<'a> {
                /// Core (Cortex-M) peripherals
                pub core: rtic::export::Peripherals,
                /// Device peripherals
                pub device: #device::Peripherals,
                /// Critical section token for init
                pub cs: rtic::export::CriticalSection<'a>,
            }
            impl<'a> __rtic_internal_init_Context<'a> {
                #[inline(always)]
                pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
                    __rtic_internal_init_Context {
                        device: #device::Peripherals::steal(),
                        cs: rtic::export::CriticalSection::new(),
                        core,
                    }
                }
            }
            #[allow(non_snake_case)]
            ///Initialization function
            pub mod init {
                pub use super::__rtic_internal_Monotonics as Monotonics;
                pub use super::__rtic_internal_init_Context as Context;
            }
            /// Execution context
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            pub struct #internal_context {}
            impl #internal_context {
                #[inline(always)]
                pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
                    #internal_context {}
                }
            }



            
            //#(#mains)*
            #[doc(hidden)]
            mod rtic_ext {
                use super::*;
                #[no_mangle]
                unsafe extern "C" fn #main() -> ! {
                    //#(#assertion_stmts)*

                    // pre_init.rs
                    // #(#pre_init_stmts)*
                    // Disable interrupts -- `init` must run with interrupts disabled
                    rtic::export::interrupt::disable();
                    // To set the variable in cortex_m so the peripherals cannot be taken multiple times
                    let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal().into();
                    
                    #(#interrupt_vec)*

                    //here comes the magic form hardware tasks. I need to look in to it but
                    // not today... Sounds like a monday problem


                    #[inline(never)]
                    fn __rtic_init_resources<F>(f: F) where F: FnOnce() {
                        f();
                    }

                    // Wrap late_init_stmts in a function to ensure that stack space is reclaimed.
                    __rtic_init_resources(||{
                        //#call_init
                        let (shared_resources, local_resources, mut monotonics) = #init_name(#init_name::Context::new(core.into()));
                        
                        // no monotonic, local resources or shared resources
                        //#(#post_init_stmts)*
                        rtic::export::interrupt::enable();
                        //so... do we need
                        //use #device::Interrupt;
                        //????
                    });

                    /// #call_idle on idle loop right now
                    loop {
                        rtic::export::nop()
                    }
                }
            }

        }
    );

    output
}




// these functions are from util.rs

const RTIC_INTERNAL: &str = "__rtic_internal";

pub fn interrupt_ident() -> Ident {
    let span = Span::call_site();
    Ident::new("interrupt", span)
}


/// The name to get better RT flag errors
pub fn rt_err_ident() -> Ident {
    Ident::new(
        "you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml",
        Span::call_site(),
    )
}

fn suffixed(name: &str) -> Ident {
    let span = Span::call_site();
    Ident::new(name, span)
}

/// Generate an internal identifier for tasks
pub fn internal_task_ident(task: &Ident, ident_name: &str) -> Ident {
    mark_internal_name(&format!("{}_{}", task, ident_name))
}

/// Mark a name as internal
pub fn mark_internal_name(name: &str) -> Ident {
    Ident::new(&format!("{}_{}", RTIC_INTERNAL, name), Span::call_site())
}
