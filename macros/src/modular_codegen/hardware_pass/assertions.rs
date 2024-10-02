use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::modular_codegen::{analyze::Analysis, check::Extra};
use super::util;
use rtic_syntax::ast::App;

/// Generates compile-time assertions that check that types implement the `Send` / `Sync` traits
pub fn codegen(app: &App, _analysis: &Analysis, _extra: &Extra) -> Vec<TokenStream2> {
    let mut stmts = vec![];

    for (_, monotonic) in &app.monotonics {
        let ty = &monotonic.ty;
        stmts.push(quote!(rtic::export::assert_monotonic::<#ty>();));
    }

    stmts
}
