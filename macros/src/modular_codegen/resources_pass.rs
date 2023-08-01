

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{
    ast::{App, SharedResource, SharedResources, LocalResources}, 
    analyze::Priority, 
    Context};
use std::fs;

use std::str::FromStr;
use crate::{analyze::Analysis, check::Extra};

use super::hardware_pass;

use super::recreate_feature;


use crate::codegen::util;

mod local_resources_struct;
mod local_resources;
mod shared_resources_struct;
mod shared_resources;
mod r_names;
mod context;
mod post_init;
mod assertion;


/// Generates the resources, but can't handle late resources.
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
    let (user_init,_) = recreate_feature::init(app, true);
    let (user_idle,_) = recreate_feature::idle(app, true);
    let (hardware_tasks, _) = recreate_feature::hardware_tasks(app, true);
    let (software_tasks,_) = recreate_feature::software_tasks(app, true);
    let resources_user_structs = recreate_feature::resources_structs(app);

    
    // creates the argument used in the rtic parser
    let argument = recreate_feature::argument(app, extra);

    // let (contexts, structs,) = resources::codegen(app, analysis, extra);
    let assertions = assertion::codegen(app, analysis, extra);
    let post_init_resources = post_init::codegen(app, analysis, extra);

    let mut contexts = vec![];
    let mut structs = vec![];
    let mut local_life_time = false;
    let mut shared_life_time = false;

    // init resources and context
    // init can't have shared resources.
    let context = Context::Init;
    let task_name = &app.init.name;
    let has_local = app.init.args.local_resources.len() > 0;
    let has_shared = false;

    if has_local{
        let (item, constructor) = 
            local_resources_struct::codegen(task_name, context, &mut local_life_time, app);
        structs.push(quote!(#item #constructor));
    };

    contexts.push(context::codegen( 
        app,
        task_name, 
        has_local, 
        &local_life_time, 
        has_shared, 
        &shared_life_time,
        true
    ));


    // idle resources and context
    if let Some(idle) = &app.idle{
        let context = Context::Idle;
        let task_name = &idle.name;
        let has_local = idle.args.local_resources.len() > 0;
        let has_shared = idle.args.shared_resources.len() > 0;

        if has_local{
            let (item, constructor) = 
                local_resources_struct::codegen(task_name, context, &mut local_life_time, app);
            structs.push(quote!(#item #constructor));
        };

        if has_shared{
            let (item, constructor) = 
                shared_resources_struct::codegen(task_name, context,  &mut shared_life_time, app);
            structs.push(quote!(#item #constructor));
        }

        contexts.push(context::codegen( 
            app,
            task_name, 
            has_local, 
            &local_life_time, 
            has_shared, 
            &shared_life_time, 
            false
        ));

    }

    // hardware tasks resources and context
    for (task_name, task) in &app.hardware_tasks{
        let context = Context::HardwareTask(task_name);
        let has_local = task.args.local_resources.len() > 0;
        let has_shared = task.args.shared_resources.len() > 0;

        if has_local{
            let (item, constructor) = 
            local_resources_struct::codegen(task_name, context, &mut local_life_time, app);
            structs.push(quote!(#item #constructor));
        };

        if has_shared{
            let (item, constructor) = 
            shared_resources_struct::codegen(task_name, context,  &mut shared_life_time, app);
            structs.push(quote!(#item #constructor));
        }
        
        contexts.push(context::codegen( 
            app,
            task_name, 
            has_local, 
            &local_life_time, 
            has_shared, 
            &shared_life_time, 
            false
        ));

    }

    // software tasks resources and context
    for (task_name, task) in &app.software_tasks{
        let context = Context::SoftwareTask(task_name);
        let has_local = task.args.local_resources.len() > 0;
        let has_shared = task.args.shared_resources.len() > 0;

        if has_local{
            let (item, constructor) = 
            local_resources_struct::codegen(task_name, context, &mut local_life_time, app);
            structs.push(quote!(#item #constructor));
        };

        if has_shared{
            let (item, constructor) = 
            shared_resources_struct::codegen(task_name, context,  &mut shared_life_time, app);
            structs.push(quote!(#item #constructor));
        }

        contexts.push(context::codegen( 
            app,
            task_name, 
            has_local, 
            &local_life_time, 
            has_shared, 
            &shared_life_time, 
            false
        ));

    }
    let mod_app_local = local_resources::codegen(app);
    let (mod_app_shared, resources_module) = shared_resources::codegen(app, analysis, extra);
    structs.push(quote!(#(#mod_app_local)* #(#mod_app_shared)* #resources_module));

    let mut passes_pre_init = &vec![];
    let mut passes_post_init = &vec![];
    if let Some(main_fn) = &app.main_fn{
        passes_pre_init = &main_fn.pre_init;
        passes_post_init = &main_fn.post_init;
    }

    let code = quote!{
        mod #name{

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
                #(#assertions)*
                #(#passes_pre_init)*
                #[__post_init]
                fn post_init(){
                    #(#post_init_resources)*
                    #(#passes_post_init)*
                }
            }
        }
    };

    (argument, code)
}




