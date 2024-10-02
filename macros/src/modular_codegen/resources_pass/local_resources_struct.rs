use proc_macro2::{TokenStream as TokenStream2, Ident};
use quote::quote;
use rtic_syntax::{
    ast::{App, TaskLocal, LocalResources},
    Context,
};

use super::r_names;


/// Generates local resources structs
pub fn codegen(
    task_name: &Ident, 
    context: Context, 
    needs_local_life_time: &mut bool, 
    app: &App) -> 
    (TokenStream2, TokenStream2) 
{
    let mut life_time = None;

    let resources = match context {
        Context::Init => {
            &app.init.args.local_resources
        },
        Context::Idle => {
            &app.idle.as_ref().unwrap().args.local_resources
        },
        Context::HardwareTask(name) => {
            &app.hardware_tasks[name].args.local_resources
        },
        Context::SoftwareTask(name) => {
            &app.software_tasks[name].args.local_resources
        },
    };

    let mut fields = vec![];
    let mut values = vec![];
    let mut has_cfgs = false;

    for (name, task_local) in resources {
        let (cfgs, ty, is_declared) = match task_local {
            TaskLocal::External => {
                let r = app.local_resources.get(name).expect("UNREACHABLE");
                (&r.cfgs, &r.ty, false)
            }
            TaskLocal::Declared(r) => {
                (&r.cfgs, &r.ty, true)
            },
            _ => unreachable!(),
        };
        
        has_cfgs |= !cfgs.is_empty();


        let lt = if context.runs_once() {
            quote!('static)
        } else {
            life_time = Some(quote!('a));
            quote!('a)
        };

        let mangled_name = if matches!(task_local, TaskLocal::External) {
            r_names::racycell_external_local_r(name)
        } else {
            r_names::racycell_declared_local_r(name, &task_name)
        };

        fields.push(quote!(
            #(#cfgs)*
            pub #name: &#lt mut #ty
        ));

        let expr = if is_declared {
            // If the local resources is already initialized, we only need to access its value and
            // not go through an `MaybeUninit`
            quote!(&mut *#mangled_name.get_mut())
        } else {
            quote!(&mut *(&mut *#mangled_name.get_mut()).as_mut_ptr())
        };

        values.push(quote!(
            #(#cfgs)*
            #name: #expr
        ));
    }

    if life_time.is_some() {
        *needs_local_life_time = true;
        
        // The struct could end up empty due to `cfg`s leading to an error due to `'a` being unused
        if has_cfgs {
            fields.push(quote!(
                #[doc(hidden)]
                pub __marker__: core::marker::PhantomData<&'a ()>
            ));

            values.push(quote!(__marker__: core::marker::PhantomData));
        }
    }

    let doc = format!("Local resources `{}` has access to", context.ident(app));
    let ident = r_names::local_r_struct(task_name);
    let item = quote!(
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        #[doc = #doc]
        pub struct #ident<#life_time> {
            #(#fields,)*
        }
    );

    let constructor = quote!(
        impl<#life_time> #ident<#life_time> {
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
