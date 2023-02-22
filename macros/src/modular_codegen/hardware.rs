
use proc_macro2::TokenStream as TokenStream2;
// use proc_macro::TokenStream;
use quote::quote;
use rtic_syntax::ast::App;
// use std::fs;


// use syn::{Attribute, Ident, LitInt, PatType};

use crate::{analyze::Analysis, check::Extra, codegen::util};

mod assertions;
mod hardware_tasks;
mod idle;
mod init;
mod module;
mod shared_resources_struct;
mod shared_resources;

pub fn codegen(
    app: &App, 
    analysis: &Analysis,
    extra: &Extra,
) -> (
    // Returns the argument needed for rtic_syntax::parse()
    TokenStream2,
    TokenStream2) {

    if !app.software_tasks.is_empty(){
        panic!("the hardware pass can't handle software tasks. Try adding \"software\" to compiler_passes");
    }
    
    let mut main = vec![];

    let user_imports = &app.user_imports;
    let user_code = &app.user_code;
    let name = &app.name;
    let device = &extra.device;

    let rt_err = util::rt_err_ident();

    let assertion_stmts = assertions::codegen(app, analysis, extra);
    let (mod_app_hardware_tasks, 
        root_hardware_tasks, 
        user_hardware_tasks) = 
        hardware_tasks::codegen(app, analysis, extra);
    let (
        mod_app_init, 
        root_init, 
        user_init, 
        main_init
        ) = init::codegen(app, analysis, extra);
    let (
        mod_app_idle, 
        root_idle, 
        user_idle, 
        call_idle
        ) = idle::codegen(app, analysis, extra);
    
    //Feels like these are called from all modules...
    // let (mod_app_shared_resources, mod_shared_resources) =
    //     shared_resources::codegen(app, analysis, extra);
    // let (mod_app_local_resources, mod_local_resources) =
    //     local_resources::codegen(app, analysis, extra);
    

    
    let passes_init = &app.main_fn;

    let mut modules = vec![]; 
    for (i,t) in &app.task_modules{
        let items = &t.items;
        let module = quote!{
            pub mod #i{
                #(#items)*
            }
        };
        modules.push(module);
    }

    let main_name = util::suffixed("main");
    main.push(quote!(
        #[doc(hidden)]
        mod rtic_ext {
            use super::*;
            #[no_mangle]
            unsafe extern "C" fn #main_name() -> ! {
                #(#assertion_stmts)*

                #(#passes_init)*

                #main_init 

                #call_idle 
            }
        }
    ));

    let generated_code = quote!(
        /// The RTIC application module
        pub mod #name {
            /// Always include the device crate which contains the vector table
            use #device as #rt_err;

            // #monotonics Not needed yet

            /// #user_imports
            #(#user_imports)*

            /// #user_code 
            #(#user_code)*

            /// #user_init
            #user_init

            /// #user_idle
            #user_idle
            
            /// #user_hardware_tasks
            #(#user_hardware_tasks)*
            
            /// #root_init
            #(#root_init)*

            /// #root_idle
            #(#root_idle)*

            // /// #mod_shared_resources
            // #mod_shared_resources

            // /// #mod_local_resources
            // #mod_local_resources

            /// #root_hardware_tasks
            #(#root_hardware_tasks)*

            /// #mod_app_init
            #mod_app_init

            /// #mod_app_idle
            #(#mod_app_idle)*

            // /// #mod_app_shared_resources
            // #(#mod_app_shared_resources)*

            // /// #mod_app_local_resources
            // #(#mod_app_local_resources)*

            /// #mod_app_hardware_tasks
            #(#mod_app_hardware_tasks)*

            #(#modules)*

            /// #main
            #(#main)*
        }
    );

    let generated_argument = quote!(
        // This is equal to the rtic macro:
        // #[rtic::app(device = #device)]
        device = #device
    );

    (generated_argument,generated_code)
}

