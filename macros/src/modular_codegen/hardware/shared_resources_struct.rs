use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{ast::App, Context};

use crate::codegen::util;

/// Generate shared resources structs
pub fn codegen_original(ctxt: Context, needs_lt: &mut bool, app: &App) -> (TokenStream2, TokenStream2) {
    let mut lt = None;

    let resources = match ctxt {
        Context::Init => unreachable!("Tried to generate shared resources struct for init"),
        Context::Idle => {println!("shared_r_str 000"); &app.idle.as_ref().unwrap().args.shared_resources},
        Context::HardwareTask(name) => {println!("shared_r_str 001"); &app.hardware_tasks[name].args.shared_resources},
        Context::SoftwareTask(name) => {println!("shared_r_str 002"); &app.software_tasks[name].args.shared_resources},
    };

    let mut fields = vec![];
    let mut values = vec![];
    let mut has_cfgs = false;

    for (name, access) in resources {
        println!("shared_r_str {:?}",name); 
        let res = app.shared_resources.get(name).expect("UNREACHABLE");

        let cfgs = &res.cfgs;
        has_cfgs |= !cfgs.is_empty();

        // access hold if the resource is [x] (exclusive) or [&x] (shared)
        let mut_ = if access.is_exclusive() {
            println!("shared_r_str 003");
            Some(quote!(mut))
        } else {
            println!("shared_r_str 004");
            None
        };
        let ty = &res.ty;
        let mangled_name = util::static_shared_resource_ident(name);
        let shared_name = util::need_to_lock_ident(name);

        if res.properties.lock_free {
            // Lock free resources of `idle` and `init` get 'static lifetime
            let lt = if ctxt.runs_once() {
                println!("shared_r_str 005");
                quote!('static)
            } else {
                println!("shared_r_str 006");
                lt = Some(quote!('a));
                quote!('a)
            };

            fields.push(quote!(
                #(#cfgs)*
                pub #name: &#lt #mut_ #ty
            ));
        } else if access.is_shared() {
            println!("shared_r_str 007");
            lt = Some(quote!('a));

            fields.push(quote!(
                #(#cfgs)*
                pub #name: &'a #ty
            ));
        } else {
            println!("shared_r_str 008");
            // Resource proxy
            lt = Some(quote!('a));

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
            println!("shared_r_str 009");
            quote!(&mut *(&mut *#mangled_name.get_mut()).as_mut_ptr())
        } else {
            println!("shared_r_str 010");
            quote!(&*(&*#mangled_name.get()).as_ptr())
        };

        values.push(quote!(
            #(#cfgs)*
            #name: #expr
        ));
    }

    if lt.is_some() {
        println!("shared_r_str 011");
        *needs_lt = true;

        // The struct could end up empty due to `cfg`s leading to an error due to `'a` being unused
        if has_cfgs {
            println!("shared_r_str 012");
            fields.push(quote!(
                #[doc(hidden)]
                pub __marker__: core::marker::PhantomData<&'a ()>
            ));

            values.push(quote!(__marker__: core::marker::PhantomData));
        }
    }

    let doc = format!("Shared resources `{}` has access to", ctxt.ident(app));
    let ident = util::shared_resources_ident(ctxt, app);
    let item = quote!(
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        #[doc = #doc]
        pub struct #ident<#lt> {
            #(#fields,)*
        }
    );

    let arg = if ctxt.is_init() {
        println!("shared_r_str 013");
        None
    } else {
        println!("shared_r_str 014");
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
