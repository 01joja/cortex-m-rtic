use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{ast::{App, HardwareTask}, Context};

use syn::{Attribute, Ident, LitInt, PatType};

use crate::new_codegen::{
    analyze::Analysis,
    check::Extra,
};

mod post_init;
mod pre_init;

use super::{module,local_resources_struct,shared_resources_struct};

use crate::codegen::util;

/// Generate support code for hardware tasks (`#[exception]`s and `#[interrupt]`s)
pub fn codegen(
    app: &App, 
    analysis: &Analysis,
    extra: &Extra,
) -> (
    // mod_app_idle -- the `${init}Resources` constructor
    Option<TokenStream2>,
    // root_init -- items that must be placed in the root of the crate:
    // - the `${init}Locals` struct
    // - the `${init}Resources` struct
    // - the `${init}LateResources` struct
    // - the `${init}` module, which contains types like `${init}::Context`
    Vec<TokenStream2>,
    // user_init -- the `#[init]` function written by the user
    TokenStream2,
    // main_init: (put pre_init + call_init + post_init in to one.)
    // - pre_init
    // ++ generates code that runs before `#[init]`
    // - call_init:
    // ++ call to the user `#[init]`
    // - post_init:
    // ++ generates code that runs after `#[init]` returns
    TokenStream2,
) {

    let init = &app.init;
    let mut local_needs_lt = false;
    let name = &init.name;

    let mut root_init = vec![];

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

    root_init.push(quote! {
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

    let mut mod_app = None;

    // `${task}Locals`
    if !init.args.local_resources.is_empty() {
        let (item, constructor) =
            local_resources_struct::codegen_original(Context::Init, &mut local_needs_lt, app);

        root_init.push(item);

        mod_app = Some(constructor);
    }
    
    let main_init = codegen_main_init(name, app, analysis, extra);


    let _module1 = module::codegen_original(
        false,
        true,
        false,
        Context::Init,
        false,
        local_needs_lt,
        app, 
        analysis, 
        extra
    );


    let module2 = module_func(
        name, 
        local_needs_lt, 
        app, 
        analysis, 
        extra);
    
    // let test1 = format!("{:?}", module1);
    // let test2 = format!("{:?}", module2);
    // let hej = assert_eq!(test1,test2);

    root_init.push(module2);

    (mod_app, root_init, user_init, main_init)
}


// Will replace module.
fn module_init() -> TokenStream2{

    quote!(println!("mod_app"))
}


fn codegen_main_init(
    name: &Ident, 
    app: &App, 
    analysis: &Analysis,
    extra: &Extra,
) -> TokenStream2{

    let pre_init_stmts = pre_init::codegen_original(app, analysis, extra);
    let post_init_stmts = post_init::codegen_original(app, analysis);
    
    
    // let locals_new = locals_new.iter();
    let call_init = quote! {
        let (shared_resources, local_resources, mut monotonics) = #name(#name::Context::new(core.into()));
    };

    quote!(#(#pre_init_stmts)*

    #[inline(never)]
    fn __rtic_init_resources<F>(f: F) where F: FnOnce() {
        f();
    }

    // Wrap late_init_stmts in a function to ensure that stack space is reclaimed.
    __rtic_init_resources(||{
        #call_init

        #(#post_init_stmts)*
    });)
}



// during basic:
// Module 001
// Module 007
// Module 009
// Module 014
// Module 016
// Module 018
// Module 021


fn module_func(
    name: &Ident,
    _local_resources_tick:bool,
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

fn user_init_fn() -> TokenStream2{
    quote!(println!("user_init"))
}



