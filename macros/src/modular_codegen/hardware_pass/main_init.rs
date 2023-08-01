use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{ast::{App, HardwareTask}, Context};

use syn::{Attribute, Ident, LitInt, PatType};

use crate::modular_codegen::{
    analyze::Analysis,
    check::Extra,
};

use super::{module};

use crate::codegen::util;

mod post_init;
mod pre_init;


pub fn codegen(
    app: &App, 
    analysis: &Analysis,
    extra: &Extra,
) -> 
    // main_init: (put pre_init + call_init + post_init in to one.)
    // - pre_init
    // ++ generates code that runs before `#[init]`
    // - call_init:
    // ++ call to the user `#[init]`
    // - post_init:
    // ++ generates code that runs after `#[init]` returns
    TokenStream2{

    let name = &app.init.name;

    let pre_init_stmts = pre_init::codegen_original(app, analysis, extra);
    // let post_init_stmts = post_init::codegen_original(app, analysis);
    let mut pre_init_passes_stmts = &vec![];
    let mut post_init_stmts = &vec![];

    if let Some(main_fn) = &app.main_fn{
        pre_init_passes_stmts = &main_fn.pre_init;
        post_init_stmts = &main_fn.post_init;
    }
    
    
    // let locals_new = locals_new.iter();
    let call_init = quote! {
        let (shared_resources, local_resources, mut monotonics) = #name(#name::Context::new(core.into()));
    };

    quote!{
        
        #(#pre_init_stmts)*
        
        #(#pre_init_passes_stmts)*

        #[inline(never)]
        fn __rtic_init_resources<F>(f: F) where F: FnOnce() {
            f();
        }

        // Wrap late_init_stmts in a function to ensure that stack space is reclaimed.
        __rtic_init_resources(||{
            #call_init

            #(#post_init_stmts)*
            
            rtic::export::interrupt::enable();
        });
    }
}
