
//Generate special syntax that has with spawn to do.

use proc_macro2::{Span};
use syn::{Ident, LitInt};

// creates an indent.
pub fn ident(name: &str) -> Ident {
    let span = Span::call_site();
    Ident::new(name, span)
}


/// The name to get better RT flag errors
pub fn rt_error() -> Ident {
    Ident::new(
        "you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml",
        Span::call_site(),
    )
}


/// Turns `capacity` into an unsuffixed integer literal
pub fn capacity_literal(capacity: usize) -> LitInt {
    LitInt::new(&capacity.to_string(), Span::call_site())
}

/// Insert value as a string and get a LitInt (Literal Integer)
pub fn literal_int(value: String) -> LitInt {
    LitInt::new(value.as_str(),Span::call_site())
}
