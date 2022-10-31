use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro::TokenStream;
use quote::quote;
use rtic_syntax::ast::App;
use std::fs;

mod print_app;
mod sw_to_hw;

use syn::{Attribute, Ident, LitInt, PatType};

use crate::{analyze::Analysis, check::Extra};

pub fn app(
    app: &App, 
    analysis: &Analysis,
    extra: &Extra,
) -> TokenStream2 {

    // saves contents of app to contents/app.txt and contents/app/
    print_app::print_app(&app);

    let output = sw_to_hw::new_codegen(&app,&analysis,&extra);
    

    // saves/prints information about extra
    // extra only contains device name. (For now)
    let contents_extra = format!("{:#?}",extra);
    fs::write("contents/extra.rs", contents_extra);

    
    let contents_output = format!("{:#?}",output);
    fs::write("contents/output.rs", contents_output).expect("Unable to write file");
    
    output
}


