#![allow(unused_imports)]
#![allow(dead_code)]


use quote::quote;
use proc_macro2::{
    Span, 
    TokenStream as TokenStream2};
// use proc_macro::TokenStream;
// use quote::quote;
use rtic_syntax::ast::App;

use std::io::{Error, ErrorKind};

use proc_macro::TokenStream;
use std::{env, fs, path::Path};

use rtic_syntax::Settings;
use rtic_syntax::P;

// Internal
use crate::analyze;
use crate::check;

mod print;
mod generate_syntax;

// Passes
mod software_pass;
mod hardware_pass;
mod monotonics_pass;
mod resources_pass;
mod recreate_feature;

// use syn::{Attribute, Ident, LitInt, PatType};

use crate::{analyze::Analysis, check::Extra};

/// The passes implemented, If you want a new one
/// add it here and in the match statement in the
/// loop further down
const STANDARD_PASSES: [&'static str; 4] = [
    "monotonics",
    "resources",
    "software",
    "hardware"
];

/// #Modular RTIC
/// 
/// To run the modular RTIC specify a pass in the rtic::app()
/// 
/// Example:
/// #[rtic::app(device = lm3s6965, compiler_passes = ["standard"])]
/// 
/// That runs the following passes:
/// rtic::parser -> codegen::monotonics -> rtic::parser -> codegen::resources -> 
/// rtic::parser -> codegen::software -> rtic::parser -> codegen::hardware -> output
/// 
/// Example of self-defined passes:
/// #[rtic::app(device = lm3s6965, compiler_passes = ["hardware"])]
/// rtic::parser -> codegen::hardware -> output
///   
pub fn app(
    user_argument: TokenStream, 
    user_code: TokenStream
) -> TokenStream2 {

    let mut app;
    let mut analysis;
    let mut extra;

    // First parse the macro so the order of the passes can be 
    // accessed
    (app, analysis, extra) = 
        match call_parse(user_argument, user_code){
            Ok(x) => x,
            Err(e) => return e.into(),
        };
    
    // Extracts the passes.
    let mut passes = app.args.passes.clone();

    // Inserts the standard passes if standard is used.
    if passes[0].as_str() == "standard"{
        passes = vec![];
        for pass in STANDARD_PASSES{
            passes.push(pass.to_string());
        }
    }else{
        match check_and_extract_passes(app.args.passes.clone()){
            Ok(p) => p,
            Err(e) => return e.into(),
        };
    }

    // Reverses the passes to pop the last element first.
    passes.reverse();

    let mut generated_arguments = quote!();
    let mut generated_code = quote!();
    let mut skip_first_parse = true;

    loop{
        let pass = match passes.pop(){
            Some(s) => s,
            None => {        
                // No more passes and codegen terminates.
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


        // Add different passes here:
        match pass.as_str() {
            "monotonics" =>{
                (generated_arguments, generated_code) 
                    = monotonics_pass::codegen(&app, &analysis, &extra);
            }
            
            "resources" =>{
                (generated_arguments, generated_code) 
                    = resources_pass::codegen(&app, &analysis, &extra);
            }

            "software" => {
                (generated_arguments, generated_code) 
                    = software_pass::codegen(&app,&extra);
            }

            "hardware" => {
                (generated_arguments, generated_code)  
                    = hardware_pass::codegen(&app,&analysis,&extra);
            }
            
            unknown_pass => {
                // Should be caught in "check_and_extract_passes".
                let mut message = format!("Pass \"{}\" is not implemented in mod app in modular_codegen.\n", unknown_pass);
                message.push_str("If you added a new one, make sure it is added to the match statement");
                unimplemented!("{}",message)
            }
        }
    }
}

/// Makes a call to the parser in rtic-syntax.
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

/// Checks the passes and extracts the strings as &str.
fn check_and_extract_passes(passes: Vec<String>) -> Result<(),TokenStream>{
    let mut last_i = 0;

    for pass in passes{
        let mut current_i = 0;
        let mut found_pass = false;

        for s_pass in STANDARD_PASSES{
            if s_pass == pass.as_str(){
                found_pass = true;
                break;
            } else {
                current_i +=1;
            }
        }

        if !found_pass{
            let mut message = format!("Pass \"{pass}\" is not implemented. Make sure you have spelled it correctly or try:\n");
            message.push_str("\"compile_passes = [standard]\" to get the standard chain");
            let error = syn::Error::new(Span::call_site(), message).to_compile_error();
            return Err(error.into());
        }else if current_i < last_i {
            let message = format!("Pass \"{pass}\" should come before \"{}\", please change the order",STANDARD_PASSES[last_i]);
            let error = syn::Error::new(Span::call_site(), message).to_compile_error();
            return Err(error.into());
        }

        last_i = current_i;
    }
    Ok(())
}
