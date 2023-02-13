
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{
    ast::{App}, 
    analyze::Priority};
use std::fs;

use std::str::FromStr;
use crate::{analyze::Analysis, check::Extra};

use super::hardware;

pub fn codegen(
    app: &App, 
    extra: &Extra
) -> (
// Returns the argument needed for rtic_syntax::parse()
TokenStream2,

// don't know yet
TokenStream2){
    todo!("Monotonics is not implemented yet");
}