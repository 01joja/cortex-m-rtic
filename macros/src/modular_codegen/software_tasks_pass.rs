

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

mod dispatchers;
mod software_tasks;
mod sw_names;

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
    let (user_init, init_module) = recreate_feature::init(app, false);
    let (user_idle, idle_module) = recreate_feature::idle(app, false);
    let (hardware_tasks, hardware_module) = recreate_feature::hardware_tasks(app, false);
    let resources_structs = recreate_feature::resources_structs(app);

    let (dispatchers, 
        software_tasks, 
        init_software) = dispatchers::codegen(app, extra);

    let mut pre_init = vec![];
    let mut post_init = vec![];
    if let Some(main_fn) = &app.main_fn{
        pre_init.extend(&main_fn.pre_init);
        post_init.extend(&main_fn.post_init);
    }
        
    // creates the argument used in the rtic parser
    let argument = recreate_argument(extra);

    let code = quote!(
        mod #name{

            #resources_structs

            #(#user_imports)*
            
            #user_init
            
            #init_module

            #user_idle

            #idle_module
            
            #(#user_code)*

            #(#dispatchers)*
            
            #(#software_tasks)*
            
            #(#hardware_tasks)*

            #(#hardware_module)*
            
            #[__rtic_main]
            fn __rtic_main(){
                #init_software
                #(#pre_init)*
                #[__post_init]
                fn post_init(){
                    #(#post_init)*
                }
            }
        }
    );

    (argument, code)
}

/// Removes dispatcher/external interrupts from
/// argument so they don't cause issues later in
/// the pipeline.
pub fn recreate_argument(extra: &Extra) -> TokenStream2{    
    let device = &extra.device;

    quote!(
        device = #device,
    )
}


