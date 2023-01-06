
use proc_macro2::{Span,TokenStream as TokenStream2};

use syn::{Ident, LitInt, Path};

const RTIC_DISPATCHER: &str = "__rtic_dispatcher";
const RTIC_SOFTWARE_TASK: &str = "__rtic_sw_task";


pub fn task_variable(task_name: &Ident, variable_name: &str) -> Ident{
    Ident::new(
        &format!("{RTIC_SOFTWARE_TASK}_{task_name}_{variable_name}"), 
        Span::call_site())
}

pub fn task_same_prio(priority: &str) -> Ident{
    Ident::new(
        &format!("{RTIC_DISPATCHER}_priority_{priority}"), 
        Span::call_site())
}

pub fn dispatcher_variable(variable_name: &str) -> Ident{
    Ident::new(
        &format!("{RTIC_DISPATCHER}_{variable_name}"), 
        Span::call_site())
}

