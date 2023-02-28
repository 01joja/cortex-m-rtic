
use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;
use quote::quote;
use rtic_syntax::{ast::{App, SharedResources, LocalResources}, Context};

// use crate::{analyze::Analysis, check::Extra};

use super::r_names;


///Generates following:
/// - context struct
/// - context implementation
/// - module for task
pub fn codegen(
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
            shared: #name::SharedResources::new(priority),
        ));
    }


    let arguments;
    if init{
        // init needs to have some specific things
        // to be able to configure everything
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
        arguments = quote!(core: rtic::export::Peripherals,);
    }else{
        arguments = quote!(priority: &#life_time rtic::export::Priority);
    }

    let context_name = r_names::context_name(name);

    quote!(

        #[__rtic_pass_module(has_context = true)]
        pub mod #name{
            pub use super::#context_name as Context;
            #(#module)*
        }

        // #(#cfgs)*
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        pub struct #context_name<#life_time> {
            #(#structure)*
        }

        // #(#cfgs)*
        impl<#life_time> #context_name<#life_time> {
            #[inline(always)]
            pub unsafe fn new(#arguments) -> Self {
                #context_name {
                    #(#implementation)*
                }
            }
        }
    )
}

