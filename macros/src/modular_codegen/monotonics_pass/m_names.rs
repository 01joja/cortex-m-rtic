

use crate::modular_codegen::generate_syntax;

use proc_macro2::{Span,TokenStream as TokenStream2};

use syn::Ident;

// use syn::{Ident, LitInt, Path};

// m stands for monotonic

const RTIC_MONOTONIC: &str = "__rtic_monotonic";

/// Monotonic struct. Used to pass Systick<100> in init to 
pub fn monotonic_struct() -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_monotonic_struct")
    )
}

/// Generates an identifier for the `enum` of `schedule`-able tasks
pub fn schedule_tasks() -> Ident {
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_schedule_tasks")
    )
}

/// An interrupt enum
pub fn interrupt() -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_interrupt")
    )
}

pub fn timer_queue_marker() -> Ident {
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_TIMER_QUEUE_MARKER")
    )
}

/// Generates an identifier for a timer queue
pub fn timer_queue(name: &Ident) -> Ident {
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_{name}_timer_q")
    )
}

/// Really needed? adds the internal to the monotonic name.
pub fn internal_monotonic_name(name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_{name}")
    )
}

pub fn monotonic_storage(name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_STORAGE_{name}")
    )
}

/// Generates an identifier for a ready queue
///
/// There may be several task dispatchers, one for each priority level.
/// The ready queues are SPSC queues
pub fn rq_ident(priority: u8) -> Ident {
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_PRIO{priority}_REQUEST_Q")
    )
}


pub fn monotonic_instants(m_name: &Ident, sw_name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_{sw_name}_{m_name}_INSTANTS")
    )
}

pub fn monotonic_spawn_after(m_name: &Ident, sw_name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_{m_name}_{sw_name}_spawn_after")
    )
}

pub fn monotonic_spawn_at(m_name: &Ident, sw_name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_{m_name}_{sw_name}_spawn_at")
    )
}

pub fn monotonic_spawn_handler(m_name: &Ident, sw_name: &Ident) -> Ident{
    generate_syntax::ident(
        &format!("{RTIC_MONOTONIC}_{m_name}_{sw_name}_spawn_handler")
    )
}

/// Just calls generate_syntax::rt_error()
pub fn rt_error() -> Ident{
    generate_syntax::rt_error()
}