use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use rtic_syntax::{ast::App, Context};

use crate::modular_codegen::{
    analyze::Analysis,
    check::Extra,
};

use syn::{Attribute, Ident};


use super::{
    module, 
};

/// Generates support code for `#[idle]` functions
pub fn codegen(
    app: &App,
    _analysis: &Analysis,
    _extra: &Extra,
) -> (
    // mod_app_idle -- the `${idle}Resources` constructor
    Vec<TokenStream2>,
    // module -- idles module
    Option<TokenStream2>,
    // user_idle
    Option<TokenStream2>,
    // call_idle
    TokenStream2,
) {
    if let Some(idle) = &app.idle {
        let mod_app = vec![];
        let mut module_idle = None;

        let attrs = &idle.attrs;
        let context = &idle.context;
        let stmts = &idle.stmts;
        let name = &idle.name;
        
        let doc = "idle loop";


        let mut context_call_to_idle = None;
        let mut context_function = None;

        if let Some(module) = app.pass_modules.get(name){
            let item = &module.items;

            module_idle = Some(quote!{
                    #[allow(non_snake_case)]
                    #[doc = #doc]
                    pub mod #name {
                        #(#item)*
                    }
                }
            );

            if module.has_context{
                context_call_to_idle = Some(quote!{
                    #name::Context::new(&rtic::export::Priority::new(0))
                });
                context_function = Some(quote!{
                    #context: #name::Context
                })
            }

        };
        
        let user_idle = Some(quote!(
            #(#attrs)*
            #[allow(non_snake_case)]
            fn #name(#context_function) -> ! {
                use rtic::Mutex as _;
                use rtic::mutex::prelude::*;

                #(#stmts)*
            }
        ));

        let call_idle = quote!{
            #name(#context_call_to_idle)
        };

        (mod_app, module_idle, user_idle, call_idle)
    } else {
        (
            vec![],
            None,
            None,
            quote!(loop {
                rtic::export::nop()
            }),
        )
    }
}
