use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use rtic_syntax::{ast::App, Context};

use crate::modular_codegen::{
    analyze::Analysis,
    check::Extra,
};

use syn::{Attribute, Ident};


use super::{
    local_resources_struct, 
    module, 
    shared_resources_struct, 
    local_resources
};

/// Generates support code for `#[idle]` functions
pub fn codegen(
    app: &App,
    analysis: &Analysis,
    extra: &Extra,
) -> (
    // mod_app_idle -- the `${idle}Resources` constructor
    Vec<TokenStream2>,
    // root_idle -- items that must be placed in the root of the crate:
    // - the `${idle}Locals` struct
    // - the `${idle}Resources` struct
    // - the `${idle}` module, which contains types like `${idle}::Context`
    Vec<TokenStream2>,
    // user_idle
    Option<TokenStream2>,
    // call_idle
    TokenStream2,
) {
    if let Some(idle) = &app.idle {
        let mut shared_needs_lt = false;
        let mut local_needs_lt = false;
        let mut mod_app = vec![];
        let mut root_idle = vec![];

        let name = &idle.name;

        if !idle.args.shared_resources.is_empty() {
            let (item, constructor) =
                shared_resources_struct::codegen_original(Context::Idle, &mut shared_needs_lt, app);

            root_idle.push(item);
            mod_app.push(constructor);
        }

        if !idle.args.local_resources.is_empty() {
            let (item, constructor) =
                local_resources_struct::codegen_original(Context::Idle, &mut local_needs_lt, app);

            root_idle.push(item);

            mod_app.push(constructor);
        }

        module::codegen_original(
            "idle",
            true,
            false,
            false,
            Context::Idle,
            shared_needs_lt,
            local_needs_lt,
            app,
            analysis,
            extra,
        );

        
        //items - outside of the module.
        let mut items = vec![];
        //module_items - Things that are put in the function module
        let mut module_items = vec![];
        //fields - builds the execution context struct.
        let mut fields: Vec<TokenStream2> = vec![];
        //values - the implementation of execution context.
        let mut values: Vec<TokenStream2> = vec![];
        // Used to copy task cfgs to the whole module
        let task_cfgs: Vec<TokenStream2> = vec![];
        
        let mut lt = None;
        if !idle.args.local_resources.is_empty(){
            println!("ja,jo jag körs allt");
            let (module_item, field, value, lt_return) 
                = local_resources::codegen_module(name,local_needs_lt);
            module_items.push(module_item);
            fields.push(field);
            values.push(value);
            lt = lt_return;
        }

        let doc = "idle loop";
        let core:Option<TokenStream2> = None;
        let priority = Some(quote!(priority: &#lt rtic::export::Priority));

        let internal_context_name = Ident::new(&format!("__rtic_idle_{}_context", name), Span::call_site());

        let cfgs: Vec<Attribute> = vec![];

        items.push(quote!(
        ));

        module_items.push(quote!(
        ));
        

        root_idle.push(quote!(
            #(#items)*

            #(#cfgs)*
            /// Execution context
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            pub struct #internal_context_name<#lt> {
                #(#fields,)*
            }

            #(#cfgs)*
            impl<#lt> #internal_context_name<#lt> {
                #[inline(always)]
                pub unsafe fn new(#core #priority) -> Self {
                    #internal_context_name {
                        #(#values,)*
                    }
                }
            }

            #[allow(non_snake_case)]
            #(#task_cfgs)*
            #[doc = #doc]
            pub mod #name {
                #(#cfgs)*
                pub use super::#internal_context_name as Context;
                #(#module_items)*
            }
        ));


        let attrs = &idle.attrs;
        let context = &idle.context;
        let stmts = &idle.stmts;
        let user_idle = Some(quote!(
            #(#attrs)*
            #[allow(non_snake_case)]
            fn #name(#context: #name::Context) -> ! {
                use rtic::Mutex as _;
                use rtic::mutex::prelude::*;

                #(#stmts)*
            }
        ));

        let call_idle = quote!(#name(
            #name::Context::new(&rtic::export::Priority::new(0))
        ));

        (mod_app, root_idle, user_idle, call_idle)
    } else {
        (
            vec![],
            vec![],
            None,
            quote!(loop {
                rtic::export::nop()
            }),
        )
    }
}
