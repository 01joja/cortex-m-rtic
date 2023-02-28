

use crate::modular_codegen::generate_syntax;

use proc_macro2::{Span,TokenStream as TokenStream2};

use syn::Ident;

// use syn::{Ident, LitInt, Path};

//r stands for resource

const RTIC_LOCAL_RESOURCE: &str = "__rtic_local_resource";
const RTIC_SHARED_RESOURCE: &str = "__rtic_shared_resource";
const RTIC_CONTEXT: &str = "__rtic_context";

/// Struct 
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

/// _External_
/// 
/// name of RacyCell that holds a local resource declared in #[local] 
pub fn racycell_external_local_r(resource_name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_LOCAL_RESOURCE}_{resource_name}")
    )
}

/// _Declared(Local)_
/// 
/// name of RacyCell that holds a local resource only declared in the task
///  
/// #[task(local = [#resource_name: u32 = 0])]
pub fn racycell_declared_local_r(resource_name: &Ident, task_name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_LOCAL_RESOURCE}_{task_name}_{resource_name}")
    )
}

pub fn need_to_lock_r(resource_name: &Ident) -> Ident {
    generate_syntax::ident(
        &format!("{}_that_needs_to_be_locked", resource_name)
    )
}

/// name of RacyCell that holds a shared resource declared in #[shared] 
pub fn racycell_shared_r(resource_name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_SHARED_RESOURCE}_{resource_name}")
    )
}

/// Generates an Ident for the number of 32 bit chunks used for Mask storage.
pub fn priority_mask_chunks_ident() -> Ident {
    generate_syntax::ident(
        &format!("{RTIC_SHARED_RESOURCE}s_MASK_CHUNKS")
    )
}

pub fn priority_masks_ident() -> Ident {
    generate_syntax::ident(
        &format!("{RTIC_SHARED_RESOURCE}s_MASKS")
    )
}

pub fn context_name(name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_CONTEXT}_{name}_context")
    )
}

pub fn link_section(name: &Ident) -> String{
    format!(".uninit.rtic_r_{name}",)
}
