#![allow(unused_imports)]
#![allow(dead_code)]
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro::TokenStream;
use quote::quote;
use rtic_syntax::ast::App;
use std::fs;


use syn::{Attribute, Ident, LitInt, PatType};

use crate::{analyze::Analysis, check::Extra, codegen::app};


mod assertions;
mod hardware_tasks;
mod idle;
mod init;
mod local_resources_struct;
mod local_resources;
mod module;
mod shared_resources_struct;
mod shared_resources;
mod util;

pub fn new_codegen(
    app: &App, 
    analysis: &Analysis,
    extra: &Extra,
) -> TokenStream2 {
    let mut mod_app = vec![];
    let mut main = vec![];
    let mut root = vec![];
    let mut user = vec![];

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
    let (mod_app_shared_resources, mod_shared_resources) =
        shared_resources::codegen(app, analysis, extra);
    let (mod_app_local_resources, mod_local_resources) =
        local_resources::codegen(app, analysis, extra);

    user.push(quote!(
        #user_init

        #user_idle
    ));

    root.push(quote!(
        #(#root_init)*

        #(#root_idle)*
    ));

    mod_app.push(quote!(
        #mod_app_init

        #(#mod_app_idle)*
    ));

    let main_name = util::suffixed("main");
    main.push(quote!(
        #[doc(hidden)]
        mod rtic_ext {
            use super::*;
            #[no_mangle]
            unsafe extern "C" fn #main_name() -> ! {
                #(#assertion_stmts)*

                #main_init

                #call_idle
            }
        }
    ));

    let output = quote!(
        /// The RTIC application module
        pub mod #name {
            /// Always include the device crate which contains the vector table
            use #device as #rt_err;

            // #monotonics Not needed yet

            #(#user_imports)*

            /// User code from within the module
            #(#user_code)*
            /// User code end

            #(#user)*
            
            
            ///
            ///#user_hardware_tasks
            /// ||||
            /// \/\/ 
            #(#user_hardware_tasks)*
            
            /// /\/\ 
            /// ||||
            /// #user_hardware_tasks
            ///
            

            #(#root)*

            #mod_shared_resources

            #mod_local_resources

            #(#root_hardware_tasks)*

            /// app module
            #(#mod_app)*

            #(#mod_app_shared_resources)*

            #(#mod_app_local_resources)*

            #(#mod_app_hardware_tasks)*

            #(#main)*
        }
    );

    output
}

