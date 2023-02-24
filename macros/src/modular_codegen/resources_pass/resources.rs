

use std::task;
use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;
use quote::quote;
use rtic_syntax::{ast::{App, SharedResources, LocalResources}, Context};

use crate::{analyze::Analysis, check::Extra};


use super::r_names;
use super::context;
use super::local_resources_struct;
use super::local_resources;
use super::shared_resources_struct;
use super::shared_resources;


/// Collects all the context and resource structs for
/// init, idle, software tasks and hardware tasks.
pub fn codegen(
    app: &App,
    analysis: &Analysis,
    extra: &Extra
    ) ->(
    // context -- the contexts and modules
    Vec<TokenStream2>,
    // structs -- the resource structs
    Vec<TokenStream2>
    ) {
    let mut contexts = vec![];
    let mut structs = vec![];
    let mut local_life_time = false;
    let mut shared_life_time = false;

    // init resources
    // init can't have shared resources.
    let context = Context::Init;
    // let name = &app.init.name;
    let name = &app.init.name;
    let has_local = app.init.args.local_resources.len() > 0;
    let has_shared = false;

    if has_local{
        let (item, constructor) = local_resources_struct::codegen(name, context, &mut local_life_time, app);
        structs.push(quote!(#item #constructor));
    };

    contexts.push(context::codegen( 
        name, 
        has_local, 
        &local_life_time, 
        has_shared, 
        &shared_life_time,
        true
    ));


    // idle resources
    if let Some(idle) = &app.idle{
        let context = Context::Idle;
        let name = &idle.name;
        let has_local = idle.args.local_resources.len() > 0;
        let has_shared = idle.args.shared_resources.len() > 0;

        if has_local{
            let (item, constructor) = local_resources_struct::codegen(name, context, &mut local_life_time, app);
            structs.push(quote!(#item #constructor));
        };

        if has_shared{
            let (item, constructor) = shared_resources_struct::codegen(context,  &mut shared_life_time, app);
            structs.push(quote!(#item #constructor));
        }

        contexts.push(context::codegen( 
            name, 
            has_local, 
            &local_life_time, 
            has_shared, 
            &shared_life_time, 
            false
        ));

    }

    // hardware tasks resources
    for (name, task) in &app.hardware_tasks{
        let context = Context::HardwareTask(name);
        let has_local = task.args.local_resources.len() > 0;
        let has_shared = task.args.shared_resources.len() > 0;

        if has_local{
            let (item, constructor) = local_resources_struct::codegen(name, context, &mut local_life_time, app);
            structs.push(quote!(#item #constructor));
        };

        if has_shared{
            let (item, constructor) = shared_resources_struct::codegen(context,  &mut shared_life_time, app);
            structs.push(quote!(#item #constructor));
        }
        
        contexts.push(context::codegen( 
            name, 
            has_local, 
            &local_life_time, 
            has_shared, 
            &shared_life_time, 
            false
        ));

    }

    // software tasks resources
    for (name, task) in &app.software_tasks{
        let context = Context::SoftwareTask(name);
        let has_local = task.args.local_resources.len() > 0;
        let has_shared = task.args.shared_resources.len() > 0;

        if has_local{
            let (item, constructor) = local_resources_struct::codegen(name, context, &mut local_life_time, app);
            structs.push(quote!(#item #constructor));
        };

        if has_shared{
            let (item, constructor) = shared_resources_struct::codegen(context,  &mut shared_life_time, app);
            structs.push(quote!(#item #constructor));
        }

        contexts.push(context::codegen( 
            name, 
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


    (contexts, structs)
}


