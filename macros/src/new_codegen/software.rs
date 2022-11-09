

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{ast::App, analyze::Priority};
use std::fs;

use std::str::FromStr;
use crate::{analyze::Analysis, check::Extra};

use super::hardware;

use crate::codegen::util;



pub fn codegen(
    app: &App, 
    _analysis: &Analysis,
    extra: &Extra,
) -> (
    // Returns the argument needed for rtic_syntax::parse()
    TokenStream2,
    
    // The generated code where all software tasks have 
    // been transformed and expressed as hardware tasks.
    // After this rtic_syntax::parse should not find any
    // software tasks.
    TokenStream2) {

    let name = &app.name;
    let device = &extra.device;
    let user_imports = &app.user_imports;
    let user_code = &app.user_code;

    let user_init = codegen_init(app);
    let user_idle = codegen_idle(app);
       
    let argument = quote!(
        // This is equal to the rtic macro:
        // #[rtic::app(device = #device)]
        device = #device
    );

    let hardware_task = codegen_hardware(app);

    let resources = codegen_resources(app);
    
    let code = quote!(
        mod #name{


            #resources


            #(#user_imports)*

            #user_init

            #user_idle

            #(#user_code)*

            #(#hardware_task)*

        }

    );

    println!("hej hej hej2{:?}", format!("{:?}", argument));

    (argument, code)
}



fn codegen_init(app:&App) -> TokenStream2{

    let init = &app.init;
    let name = &init.name;
    let context = &init.context;
    let attrs = &init.attrs;
    let stmts = &init.stmts;
    let shared = &init.user_shared_struct;
    let local = &init.user_local_struct;

    let user_init_return = quote! {#shared, #local, #name::Monotonics};

    // let mut a_vector = vec![];
    // for i in stmts{
    //     a_vector.push(format!("{:?}",i));
    // }
    // println!("{:?}",a_vector);

    quote!(
        #(#attrs)*
        #[init]
        fn #name(#context: #name::Context) -> (#user_init_return) {
            #(#stmts)*
        }
    )
}

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

fn codegen_hardware(app: &App) -> Vec<TokenStream2>{
    let mut hw_tasks = vec![];

    
    for (name, task) in &app.hardware_tasks{

        let attrs = &task.attrs;
        let binds = &task.args.binds;
        let context = &task.context;
        let stmts = &task.stmts;

        // Transforms suffix literal to unsuffixed literal
        // and can there for be put as a priority value
        let priority = format!("{:?}",&task.args.priority);
        let priority = TokenStream2::from_str(priority.as_str()).unwrap();      

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
