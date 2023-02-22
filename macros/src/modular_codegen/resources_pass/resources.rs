

use std::task;
use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;
use quote::quote;
use rtic_syntax::{ast::{App, SharedResources, LocalResources}, Context};

use crate::{analyze::Analysis, check::Extra};


use super::r_names;
use super::local_resources_struct;
use super::local_resources;
use super::shared_resources_struct;
use super::shared_resources;


/// Generates `local` variables and local resource proxies
///
/// I.e. the `static` variables and theirs proxies.
pub fn codegen(
    app: &App,
    analysis: &Analysis,
    extra: &Extra
    ) ->(
    // context -- the context of that task.
    Vec<TokenStream2>,
    // structs -- the resource structs.
    Vec<TokenStream2>
    ) {
    let mut contexts = vec![];
    let mut structs = vec![];
    let mut local_life_time = false;
    let mut shared_life_time = false;

    // init resources
    let context = Context::Init;
    let name = &app.init.name;

    let (item, constructor) = local_resources_struct::codegen(name, context, &mut local_life_time, app);
    structs.push(quote!(#item #constructor));
    
    let has_local = app.init.args.local_resources.len() > 0;
    let has_shared = false;

    contexts.push(gen_context( name, has_local, &local_life_time, has_shared, &shared_life_time,true));


    // idle resources
    if let Some(idle) = &app.idle{
        let context = Context::Idle;
        let name = &idle.name;

        let (item, constructor) = local_resources_struct::codegen(name, context, &mut local_life_time, app);
        structs.push(quote!(#item #constructor));

        let (item, constructor) = shared_resources_struct::codegen(context, &mut shared_life_time, app);
        structs.push(quote!(#item #constructor));
        
        let has_local = idle.args.local_resources.len() > 0;
        let has_shared = idle.args.shared_resources.len() > 0;
    contexts.push(gen_context( name, has_local, &local_life_time, has_shared, &shared_life_time, false));

    }

    // hardware tasks resources
    for (name, task) in &app.hardware_tasks{
        let context = Context::HardwareTask(name);

        let (item, constructor) = local_resources_struct::codegen(name, context, &mut local_life_time, app);
        structs.push(quote!(#item #constructor ));

        let (item, constructor) = shared_resources_struct::codegen(context, &mut shared_life_time, app);
        structs.push(quote!(#item #constructor));
        
        let has_local = task.args.local_resources.len() > 0;
        let has_shared = task.args.shared_resources.len() > 0;
        contexts.push(gen_context( name, has_local, &local_life_time, has_shared, &shared_life_time, false));

    }

    // software tasks resources
    for (name, task) in &app.software_tasks{
        let context = Context::SoftwareTask(name);

        let (item, constructor) = local_resources_struct::codegen(name, context, &mut local_life_time, app);
        structs.push(quote!(#item #constructor));
        
        let (item, constructor) = shared_resources_struct::codegen(context,  &mut shared_life_time, app);
        structs.push(quote!(#item #constructor));
        
        let has_local = task.args.local_resources.len() > 0;
        let has_shared = task.args.shared_resources.len() > 0;
        contexts.push(gen_context( name, has_local, &local_life_time, has_shared, &shared_life_time, false));

    }
    let mod_app_local = local_resources::codegen(app);
    let (mod_app_shared, resources_module) = shared_resources::codegen(app, analysis, extra);
    structs.push(quote!(#(#mod_app_local)* #(#mod_app_shared)* #resources_module));


    (contexts, structs)
}

fn gen_context(
    name: &Ident, 
    has_local: bool,
    local_life_time: &bool,
    has_shared: bool, 
    shared_life_time: &bool,
    init: bool,
    ) -> TokenStream2 {
    
    let mut module = vec![];
    let mut structure = vec![];
    let mut implementation = vec![];
    let mut life_time = None;

    if init{
        life_time = Some(quote!('a));
        structure.push(quote!{
            /// Core (Cortex-M) peripherals
            pub core: rtic::export::Peripherals,
            /// Device peripherals
            pub device: lm3s6965::Peripherals,
            /// Critical section token for init
            pub cs: rtic::export::CriticalSection<'a>,
        });
        implementation.push(quote!{
            device: lm3s6965::Peripherals::steal(),
            cs: rtic::export::CriticalSection::new(),
            core,
        });
    }

    if has_local {
        let struct_local_name = r_names::local_r_struct(name);
        module.push(quote!(
            pub use super::#struct_local_name as LocalResources;
        ));
        if *local_life_time{
            life_time = Some(quote!('a));
            structure.push(quote!(
                pub local: #name::LocalResources<'a>,
            ));
        }else{
            structure.push(quote!(
                pub local: #name::LocalResources,
            ));
        }
        implementation.push(quote!(
            local: #name::LocalResources::new(),
        ));
    }

    if has_shared {
        let struct_shared_name = r_names::shared_r_struct(name);
        module.push(quote!(
            pub use super::#struct_shared_name as SharedResources;
        ));
        if *shared_life_time{
            life_time = Some(quote!('a));
            structure.push(quote!(
                pub shared: #name::SharedResources<'a>,
            ));
        }else{
            structure.push(quote!(
                pub shared: #name::SharedResources,
            ));
        }
        implementation.push(quote!(
            shared: #name::SharedResources::new(),
        ));
    }

    let context_name = r_names::context_name(name);

    quote!(
        #[__rtic_pass_module]
        pub mod #name{
            pub use super::#context_name as Context;
            #(#module)*
        }

        #(#cfgs)*
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        pub struct #context_name<#life_time> {
            #(#structure)*
        }

        #(#cfgs)*
        impl<#life_time> #context_name<#life_time> {
            #[inline(always)]
            pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
                #context_name {
                    #(#implementation)*
                }
            }
        }
    )
}

// /// context for init, it is the most unique so I let it be a septerat one
// fn context_init(app: &App, extra: &Extra, local_life_time: &bool) -> TokenStream2 {

//     let name = &app.init.name;
//     let device = &extra.device;
//     let life_time = if *local_life_time{
//         Some(quote!('a))
//     }else{
//         None
//     };

//     let mut structure = None;
//     let mut implementation = None;
//     if !&app.init.args.local_resources.is_empty(){

//         structure = Some(quote!{
//             pub local: #name::LocalResources<#life_time>,
//         });

//         implementation = Some(quote!{
//             local: #name::LocalResources::new(),
//         });

//     }

//     quote!{
//         #[allow(non_snake_case)]
//         #[allow(non_camel_case_types)]
//         pub struct __rtic_internal_init_Context<life_time> {
            
//             // /// Core (Cortex-M) peripherals
//             // pub core: rtic::export::Peripherals,
//             // /// Device peripherals
//             // pub device: #device::Peripherals,
//             // /// Critical section token for init
//             // pub cs: rtic::export::CriticalSection<life_time>,

//             #structure
//         }
//         impl<'a> __rtic_internal_init_Context<life_time> {
//             #[inline(always)]
//             pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
//                 __rtic_internal_init_Context {
                    
//                     // Init only
//                     // device: #device::Peripherals::steal(),
//                     // cs: rtic::export::CriticalSection::new(),
//                     // core,

//                     #implementation
//                 }
//             }
//         }
//     }
// }

