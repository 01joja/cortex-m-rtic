
use rtic_syntax::ast::App;
use quote::quote;
use crate::{analyze::Analysis, check::Extra};
use std::fs;
use proc_macro2::TokenStream as TokenStream2;


/// saves/prints information about app, a̶n̶a̶l̶y̶s̶i̶s̶  and extra
/// to the corresponding folder in contents/
pub fn abstract_syntax_tree(
    app: &App, 
){

    create_folders();

    //app
    write_to_file("contents/app_print.rs", format!("{:#?}",app));

    write_to_file("contents/app/args_print.rs", format!("{:#?}",app.args));

    write_to_file("contents/app/name_print.rs", format!("{:#?}",app.name));

    write_to_file("contents/app/init_print.rs", format!("{:#?}",app.init));

    write_to_file("contents/app/idle_print.rs", format!("{:#?}",app.idle));

    write_to_file("contents/app/monotonics_print.rs", format!("{:#?}",app.monotonics));

    write_to_file("contents/app/shared_resources_print.rs", format!("{:#?}",app.shared_resources));

    write_to_file("contents/app/local_resources_print.rs", format!("{:#?}",app.local_resources));
    
    write_to_file("contents/app/user_imports_print.rs", format!("{:#?}",app.user_imports));
    
    write_to_file("contents/app/user_code_print.rs", format!("{:#?}",app.user_code));

    write_to_file("contents/app/hardware_tasks_print.rs", format!("{:#?}",app.hardware_tasks));

    write_to_file("contents/app/software_task_print.rs", format!("{:#?}",app.software_tasks));

    write_to_file("contents/app/user_imports_print.rs", format!("{:#?}",app.user_imports));

    write_to_file("contents/app/task_modules_print.rs", format!("{:#?}",app.task_modules));
    
    write_to_file("contents/app/main_function.rs", format!("{:#?}",app.main_fn));

    //Extra
    //fs::write("contents/extra_print.rs", format!("{:#?}",extra))?;

}

fn create_folders(){
    match fs::create_dir("contents"){
        Ok(_) => {
            println!("Stor the AST from the first pars in the folder contents/");
            println!("Created folder contents");
        },
        Err(_) => {//folder already exists. 
        }
    }

    match fs::create_dir("contents/app"){
        Ok(_) => {
            println!("Created folder contents/app");
        },
        Err(_) => {//folder already exists. 
        }
    }
}

pub fn software_token_stream(software_output: &TokenStream2){

    
    create_folders();

    let print = format!("{:#?}",software_output);
    match fs::write("contents/software_output.rs", print){
        Ok(()) => {},
        Err(error) => println!("Problem opening the file: {:?}", error)
    }
}

fn write_to_file<P: std::convert::AsRef<std::path::Path>,C: std::convert::AsRef<[u8]>>(path: P, contents: C){
    match fs::write(path, contents){
        Ok(()) => {},
        Err(error) => println!("Problem opening the file: {:?}", error)
    }
}