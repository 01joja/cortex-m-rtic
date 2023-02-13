

use proc_macro2::{Span};
use syn::{Ident};


pub fn ident(name: &str) -> Ident {
    let span = Span::call_site();
    Ident::new(name, span)
}


