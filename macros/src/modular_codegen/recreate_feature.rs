
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use rtic_syntax::ast::{App, SharedResources, LocalResources};

use crate::check::Extra;
use super::tokens;

// recreates the argument created from the macro:
// #[rtic::app(device = #device, ...)]
pub fn argument(extra: &Extra) -> TokenStream2{    
    let device = &extra.device;
    quote!(
        device = #device
    )
}

/// Recreates the hardware tasks
/// #[task(binds = <>, priority = <>, local_resources = vec<>, local_resources = vec<>)]
pub fn hardware(app: &App) -> Vec<TokenStream2>{
    
    let mut hw_tasks = vec![];
    
    for (name, task) in &app.hardware_tasks{

        let attrs = &task.attrs;
        let binds = &task.args.binds;
        let context = &task.context;
        let stmts = &task.stmts;


        // Transforms suffix literal to unsuffixed literal
        // and can there for be put as a priority value 
        let priority = tokens::literal_int(format!("{}",&task.args.priority));

        let resources = resources(
            &task.args.local_resources,
            &task.args.shared_resources,
        );

        hw_tasks.push(quote!{

            #(#attrs)*
            #[task(binds = #binds, priority = #priority, #(#resources)*)]
            fn #name(#context: #name::Context) {
                #(#stmts)*
            }

        });
    }
    hw_tasks
}


/// Recreates idle
pub fn idle(app:&App) -> Option<TokenStream2>{

    if let Some(idle) = &app.idle{
        let name = &idle.name;
        let attrs = &idle.attrs;
        let context = &idle.context;
        let stmts = &idle.stmts;

        let resources = resources(
            &idle.args.local_resources,
            &idle.args.shared_resources,
        );

        println!("{:?}",resources);

        Some(quote!(
            #(#attrs)*
            #[allow(non_snake_case)]
            #[idle(#(#resources)*)]
            fn #name(#context: #name::Context) -> ! {
                #(#stmts)*
            }
        ))
    }else{
        None
    }  
}

/// Recreates init
pub fn init(app:&App) -> TokenStream2{
    let init = &app.init;
    let name = &init.name;
    let context = &init.context;
    let attrs = &init.attrs;
    let stmts = &init.stmts;
    let shared = &init.user_shared_struct;
    let local = &init.user_local_struct;

    let user_init_return = quote! {#shared, #local, #name::Monotonics};

    quote!(
        #(#attrs)*
        #[init]
        fn #name(#context: #name::Context) -> (#user_init_return) {
            #(#stmts)*
        }
    )
}

/// Recreates the resources structs
pub fn resources_structs(app: &App) -> TokenStream2 {

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


/// regenerates a vec of shared and local resource for tasks, idle etcetera
/// ex:
/// vec! = ["local = [regenerated1,regenerated2]", "shared = [regenerated3]"]
fn resources(
    local_resources: &LocalResources,
    shared_resources: &SharedResources
) -> Vec<TokenStream2>{
    let mut resource = vec![];
    if !local_resources.is_empty(){
        let mut locals = vec![];
        for local in local_resources{
            let ident = local.0;
            locals.push(quote!(#ident, ))
        }
        resource.push(quote!(local = [#(#locals)*]));
    }
    if !shared_resources.is_empty(){
        let mut shareds = vec![];
        for local in shared_resources{
            let ident = local.0;
            shareds.push(quote!(#ident, ))
        }
        resource.push(quote!(shared = [#(#shareds)*]));
    }
    resource

}



