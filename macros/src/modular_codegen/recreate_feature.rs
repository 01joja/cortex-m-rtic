
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use rtic_syntax::ast::{App, SharedResources, LocalResources, PassModule, TaskLocal};

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
pub fn hardware_tasks(app: &App, skip_resources: bool) -> (
    // Functions
    Vec<TokenStream2>,
    // Modules
    Vec<TokenStream2>
){
    
    let mut hw_tasks = vec![];
    let mut hw_modules = vec![];
    
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

        if let Some(module) = app.pass_modules.get(name){
            let has_context = &module.has_context;
            let has_monotonic = &module.has_monotonic;
            let items = &module.items;

            hw_modules.push(quote!{
                #[__rtic_pass_module(has_context = #has_context, has_monotonic = #has_monotonic)]
                pub mod #name{
                    #(#items)*
                }

            })
        }

    }
    (hw_tasks,hw_modules)
}

/// Recreates the software tasks
/// 
/// #[task(priority = <>, local_resources = vec<>, shared_resources = vec<>)]
/// 
/// if skip_resources == true, it skips local_resources and shared_resources
pub fn software_tasks(app: &App, skip_resources: bool) -> (
    // Functions
    Vec<TokenStream2>,
    // Modules
    Vec<TokenStream2>
){
    
    let mut sw_tasks = vec![];
    let mut sw_modules = vec![];
    
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

        if let Some(module) = app.pass_modules.get(name){
            let has_context = &module.has_context;
            let has_monotonic = &module.has_monotonic;
            let items = &module.items;

            sw_modules.push(quote!{
                #[__rtic_pass_module(has_context = #has_context, has_monotonic = #has_monotonic)]
                pub mod #name{
                    #(#items)*
                }

            })
        }
    }
    
    (sw_tasks,sw_modules)
}


/// Recreates idle
/// 
/// #[idle]
/// 
/// fn idle(cx: idle::Context){...}
/// 
/// if skip_resources == true, it skips local_resources and shared_resources
pub fn idle(app:&App, skip_resources: bool) -> (
    // function
    Option<TokenStream2>,
    // module
    Option<TokenStream2>
){  

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

        let idle_func = Some(quote!{
            #(#attrs)*
            #[allow(non_snake_case)]
            #[idle(#(#resources)*)]
            fn #name(#context: #name::Context) -> ! {
                #(#stmts)*
            }
        });

        let mut idle_module = None;
        if let Some(module) = app.pass_modules.get(name){
            let has_context = &module.has_context;
            let has_monotonic = &module.has_monotonic;
            let items = &module.items;

            idle_module = Some(quote!{
                #[__rtic_pass_module(has_context = #has_context, has_monotonic = #has_monotonic)]
                pub mod #name{
                    #(#items)*
                }
            })
        }
        (idle_func,idle_module)
    }else{
        (None,None)
    }  
}

/// Recreates init
/// 
/// #[init]
/// 
/// fn init(cx: init::Context){...}
/// 
/// if skip_resources == true, it skips local_resources and shared_resources
pub fn init(app:&App, skip_resources: bool) -> (
    // function
    TokenStream2,
    // module
    Option<TokenStream2>,
){

    

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

    let init_func = quote!{
        #(#attrs)*
        #[init(#(#resources)*)]
        fn #name(#context: #name::Context) -> (#user_init_return) {
            #(#stmts)*
        }
    };

    let mut init_module = None;
    if let Some(module) = app.pass_modules.get(name){
        let has_context = &module.has_context;
        let has_monotonic = &module.has_monotonic;
        let items = &module.items;

        init_module = Some(quote!{
            #[__rtic_pass_module(has_context = #has_context, has_monotonic = #has_monotonic)]
            pub mod #name{
                #(#items)*
            }

        })
    }

    (init_func,init_module)
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
    let mut locals = vec![];

    if !local_resources.is_empty(){
        
        for (ident,task_local) in local_resources{

            match task_local{
                TaskLocal::External => {
                    locals.push(quote!(#ident, ))
                },
                TaskLocal::Declared(local) => {
                    let the_type = &local.ty;
                    let expression = &local.expr;
                    locals.push(quote!(#ident: #the_type = #expression,)) 
                },
                _ => todo!(),
            };


        }

        // resource.push(quote!(local = [#(#locals)*]));
    }

    
    let mut shared_tokens = vec![];
    if let Some(shared) = shared_resources{
        if !shared.is_empty(){

            for local in shared{
                let ident = local.0;
                shared_tokens.push(quote!(#ident, ))
            }

            // resource.push(quote!(shared = [#(#shared_tokens)*]));
        }
    }

    if !locals.is_empty() && !shared_tokens.is_empty(){
        resource.push(quote!(local = [#(#locals)*] , shared = [#(#shared_tokens)*]));
    }
    else if !locals.is_empty(){
        resource.push(quote!(local = [#(#locals)*]));
    }
    else if !shared_tokens.is_empty(){
        resource.push(quote!(shared = [#(#shared_tokens)*]));
    } 


    resource

}

