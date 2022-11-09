use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{
    ast::{App, TaskLocal},
    Context,
};

use super::util;

/// Generates local resources structs
pub fn codegen_original(ctxt: Context, needs_lt: &mut bool, app: &App) -> (TokenStream2, TokenStream2) {
    let mut lt = None;

    let resources = match ctxt {
        Context::Init => {println!("local_r_str 000"); &app.init.args.local_resources},
        Context::Idle => {println!("local_r_str 001"); &app.idle.as_ref().unwrap().args.local_resources},
        Context::HardwareTask(name) => {println!("local_r_str 002"); &app.hardware_tasks[name].args.local_resources},
        Context::SoftwareTask(name) => {println!("local_r_str 003"); &app.software_tasks[name].args.local_resources},
    };

    let task_name = util::get_task_name(ctxt, app);

    let mut fields = vec![];
    let mut values = vec![];
    let mut has_cfgs = false;

    for (name, task_local) in resources {
        println!("local_r_str {:?}",name);
        let (cfgs, ty, is_declared) = match task_local {
            TaskLocal::External => {
                println!("local_r_str 004"); 
                let r = app.local_resources.get(name).expect("UNREACHABLE");
                (&r.cfgs, &r.ty, false)
            }
            TaskLocal::Declared(r) => {
                println!("local_r_str 005"); 
                (&r.cfgs, &r.ty, true)
            },
            _ => unreachable!(),
        };
        
        has_cfgs |= !cfgs.is_empty();


        let lt = if ctxt.runs_once() {
            println!("local_r_str 006"); 
            quote!('static)
        } else {
            println!("local_r_str 007"); 
            lt = Some(quote!('a));
            quote!('a)
        };

        let mangled_name = if matches!(task_local, TaskLocal::External) {
            println!("local_r_str 008"); 
            util::static_local_resource_ident(name)
        } else {
            println!("local_r_str 009"); 
            util::declared_static_local_resource_ident(name, &task_name)
        };

        fields.push(quote!(
            #(#cfgs)*
            pub #name: &#lt mut #ty
        ));

        let expr = if is_declared {
            println!("local_r_str 010"); 
            // If the local resources is already initialized, we only need to access its value and
            // not go through an `MaybeUninit`
            quote!(&mut *#mangled_name.get_mut())
        } else {
            println!("local_r_str 011"); 
            quote!(&mut *(&mut *#mangled_name.get_mut()).as_mut_ptr())
        };

        values.push(quote!(
            #(#cfgs)*
            #name: #expr
        ));
    }

    if lt.is_some() {
        println!("local_r_str 100"); 
        *needs_lt = true;
        

        // The struct could end up empty due to `cfg`s leading to an error due to `'a` being unused
        if has_cfgs {
            println!("local_r_str 101"); 
            fields.push(quote!(
                #[doc(hidden)]
                pub __marker__: core::marker::PhantomData<&'a ()>
            ));

            values.push(quote!(__marker__: core::marker::PhantomData));
        }
    }

    let doc = format!("Local resources `{}` has access to", ctxt.ident(app));
    let ident = util::local_resources_ident(ctxt, app);
    let item = quote!(
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        #[doc = #doc]
        pub struct #ident<#lt> {
            #(#fields,)*
        }
    );

    let constructor = quote!(
        impl<#lt> #ident<#lt> {
            #[inline(always)]
            pub unsafe fn new() -> Self {
                #ident {
                    #(#values,)*
                }
            }
        }
    );

    (item, constructor)
}
