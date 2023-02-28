use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;
use rtic_syntax::{ast::App, Context};

use crate::codegen::util;

use super::r_names;

/// Generate shared resources structs
pub fn codegen(
    task_name: 
    &Ident, context: Context, 
    needs_shared_life_time: &mut bool, 
    app: &App) -> 
    (TokenStream2, TokenStream2) 
{
    let mut lt = Some(quote!('a));

    
    let resources = match context {
        Context::Init => unreachable!("Tried to generate shared resources struct for init"),
        Context::Idle => {&app.idle.as_ref().unwrap().args.shared_resources},
        Context::HardwareTask(name) => {&app.hardware_tasks[name].args.shared_resources},
        Context::SoftwareTask(name) => {&app.software_tasks[name].args.shared_resources},
    };


    let mut fields = vec![];
    let mut values = vec![];
    let mut has_cfgs = false;

    for (name, access) in resources {
        let res = app.shared_resources.get(name).expect("UNREACHABLE");

        let cfgs = &res.cfgs;
        has_cfgs |= !cfgs.is_empty();

        // access hold if the resource is [x] (exclusive) or [&x] (shared)
        let mut_ = if access.is_exclusive() {
            Some(quote!(mut))
        } else {
            None
        };
        let ty = &res.ty;
        let mangled_name = r_names::racycell_shared_r(name);
        let shared_name = r_names::need_to_lock_r(name);

        if res.properties.lock_free {
            // Lock free resources of `idle` and `init` get 'static lifetime
            let lt = if context.runs_once() {
                lt = None;
                quote!('static)
            } else {
                quote!('a)
            };

            fields.push(quote!(
                #(#cfgs)*
                pub #name: &#lt #mut_ #ty
            ));
        } else if access.is_shared() {

            fields.push(quote!(
                #(#cfgs)*
                pub #name: &'a #ty
            ));
        } else {
            // Resource proxy
            fields.push(quote!(
                #(#cfgs)*
                pub #name: shared_resources::#shared_name<'a>
            ));

            values.push(quote!(
                #(#cfgs)*
                #name: shared_resources::#shared_name::new(priority)

            ));

            // continue as the value has been filled,
            continue;
        }

        let expr = if access.is_exclusive() {
            quote!(&mut *(&mut *#mangled_name.get_mut()).as_mut_ptr())
        } else {
            quote!(&*(&*#mangled_name.get()).as_ptr())
        };

        values.push(quote!(
            #(#cfgs)*
            #name: #expr
        ));
    }

    if lt.is_some() {
        *needs_shared_life_time = true;

        // The struct could end up empty due to `cfg`s leading to an error due to `'a` being unused
        if has_cfgs {
            fields.push(quote!(
                #[doc(hidden)]
                pub __marker__: core::marker::PhantomData<&'a ()>
            ));

            values.push(quote!(__marker__: core::marker::PhantomData));
        }
    }

    let doc = format!("Shared resources `{}` has access to", context.ident(app));
    let ident = r_names::shared_r_struct(task_name);
    let item = quote!(
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        #[doc = #doc]
        pub struct #ident<#lt> {
            #(#fields,)*
        }
    );

    let arg = if context.is_init() {
        None
    } else {
        Some(quote!(priority: &#lt rtic::export::Priority))
    };
    let constructor = quote!(
        impl<#lt> #ident<#lt> {
            #[inline(always)]
            pub unsafe fn new(#arg) -> Self {
                #ident {
                    #(#values,)*
                }
            }
        }
    );

    (item, constructor)
}
