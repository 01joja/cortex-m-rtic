
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use rtic_syntax::ast::{App, SharedResources, LocalResources};

use crate::check::Extra;
use super::tokens;

/// recreates the argument created from the macro:
/// 
/// #[rtic::app(device = lm3s6965, dispatchers = ...)]
pub fn argument(app: &App, extra: &Extra) -> TokenStream2{    
    let device = &extra.device;
    
    // extracts 
    let interrupts = &app.args.extern_interrupts;
    let mut dispatchers = vec![];
    for (ident, _) in interrupts{
        dispatchers.push(quote!(#ident,));
    }
     
    quote!(
        device = #device,
        dispatchers = [#(#dispatchers)*]
    )
}

/// Recreates the hardware tasks
/// 
/// #[task(binds = <>, priority = <>, local_resources = vec<>, shared_resources = vec<>)]
/// 
/// if skip_resources == true, it skips local_resources and shared_resources
pub fn hardware_tasks(app: &App, skip_resources: bool) -> Vec<TokenStream2>{
    
    let mut hw_tasks = vec![];
    
    for (name, task) in &app.hardware_tasks{

        let attrs = &task.attrs;
        let binds = &task.args.binds;
        let context = &task.context;
        let stmts = &task.stmts;


        // Transforms suffix literal to unsuffixed literal
        // and can there for be put as a priority value 
        let priority = tokens::literal_int(format!("{}",&task.args.priority));

        let resources = if skip_resources {
            vec![quote!()]
        } else {
            resources(&task.args.local_resources,Some(&task.args.shared_resources))
        };

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

/// Recreates the software tasks
/// 
/// #[task(priority = <>, local_resources = vec<>, shared_resources = vec<>)]
/// 
/// if skip_resources == true, it skips local_resources and shared_resources
pub fn software_tasks(app: &App, skip_resources: bool) -> Vec<TokenStream2>{
    
    let mut sw_tasks = vec![];
    
    for (name, task) in &app.software_tasks{

        let attrs = &task.attrs;
        let context = &task.context;
        let stmts = &task.stmts;


        // Transforms suffix literal to unsuffixed literal
        // and can there for be put as a priority value 
        let priority = tokens::literal_int(format!("{}",&task.args.priority));
        
        let resources = if skip_resources {
            vec![quote!()]
        } else {
            resources(&task.args.local_resources,Some(&task.args.shared_resources))
        };

        sw_tasks.push(quote!{

            #(#attrs)*
            #[task(priority = #priority, #(#resources)*)]
            fn #name(#context: #name::Context) {
                #(#stmts)*
            }

        });
    }
    sw_tasks
}


/// Recreates idle
/// 
/// #[idle]
/// 
/// fn idle(cx: idle::Context){...}
/// 
/// if skip_resources == true, it skips local_resources and shared_resources
pub fn idle(app:&App, skip_resources: bool) -> Option<TokenStream2>{

    if let Some(idle) = &app.idle{
        let name = &idle.name;
        let attrs = &idle.attrs;
        let context = &idle.context;
        let stmts = &idle.stmts;

        let resources = if skip_resources {
            vec![quote!()]
        } else {
            resources(&idle.args.local_resources,Some(&idle.args.shared_resources))
        };

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
/// 
/// #[init]
/// 
/// fn init(cx: init::Context){...}
/// 
/// if skip_resources == true, it skips local_resources and shared_resources
pub fn init(app:&App, skip_resources: bool) -> TokenStream2{
    let init = &app.init;
    let name = &init.name;
    let context = &init.context;
    let attrs = &init.attrs;
    let stmts = &init.stmts;
    let shared = &init.user_shared_struct;
    let local = &init.user_local_struct;

    let user_init_return = quote! {#shared, #local, #name::Monotonics};

    

    let resources = if skip_resources {
        vec![quote!()]
    } else {
        resources(&init.args.local_resources,None)
    };

    quote!(
        #(#attrs)*
        #[init(#(#resources)*)]
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
/// 
/// Option<&SharedResources> is needed for init.
fn resources(
    local_resources: &LocalResources,
    shared_resources: Option<&SharedResources>
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


    if let Some(shared) = shared_resources{
        if !shared.is_empty(){
            let mut shared_tokens = vec![];

            for local in shared{
                let ident = local.0;
                shared_tokens.push(quote!(#ident, ))
            }

            resource.push(quote!(shared = [#(#shared_tokens)*]));
        }
    }
    resource

}



