
use core::sync::atomic::{AtomicUsize, Ordering};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Attribute, Ident, LitInt, PatType};

/// Insert value as a string and get a LitInt (Literal Integer)
pub fn literal_int(value: String) -> LitInt {
    LitInt::new(value.as_str(),Span::call_site())
}

