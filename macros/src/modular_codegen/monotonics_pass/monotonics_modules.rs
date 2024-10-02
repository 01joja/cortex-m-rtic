use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::ast::App;

use crate::modular_codegen::generate_syntax;
use super::m_names;

use crate::{analyze::Analysis, check::Extra, codegen::util};

/// Generates modules for monotonics and init
pub fn codegen(app: &App,  _extra: &Extra) -> (
    // init_module
    //    - adds a struct that is used to pass systicks from init to 
    //      post_init
    TokenStream2,
    // monotonics_module
    //    - holds a implementation of each monotonic.
    TokenStream2
) {

    let init_name = &app.init.name;
    
    // Fetching the items from previous passes.
    let mut init_items = vec![];
    if let Some(module) = app.task_modules.get(init_name){
        init_items.extend(&module.items);
    }

    // Generates the struct used to pass systics form init to post_init
    let monotonics_type: Vec<_> = app
        .monotonics
        .iter()
        .map(|(_, monotonic)| {
            let mono = &monotonic.ty;
            quote! {#mono}
        })
        .collect();

    let internal_monotonics_ident = m_names::monotonic_struct();

    let init_module = quote!{
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        pub struct #internal_monotonics_ident(
            #(pub #monotonics_type),*
        );

        #[__rtic_task_module(has_monotonic = true)]
        pub mod #init_name{
            #(#init_items)*
            pub use super::#internal_monotonics_ident as Monotonics;
        }
    };

    // Generates "pub mod monotonics {...}"
    let monotonic_parts: Vec<_> = app
        .monotonics
        .iter()
        .map(|(name, monotonic)| {
            let name_str = &name.to_string();
            let ident = m_names::monotonic_storage(name);
            let doc = &format!(
                "This module holds the static implementation for `{}::now()`",
                name_str
            );

            let default_monotonic = if monotonic.args.default {
                quote!(pub use #name::now;)
            } else {
                quote!()
            };

            quote! {
                #default_monotonic

                #[doc = #doc]
                #[allow(non_snake_case)]
                pub mod #name {
                    pub fn now() -> <super::super::#name as rtic::Monotonic>::Instant {
                        rtic::export::interrupt::free(|_| {
                            use rtic::Monotonic as _;
                            if let Some(m) = unsafe{ &mut *super::super::#ident.get_mut() } {
                                m.now()
                            } else {
                                <super::super::#name as rtic::Monotonic>::zero()
                            }
                        })
                    }
                }
            }
        })
        .collect();

    let monotonics_module = if monotonic_parts.is_empty() {
        quote!()
    } else {
        quote!(
            pub use rtic::Monotonic as _;
            pub mod monotonics {
                #(#monotonic_parts)*
            }
        )
    };

    (init_module,monotonics_module)
}
