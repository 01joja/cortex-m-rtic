#![allow(unused_imports)]
#![allow(dead_code)]


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

use quote::quote;
use proc_macro2::{
    Span, 
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

mod software_tasks;
mod print;
mod hardware;

//use syn::{Attribute, Ident, LitInt, PatType};

use crate::{analyze::Analysis, check::Extra};

/// #Modular RTIC
/// 
/// To run the modular RTIC specify a pass in the rtic::app()
/// 
/// Example:
/// #[rtic::app(device = lm3s6965, compiler_passes = ["standard"])]
/// 
/// That runs the following passes (as of 19 of december 2022):
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

    // first parse so we can access the passes
    (app, analysis, extra) = 
        match call_parse(user_argument, user_code){
            Ok(x) => x,
            Err(e) => return e.into(),
        };

    // saves contents of app to contents/app.txt and contents/app/
    print::abstract_syntax_tree(&app);
    
    //extracts the passes.
    let mut passes = app.args.passes.clone();

    // adds a standard passes if standard is given.
    if passes[0] == "standard"{
        passes = vec!["software".to_string(),"hardware".to_string()]
    }

    // reverses the passes
    // so that the first pass given gets pop first in the loop.
    passes.reverse();

    let mut generated_arguments = quote!();
    let mut generated_code = quote!();
    let mut skip_first_parse = true;

    loop{
        let pass = match passes.pop(){
            Some(s) => s,
            None => {        
                // no more passes and codegen terminates.
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

        // add different passes here:
        match pass.as_str(){
            "software" => {
                (generated_arguments, generated_code) 
                    = software_tasks::codegen(&app,&extra);
            }
            "hardware" => {
                (generated_arguments, generated_code)  = hardware::codegen(&app,&analysis,&extra);
            }
            unknown_pass => {
                //TODO, behave as other compiling errors. Don't know how yet.
                let error = create_unknown_pass_error(unknown_pass);
                panic!("{}",error);
            }
        }
    }
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

/// creates error text for unkonwn_pass.
fn create_unknown_pass_error(error: &str) -> String{
    let part1 = format!("Pass \"{}\" is not known. Try \"compiler_passes = [\"standard\"]\"",error);
    let part2 = format!("\nto get the usual passes or look at the documentation in rtic.rs to see examples");
    return format!("{}{}",part1,part2)
}
