
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;
use rtic_syntax::ast::{App,TaskLocal};

use crate::{analyze::Analysis, check::Extra};

use super::r_names;

/// Generates code that catches the return value of init and then
/// initializes the resources to the value set in init.
pub fn codegen(
    app: &App, 
    _analysis: &Analysis, 
    _extra: &Extra
) -> Vec<TokenStream2> {

    let mut resources = vec![];
    // make sure that no resources are added twice.
    let mut found_resources = vec![];

    if let Some(idle) = &app.idle{
        for (resource_name, task_type) in &idle.args.local_resources{
            if let TaskLocal::External = task_type{
                let name = resource_name.to_string();
                if found_resources.contains(&name){
                    continue;
                }
                found_resources.push(resource_name.to_string());
                
                let internal_name = r_names::racycell_external_local_r(resource_name);
                resources.push(quote!{
                    #internal_name.get_mut().write(core::mem::MaybeUninit::new(local_resources.#resource_name));
                });
            } // else if TaskLocal::Declared, it is declared in the RacyCell directly
        }
        for (resource_name, _) in &idle.args.shared_resources{
            let name = resource_name.to_string();
            if found_resources.contains(&name){
                continue;
            }
            found_resources.push(resource_name.to_string());
            
            let internal_name = r_names::racycell_shared_r(resource_name);
            resources.push(quote!{
                #internal_name.get_mut().write(core::mem::MaybeUninit::new(shared_resources.#resource_name));
            });
        }
    }

    for (_, hw_task) in &app.hardware_tasks{
        for (resource_name, task_type) in &hw_task.args.local_resources{
            if let TaskLocal::External = task_type{
                let name = resource_name.to_string();
                if found_resources.contains(&name){
                    continue;
                }
                found_resources.push(resource_name.to_string());
                
                let internal_name = r_names::racycell_external_local_r(resource_name);
                resources.push(quote!{
                    #internal_name.get_mut().write(core::mem::MaybeUninit::new(local_resources.#resource_name));
                });
            } // else if TaskLocal::Declared, it is declared in the RacyCell directly
        }
        for (resource_name, _) in &hw_task.args.shared_resources{
            let name = resource_name.to_string();
            if found_resources.contains(&name){
                continue;
            }
            found_resources.push(resource_name.to_string());
            
            let internal_name = r_names::racycell_shared_r(resource_name);
            resources.push(quote!{
                #internal_name.get_mut().write(core::mem::MaybeUninit::new(shared_resources.#resource_name));
            });
        }
    }

    for (_, sw_task) in &app.software_tasks{
        for (resource_name, task_type) in &sw_task.args.local_resources{
            if let TaskLocal::External = task_type{
                let name = resource_name.to_string();
                if found_resources.contains(&name){
                    continue;
                }
                found_resources.push(resource_name.to_string());
                
                let internal_name = r_names::racycell_external_local_r(resource_name);
                resources.push(quote!{
                    #internal_name.get_mut().write(core::mem::MaybeUninit::new(local_resources.#resource_name));
                })
            } // else if TaskLocal::Declared, it is declared in the RacyCell directly
        }
        for (resource_name, _) in &sw_task.args.shared_resources{
            let name = resource_name.to_string();
            if found_resources.contains(&name){
                continue;
            }
            found_resources.push(resource_name.to_string());
            
            let internal_name = r_names::racycell_shared_r(resource_name);
            resources.push(quote!{
                #internal_name.get_mut().write(core::mem::MaybeUninit::new(shared_resources.#resource_name));
            });
        }
    }

    resources
}
