

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

mod local_resources_struct;
mod local_resources;
mod shared_resources_struct;
mod shared_resources;
mod r_names;
mod resources;
mod context;

pub fn codegen(
    app: &App,
    analysis: &Analysis,
    extra: &Extra,
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
    let user_init = recreate_feature::init(app, true);
    let user_idle = recreate_feature::idle(app, true);
    let hardware_tasks = recreate_feature::hardware_tasks(app, true);
    let software_tasks = recreate_feature::software_tasks(app, true);
    let resources_user_structs = recreate_feature::resources_structs(app);

    
    // creates the argument used in the rtic parser
    let argument = recreate_feature::argument(app, extra);

    let (contexts, structs,) = resources::codegen(app, analysis, extra);

    let code = quote!{
        mod #name{

            //#(#local_resources)*

            #(#contexts)*

            #(#structs)*

            #resources_user_structs
            
            #(#user_imports)*
            
            #user_init
            
            #user_idle
            
            #(#user_code)*
            
            #(#hardware_tasks)*

            #(#software_tasks)*

            #[__rtic_main]
            fn __rtic_main(){
            }
        }
    };

    (argument, code)
}




