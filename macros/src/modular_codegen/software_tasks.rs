

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{ast::App, analyze::Priority};
use std::fs;

use std::str::FromStr;
use crate::{analyze::Analysis, check::Extra};

use super::hardware;

use crate::codegen::util;

mod dispatchers;
mod tasks;
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

    // Untouched in software.
    let name = &app.name;
    let device = &extra.device;
    let user_imports = &app.user_imports;
    let user_code = &app.user_code;

    let (dispatchers, 
        software_tasks, 
        init_software) = dispatchers::codegen(app, extra);

    let user_init = codegen_init(app);

    // user_idle, hardware_tasks and resources are untouched
    let user_idle = codegen_idle(app);
    let hardware_tasks = codegen_hardware(app);
    let resources = codegen_resources(app);
    
    // creates the argument used in the rtic parser
    let argument = quote!(
        // This is equal to the rtic macro:
        // #[rtic::app(device = #device)]
        device = #device
    );

    let code = quote!(
        mod #name{


            /// #user_imports
            #(#user_imports)*

            /// #dispatchers
            #(#dispatchers)*

            /// #software_tasks
            #(#software_tasks)*

            /// #user_init
            #user_init

            /// #init_software
            #init_software

            /// #user_idle
            #user_idle

            /// #user_code
            #(#user_code)*

            /// #hardware_tasks
            #(#hardware_tasks)*

            /// #resources
            #resources
        }
    );

    (argument, code)
}



/// Adds a call to software_init() if there are any software tasks. 
fn codegen_init(app:&App) -> TokenStream2{
    let init = &app.init;
    let name = &init.name;
    let context = &init.context;
    let attrs = &init.attrs;
    let stmts = &init.stmts;
    let shared = &init.user_shared_struct;
    let local = &init.user_local_struct;

    let user_init_return = quote! {#shared, #local, #name::Monotonics};

    // Software tasks needs to be initialized before anything else happens.
    let init_software_call; 
    if app.software_tasks.is_empty(){
        init_software_call = quote!();
    }else{
        init_software_call = quote!(init_software(););
    };


    quote!(
        #(#attrs)*
        #[init]
        fn #name(#context: #name::Context) -> (#user_init_return) {
            /// From software tasks
            #init_software_call
            #(#stmts)*
        }
    )
}

/// Recreates the idle task so they can be parsed again
fn codegen_idle(app:&App) -> Option<TokenStream2>{

    if let Some(idle) = &app.idle{
        let name = &idle.name;
        let attrs = &idle.attrs;
        let context = &idle.context;
        let stmts = &idle.stmts;
        Some(quote!(
            #(#attrs)*
            #[allow(non_snake_case)]
            fn #name(#context: #name::Context) -> ! {
                #(#stmts)*
            }
        ))
    }else{
        None
    }

    
}

/// Recreates the hardware tasks so they can be parsed again
fn codegen_hardware(app: &App) -> Vec<TokenStream2>{
    
    let mut hw_tasks = vec![];
    
    for (name, task) in &app.hardware_tasks{

        let attrs = &task.attrs;
        let binds = &task.args.binds;
        let context = &task.context;
        let stmts = &task.stmts;

        // Transforms suffix literal to unsuffixed literal
        // and can there for be put as a priority value 
        let priority = util::priority_literal(&task.args.priority);

        // let test = TokenStream::from_str(&format!("{:08b}", value)).unwrap();
        // println!("{:?}",format!("{:?}",test));

        hw_tasks.push(quote!{

            #(#attrs)*
            #[task(binds = #binds, priority = #priority)]
            fn #name(#context: #name::Context) {
                #(#stmts)*
            }

        });
    }


    hw_tasks
}

/// Recreates the resources so they can be parsed again
fn codegen_resources(app: &App) -> TokenStream2 {

    let mut local = vec![];

    for (name, local_r) in &app.local_resources{
        let ty = &local_r.ty;
        local.push(quote!(#name: #ty,))
    }

    let mut shared = vec![];

    for (name, shared_r) in &app.shared_resources{
        let ty = &shared_r.ty;
        shared.push(quote!(#name: #ty,))
    }


    quote!(
        #[local]
        struct Local{#(#local)*}


        #[shared]
        struct Shared{#(#shared)*}
    )
}
