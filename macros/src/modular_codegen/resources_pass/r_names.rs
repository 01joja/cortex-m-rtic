

use crate::modular_codegen::generate_syntax;

use proc_macro2::{Span,TokenStream as TokenStream2};

use syn::Ident;

// use syn::{Ident, LitInt, Path};

//r stands for resource

const RTIC_LOCAL_RESOURCE: &str = "__rtic_local_resource";
const RTIC_SHARED_RESOURCE: &str = "__rtic_shared_resource";
const RTIC_CONTEXT: &str = "__rtic_context";

pub fn local_r_struct(name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_LOCAL_RESOURCE}_{name}_local_resources")
    )
}

/// Struct 
pub fn shared_r_struct(name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_SHARED_RESOURCE}_{name}_shared_resources")
    )
}

/// name of RacyCell that holds a local resource declared in #[local] 
/// 
/// "External"
pub fn external_local_r(resource_name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_LOCAL_RESOURCE}_{resource_name}")
    )
}

/// name of RacyCell that holds a local resource declared in #[task] 
/// 
/// "Declared(Local)"
pub fn declared_local_r(resource_name: &Ident, task_name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_LOCAL_RESOURCE}_{task_name}_{resource_name}")
    )
}



pub fn context_name(name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_CONTEXT}_{name}_context")
    )
}
