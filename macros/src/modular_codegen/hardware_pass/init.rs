use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{ast::{App, HardwareTask}, Context};

use syn::{Attribute, Ident, LitInt, PatType};

use crate::modular_codegen::{
    analyze::Analysis,
    check::Extra,
};

use crate::codegen::util;

/// Generate support code for hardware tasks (`#[exception]`s and `#[interrupt]`s)
pub fn codegen(
    app: &App, 
    analysis: &Analysis,
    extra: &Extra,
) -> (
    // module_init -- items that must be placed in the root of the crate:
    // - the `${init}Locals` struct
    // - the `${init}Resources` struct
    // - the `${init}LateResources` struct
    // - the `${init}` module, which contains types like `${init}::Context`
    Vec<TokenStream2>,
    // user_init -- the `#[init]` function written by the user
    TokenStream2,
) {

    let init = &app.init;
    let name = &init.name;

    let mut init_items = vec![];

    let context = &init.context;
    let attrs = &init.attrs;
    let stmts = &init.stmts;
    let shared = &init.user_shared_struct;
    let local = &init.user_local_struct;

    let shared_resources: Vec<_> = app.shared_resources.iter().map(|(k, v)| {
            let ty = &v.ty;
            let cfgs = &v.cfgs;
            let docs = &v.docs;
            quote!(
                #(#cfgs)*
                #(#docs)*
                #k: #ty,
            )
        }).collect();

    let local_resources: Vec<_> = app.local_resources
        .iter().map(|(k, v)| {
            let ty = &v.ty;
            let cfgs = &v.cfgs;
            let docs = &v.docs;
            quote!(
                #(#cfgs)*
                #(#docs)*
                #k: #ty,
            )
        }).collect();

    init_items.push(quote! {
        struct #shared {
            #(#shared_resources)*
        }

        struct #local {
            #(#local_resources)*
        }
    });

    // let locals_pat = locals_pat.iter();

    let user_init_return = quote! {#shared, #local, #name::Monotonics};

    let user_init = quote!(
        #(#attrs)*
        #[inline(always)]
        #[allow(non_snake_case)]
        fn #name(#context: #name::Context) -> (#user_init_return) {
            #(#stmts)*
        }
    );

    let mut module = vec![];
    let mut has_context = false;
    let mut has_monotonic = false;
    if let Some(task_module) = app.task_modules.get(name){
        let items = &task_module.items;
        for i in items{
            module.push(quote!(#i));
        }
        has_context = task_module.has_context;
        has_monotonic = task_module.has_monotonic;
        if !has_monotonic{
            module.push(quote!(pub use super::__rtic_internal_Monotonics as Monotonics;));
        }
    }
    
    // Init has already been generated.
    if has_context{
        init_items.push(quote!{
            mod #name{
                #(#module)*
            }
        });
        if !has_monotonic{
            init_items.push(
                quote!{
                    #[allow(non_snake_case)]
                    #[allow(non_camel_case_types)]
                    pub struct __rtic_internal_Monotonics();
                }
            );
        }

    } else{
        let module = generate_module(
                name, 
                app, 
                analysis, 
                extra);
        init_items.push(module);
    }
    
    


    (init_items, user_init)
}


fn generate_module(
    name: &Ident,
    app: &App, 
    _analysis: &Analysis,
    extra: &Extra,
    ) -> TokenStream2{

    // items - items outside of the module.
    let mut items = vec![];
    // module_items - don't understand. Think it is functions in called function?.
    // it will be inside "pub mod #name"
    let mut module_items = vec![];
    // fields - builds the execution context struct.
    // Need to implement after shared and local resources
    let mut fields: Vec<TokenStream2> = vec![];
    // values - the implementation of execution context.
    // Need to implement after shared and local resources
    let mut values: Vec<TokenStream2> = vec![];
    // Used to copy task cfgs to the whole module
    // Don't think this will be needed here. It is only used in software.
    let task_cfgs: Vec<Attribute> = vec![];

    let lt;

    let vector: Vec<Attribute> = vec![];
    let cfgs = &vector;

    // Module 001
    fields.push(quote!(
        /// Core (Cortex-M) peripherals
        pub core: rtic::export::Peripherals
    ));
    if extra.peripherals {
        let device = &extra.device;

        fields.push(quote!(
            /// Device peripherals
            pub device: #device::Peripherals
        ));

        values.push(quote!(device: #device::Peripherals::steal()));
    }
    lt = Some(quote!('a));
    fields.push(quote!(
        /// Critical section token for init
        pub cs: rtic::export::CriticalSection<#lt>
    ));
    values.push(quote!(cs: rtic::export::CriticalSection::new()));
    values.push(quote!(core));


    // Module 007
    let monotonic_types: Vec<_> = app
        .monotonics
        .iter()
        .map(|(_, monotonic)| {
            let mono = &monotonic.ty;
            quote! {#mono}
        })
        .collect();

    let internal_monotonics_ident = util::mark_internal_name("Monotonics");

    items.push(quote!(
        /// Monotonics used by the system
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        pub struct #internal_monotonics_ident(
            #(pub #monotonic_types),*
        );
    ));

    module_items.push(quote!(
        pub use super::#internal_monotonics_ident as Monotonics;
    ));

    //Module 009
    let doc = "Initialization function";

    //Module 014
    let core: Option<TokenStream2> = Some(quote!(core: rtic::export::Peripherals,));

    //Module 016
    let priority: Option<TokenStream2> = None;

    //Module 018
    let internal_context_name = util::internal_task_ident(name, "Context");

    items.push(quote!(
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
    ));

    module_items.push(quote!(
        #(#cfgs)*
        pub use super::#internal_context_name as Context;
    ));

    //Module 020 and 021
    if items.is_empty() {
        return quote!()
    } else {
        return quote!(
            #(#items)*

            #[allow(non_snake_case)]
            #(#task_cfgs)*
            #[doc = #doc]
            pub mod #name {
                #(#module_items)*
            }
        )
    }
}



