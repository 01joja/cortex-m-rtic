#![allow(unused_imports)]
#![allow(dead_code)]

use proc_macro2::{
    //Span, 
    TokenStream as TokenStream2};
//use proc_macro::TokenStream;
//use quote::quote;
use rtic_syntax::ast::App;


use proc_macro::TokenStream;
use std::{env, fs, path::Path};

use rtic_syntax::Settings;


// Internal
use crate::analyze;
use crate::check;

mod software;
mod print_app;
mod hardware;

//use syn::{Attribute, Ident, LitInt, PatType};

use crate::{analyze::Analysis, check::Extra};

pub fn app(
    app: &App, 
    analysis: &Analysis,
    extra: &Extra,
) -> TokenStream2 {

    let (argument, software_output) = software::codegen(&app,&analysis,&extra);

    let print = format!("{:#?}",software_output);
    match fs::write("contents/software_output.rs", print){
        Ok(()) => {},
        Err(error) => println!("Problem opening the file: {:?}", error),
    }

    // saves contents of app to contents/app.txt and contents/app/
    match print_app::print_app(&app){
        Ok(()) => {},
        Err(error) => println!("Problem opening the file: {:?}", error),
    };

    let mut settings = Settings::default();
    settings.optimize_priorities = false;
    settings.parse_binds = true;
    settings.parse_extern_interrupt = true;

    let (app_hardware, analysis_hardware) = match rtic_syntax::parse(argument.into(), software_output.into(), settings) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    let extra_hardware = match check::app(&app_hardware, &analysis_hardware) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    let analysis_hardware = analyze::app(analysis_hardware, &app_hardware);

    

    let output = hardware::codegen(&app_hardware,&analysis_hardware,&extra_hardware);
    

    // saves/prints information about extra
    // extra only contains device name. (For now)
    // let contents_extra = format!("{:#?}",extra);
    // fs::write("contents/extra.rs", contents_extra);

    
    let contents_output = format!("{:#?}",output);
    fs::write("contents/output.rs", contents_output).expect("Unable to write file");
    
    output
    //software_output
}


