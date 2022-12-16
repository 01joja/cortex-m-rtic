#![allow(unused_imports)]
#![allow(dead_code)]

use quote::quote;
use proc_macro2::{
    //Span, 
    TokenStream as TokenStream2};
//use proc_macro::TokenStream;
//use quote::quote;
use rtic_syntax::ast::App;

use std::io::{Error, ErrorKind};

use proc_macro::TokenStream;
use std::{env, fs, path::Path};

use rtic_syntax::Settings;
use rtic_syntax::P;

// Internal
use crate::analyze;
use crate::check;

mod software;
mod print;
mod hardware;

//use syn::{Attribute, Ident, LitInt, PatType};

use crate::{analyze::Analysis, check::Extra};

pub fn app(
    user_argument: TokenStream, 
    user_code: TokenStream
) -> TokenStream2 {
    
    let mut app;
    let mut analysis;
    let mut extra;

    // first parse so we can access the passes
    (app, analysis, extra) = 
        match call_parse(user_argument, user_code){
            Ok(x) => x,
            Err(e) => return e.into(),
        };

    // saves contents of app to contents/app.txt and contents/app/
    print::abstract_syntax_tree(&app, &analysis, &extra);
    
    //extracts the passes.
    let mut passes = app.args.passes.clone();
    let mut generated_arguments = quote!();
    let mut generated_code = quote!();


    let mut skip_first_parse = true;

    loop{
        let pass = match passes.pop(){
            Some(s) => s,
            None => {
                let print_generated_code = format!("{:#?}",generated_code);
                fs::write("contents/generated_code.rs", print_generated_code).expect("Unable to write file");
                return generated_code;
            }
        };
        
        if skip_first_parse{
            skip_first_parse = false;
        }else{
            //Parses the generated code.
            (app, analysis, extra) = 
            match call_parse(generated_arguments.into(), generated_code.into()){
                Ok(x) => x,
                Err(e) => return e.into(),
            };
        }

        match pass.as_str(){
            "software" => {
                println!("software");
                (generated_arguments, generated_code) 
                    = software::codegen(&app,&analysis,&extra,&false);
            }
            "hardware" => {
                println!("hardware");
                (generated_arguments, generated_code)  = hardware::codegen(&app,&analysis,&extra);
            }
            _ => {
                panic!("No matching pass found")
            }
        }

        
    }

    

    // let mut settings = Settings::default();
    // settings.optimize_priorities = false;
    // settings.parse_binds = true;
    // settings.parse_extern_interrupt = true;


    // let (app_hardware, analysis_hardware) = 
    //     match rtic_syntax::parse(generated_arguments.into(), generated_code.into(), settings) {
    //     Err(e) => {
    //         println!("Failed pars after software");
    //         return e.to_compile_error().into()},
    //     Ok(x) => x,
    // };


    // let extra_hardware = match check::app(&app_hardware, &analysis_hardware) {
    //     Err(e) => return e.to_compile_error().into(),
    //     Ok(x) => x,
    // };

    // let analysis_hardware = analyze::app(analysis_hardware, &app_hardware);

    // let output = hardware::codegen(&app_hardware,&analysis_hardware,&extra_hardware);

    
    
    // output
}

/// Makes a call to the parser in rtic-syntax
fn call_parse(arguments: TokenStream, code: TokenStream) -> 
    Result<(P<App>, P<Analysis>, Extra),TokenStream> {

    let mut settings = Settings::default();
    settings.optimize_priorities = false;
    settings.parse_binds = true;
    settings.parse_extern_interrupt = true;

    let (app, analysis) = 
        match rtic_syntax::parse(
            arguments.into(), 
            code.into(), 
            settings) {
            Err(e) => {
                return Err(e.to_compile_error().into())},
            Ok(x) => x,
    };

    let extra = match check::app(&app, &analysis) {
        Err(e) => return Err(e.to_compile_error().into()),
        Ok(x) => x,
    };

    let analysis = analyze::app(analysis, &app);

    Ok((app, analysis, extra))
}



