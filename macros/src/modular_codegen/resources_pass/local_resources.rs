use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{Ident};
use quote::quote;
use rtic_syntax::{ast::App, Context};

// use crate::{analyze::Analysis, check::Extra};

use super::r_names;

/// Generates `local` variables and local resource proxies
///
/// I.e. the `static` variables and theirs proxies.
pub fn codegen(
    app: &App,
) -> 
    // mod_app -- the `static` variables behind the proxies
    Vec<TokenStream2>//,
    // task_modules -- the `resources` module
    //TokenStream2,
 {
    let mut mod_app = vec![];
    // let mut mod_resources: _ = vec![];

    // All local resources declared in the `#[local]' struct
    for (name, res) in &app.local_resources {
        let cfgs = &res.cfgs;
        let ty = &res.ty;
        let mangled_name = r_names::external_local_r(name);

        let attrs = &res.attrs;

        // late resources in `util::link_section_uninit`
        // unless user specifies custom link section
        let section = if attrs.iter().any(|attr| attr.path.is_ident("link_section")) {
            None
        } else {
            let section = format!(".uninit.rtic_{}",name);
            Some(quote!(#[link_section = #section]))
        };

        // For future use
        // let doc = format!(" RTIC internal: {}:{}", file!(), line!());
        mod_app.push(quote!(
            #[allow(non_camel_case_types)]
            #[allow(non_upper_case_globals)]
            // #[doc = #doc]
            #[doc(hidden)]
            #(#attrs)*
            #(#cfgs)*
            #section
            static #mangled_name: rtic::RacyCell<core::mem::MaybeUninit<#ty>> = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
        ));
    }


    // Is these the late resources?

    // All declared `local = [NAME: TY = EXPR]` local resources
    for (task_name, resource_name, task_local) in app.declared_local_resources() {
        let cfgs = &task_local.cfgs;
        let ty = &task_local.ty;
        let expr = &task_local.expr;
        let attrs = &task_local.attrs;


        let mangled_name = r_names::declared_local_r(resource_name, task_name);

        // For future use
        // let doc = format!(" RTIC internal: {}:{}", file!(), line!());
        mod_app.push(quote!(
            #[allow(non_camel_case_types)]
            #[allow(non_upper_case_globals)]
            // #[doc = #doc]
            #[doc(hidden)]
            #(#attrs)*
            #(#cfgs)*
            static #mangled_name: rtic::RacyCell<#ty> = rtic::RacyCell::new(#expr);
        ));
    }

    mod_app
}