
use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;
use quote::quote;
use rtic_syntax::{ast::{App, SharedResources, LocalResources}, Context};

// use crate::{analyze::Analysis, check::Extra};

use super::r_names;

/// Generates `local` variables and local resource proxies
///
/// I.e. the `static` variables and theirs proxies.
pub fn codegen(
    app: &App,
) ->
    // modules -- the modules that holds context.
    Vec<TokenStream2> {
    // let mut modules = vec![];

    // let has_shared = app.init.args.local_resources.len() > 0;
    // modules.push(module(&app.init.name, false, has_shared));

    // if let Some(idle) = &app.idle{
    //     let has_local = idle.args.local_resources.len() > 0;
    //     let has_shared = idle.args.shared_resources.len() > 0;
    //     modules.push(module(&idle.name,has_local, has_shared));
    // }

    // for (name, task) in &app.hardware_tasks{
    //     let has_local = task.args.local_resources.len() > 0;
    //     let has_shared = task.args.shared_resources.len() > 0;
    //     modules.push(module(&name, has_local, has_shared));
    // }

    // for (name, task) in &app.software_tasks{
    //     let has_local = task.args.local_resources.len() > 0;
    //     let has_shared = task.args.shared_resources.len() > 0;
    //     modules.push(module(&name, has_local, has_shared));
    // }

    let mut v = vec![];
    v
}



