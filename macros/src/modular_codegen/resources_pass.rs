

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{
    ast::{App, SharedResource, SharedResources, LocalResources}, 
    analyze::Priority};
use std::fs;

use std::str::FromStr;
use crate::{analyze::Analysis, check::Extra};

use super::hardware;

use super::recreate_feature;


use crate::codegen::util;

pub fn codegen(
    app: &App, 
    extra: &Extra
) -> (
    // Returns the argument needed for rtic_syntax::parse()
    TokenStream2,
    
    // The generated code where all software tasks have 
    // been transformed and expressed as hardware tasks.
    // After this rtic_syntax::parse should not find any
    // software tasks.
    TokenStream2) {

    //    let priority = util::priority_literal(&task.args.priority);
    // Untouched in software.
    let name = &app.name;
    let user_imports = &app.user_imports;
    let user_code = &app.user_code;

    // recreates features for later passes 
    let user_init = recreate_feature::init(app);
    let user_idle = recreate_feature::idle(app);
    let hardware_tasks = recreate_feature::hardware(app);
    let resources = recreate_feature::resources_structs(app);
    
    let (dispatchers, 
        software_tasks, 
        init_software) = (quote!(),quote!(),quote!());

    
    
    // creates the argument used in the rtic parser
    let argument = recreate_feature::argument(extra);

    let code = quote!(
        mod #name{


            /// #user_imports
            #(#user_imports)*

            /// #dispatchers
            #dispatchers

            /// #software_tasks
            #software_tasks

            /// #user_init
            #user_init

            /// #user_idle
            #user_idle

            /// #user_code
            #(#user_code)*

            /// #hardware_tasks
            #(#hardware_tasks)*

            /// #resources
            #resources


            #[__rtic_main]
            fn __rtic_main(){
                #init_software
            }
        }
    );

    (argument, code)
}




