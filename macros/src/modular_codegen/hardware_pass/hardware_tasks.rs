use std::sync::Arc;

use proc_macro2::{TokenStream as TokenStream2, Span};
use quote::quote;
use rtic_syntax::{ast::{App, HardwareTask, taskModule}, Context};

use syn::{Attribute, Ident, LitInt, PatType};

use crate::modular_codegen::{
    analyze::Analysis,
    check::Extra,
};



/// Generate support code for hardware tasks (`#[exception]`s and `#[interrupt]`s)
pub fn codegen(
    app: &App, 
    _analysis: &Analysis,
    _extra: &Extra,
) -> (
    // interrupts -- interrupt handlers
    Vec<TokenStream2>,
    // modules -- modules
    Vec<TokenStream2>,
    // user_tasks -- the `#[task]` functions written by the user
    Vec<TokenStream2>,
) {

    let mut interrupts = vec![];
    let mut modules = vec![];
    let mut user_tasks = vec![];

    for (name, task) in &app.hardware_tasks{
        let _symbol = task.args.binds.clone();
        let _priority = task.args.priority;
        let _cfgs = &task.cfgs;
        let _attrs = &task.attrs;
        
        let mut interrupt_context = None;
        let mut user_task_context = None;

        // If erlier passes has put something in the internal module.
        if let Some(module) = app.task_modules.get(name){
            
            let items = &module.items;
            modules.push(
                quote!{
                    pub mod #name{
                        #(#items)*
                    }
                }
            );

            // If the module has context it needs to be used in the call and task.
            if module.has_context{
                interrupt_context = Some(quote!{
                    #name::Context::new(&rtic::export::Priority::new(PRIORITY))
                });
                let context = &task.context;
                user_task_context = Some(quote!{#context: #name::Context})
            }
        }
        
        let attrs = &task.attrs;
        let cfgs = &task.cfgs;
        let symbol = task.args.binds.clone();
        let priority = task.args.priority;

        interrupts.push(
            quote!(
                #[allow(non_snake_case)]
                #[no_mangle]
                #(#attrs)*
                #(#cfgs)*
                unsafe fn #symbol() {
                    const PRIORITY: u8 = #priority;
                    rtic::export::run(PRIORITY, || {
                        #name(
                            #interrupt_context
                        )
                    });
                }
            )
        );

        let attrs = &task.attrs;
        let stmts = &task.stmts;
        user_tasks.push(
            quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                fn #name(#user_task_context) {
                    use rtic::Mutex as _;
                    use rtic::mutex::prelude::*;

                    #(#stmts)*
                }
            )
        );

    }

    (interrupts, modules, user_tasks)
}
