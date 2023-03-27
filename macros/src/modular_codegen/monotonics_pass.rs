

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{
    ast::{App}, 
    analyze::Priority};
use std::fs;

use crate::{analyze::Analysis, check::Extra};

use self::m_names::timer_queue;

use super::recreate_feature;

mod init;
mod monotonics_modules;
mod spawn_handler;
mod timer_queue;
mod m_names;

pub fn codegen(
    app: &App, 
    analyze: &Analysis,
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
    let (user_init, _) = recreate_feature::init(app, false);
    let (user_idle, idle_module) = recreate_feature::idle(app, false);
    let (hardware_tasks, hardware_modules) = recreate_feature::hardware_tasks(app, false);
    let resources_structs = recreate_feature::resources_structs(app);
    
    let (spawn_handlers, mut software_modules) = spawn_handler::codegen(app, extra);
    
    let software_tasks;

    if software_modules.is_empty(){
        let software = recreate_feature::software_tasks(app, false);
        software_tasks = software.0;
        software_modules.extend(software.1)
    } else {
        let software = recreate_feature::software_tasks(app, false);
        software_tasks = software.0;
    }

    let (
        pre_init, 
        post_init
    ) = init::codegen(app, extra);
    let (init_module, monotonics_module) = monotonics_modules::codegen(app, extra);
    
    let timer_queue = timer_queue::codegen(app, analyze, extra);

    // creates the argument used in the rtic parser
    let argument = recreate_feature::argument(app, extra);

    // previous passes
    let mut passes_pre_init = &vec![];
    let mut passes_post_init = &vec![];
    if let Some(main_fn) = &app.main_fn{
        passes_pre_init = &main_fn.pre_init;
        passes_post_init = &main_fn.post_init;
    }

    // // Needed to add allow camlecase to "MyMono"
    // let mut new_user_code = vec![];
    // for c in user_code{

    //     new_user_code.push(#)

    // };

    let code = quote!(
        mod #name{

            #resources_structs
            
            #(#user_imports)*
            #(#user_code)*
            
            #user_init

            #init_module

            #idle_module
            
            #user_idle
            
            #(#hardware_tasks)*

            #(#hardware_modules)*

            #(#software_tasks)*

            #(#software_modules)*

            #monotonics_module

            #(#spawn_handlers)*

            #(#timer_queue)*
            
            #[__rtic_main]
            fn __rtic_main(){
                #(#pre_init)*
                #(#passes_pre_init)*
                #[__post_init]
                fn post_init(){
                    #(#post_init)*
                    #(#passes_post_init)*
                }
            }
        }
    );

    (argument, code)
}

