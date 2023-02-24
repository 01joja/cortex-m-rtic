
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;
use rtic_syntax::ast::{App,TaskLocal};

use crate::{analyze::Analysis, check::Extra};

use super::{r_names, resources};

/// Generates a wrapper for init that initializes 
/// the resources.
pub fn codegen(
    app: &App, 
    analysis: &Analysis, 
    extra: &Extra
) -> Vec<TokenStream2> {
    let user_init_name = &app.init.name;
    let statements = &app.init.stmts;
    let attributes = &app.init.attrs;
    let context = &app.init.context;
    let user_shared_struct = &app.init.user_shared_struct;
    let user_local_struct = &app.init.user_local_struct;

    let mut resources = vec![];

    for (task_name, hw_task) in &app.hardware_tasks{
        for (resource_name, task_type) in &hw_task.args.local_resources{
            resources.push(local_init(task_type, resource_name, task_name));
        }
        for (resources_name, _) in &hw_task.args.shared_resources{
            todo!("shared resources resource_pass//init_resources.rs");
        }
    }

    for (task_name, sw_task) in &app.software_tasks{
        for (resource_name, task_type) in &sw_task.args.local_resources{
            resources.push(local_init(task_type, resource_name, task_name));
        }
        for (resources_name, _) in &sw_task.args.shared_resources{
            todo!("shared resources resource_pass//init_resources.rs");
        }
    }


    resources
}

    


fn local_init(task_type: &TaskLocal, resource_name: &Ident, task_name: &Ident) -> TokenStream2 {

    match task_type{
        TaskLocal::External => {
            let internal_name = r_names::external_local_r(resource_name);
            quote!{
                #internal_name.get_mut().write(core::mem::MaybeUninit::new(local_resources.#resource_name));
            }
        },
        TaskLocal::Declared(_) => {
            let internal_name = r_names::declared_local_r(resource_name,task_name);
            quote!{
                #internal_name.get_mut().write(core::mem::MaybeUninit::new(local_resources.#resource_name));
            }
        },
        _ => todo!(),
    }
}
