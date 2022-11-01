use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{ast::{App, HardwareTask}, Context};

use syn::{Attribute, Ident, LitInt, PatType};

use crate::{
    analyze::Analysis,
    check::Extra,
};

/// Generate support code for hardware tasks (`#[exception]`s and `#[interrupt]`s)
pub fn codegen(
    app: &App,
    analysis: &Analysis,
    extra: &Extra,
) -> (
    // mod_app_hardware_tasks -- interrupt handlers and `${task}Resources` constructors
    Vec<TokenStream2>,
    // root_hardware_tasks -- items that must be placed in the root of the crate:
    // - `${task}Locals` structs
    // - `${task}Resources` structs
    // - `${task}` modules
    Vec<TokenStream2>,
    // user_hardware_tasks -- the `#[task]` functions written by the user
    Vec<TokenStream2>,
) {
    let mut mod_app_tasks = vec![];
    let mut root_tasks = vec![];
    let mut user_tasks = vec![];

    for (name, task) in &app.hardware_tasks{
        user_tasks.push(user_task(name,task));
        root_tasks.push(root(task));
        mod_app_tasks.push(mod_app(task));
    }

    (mod_app_tasks, root_tasks, user_tasks)
}


// handles user_task
fn user_task(name: &Ident,task: &HardwareTask,) -> TokenStream2{

    if !task.is_extern {
        let attrs = &task.attrs;
        let context = &task.context;
        let stmts = &task.stmts;
        quote!(
            #(#attrs)*
            #[allow(non_snake_case)]
            fn #name(#context: #name::Context) {
                use rtic::Mutex as _;
                use rtic::mutex::prelude::*;

                #(#stmts)*
            }
        )
    }else{
        quote!()
    }
}

// handles the root thingies
fn root(task: &HardwareTask,) -> TokenStream2{
    quote!(println!("root");)
}

// handles the mod_things thingies
fn mod_app(task: &HardwareTask,) -> TokenStream2{
    quote!(println!("mod_app");)
}

