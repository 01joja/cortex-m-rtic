
use rtic_syntax::ast::App;
use quote::quote;
use crate::{analyze::Analysis, check::Extra};
use std::fs;


/// saves/prints information about app, a̶n̶a̶l̶y̶s̶i̶s̶  and extra
/// to the corresponding folder in contents/
pub fn print_all(
    app: &App, 
    _analysis: &Analysis,
    extra: &Extra,
) -> std::io::Result<()>{
    //app
    fs::write("contents/app_print.rs", format!("{:#?}",app))?;

    fs::write("contents/app/args_print.rs", format!("{:#?}",app.args))?;

    fs::write("contents/app/name_print.rs", format!("{:#?}",app.name))?;

    fs::write("contents/app/init_print.rs", format!("{:#?}",app.init))?;

    fs::write("contents/app/idle_print.rs", format!("{:#?}",app.idle))?;

    fs::write("contents/app/monotonics_print.rs", format!("{:#?}",app.monotonics))?;

    fs::write("contents/app/shared_resources_print.rs", format!("{:#?}",app.shared_resources))?;

    fs::write("contents/app/local_resources_print.rs", format!("{:#?}",app.local_resources))?;
    
    fs::write("contents/app/user_imports_print.rs", format!("{:#?}",app.user_imports))?;
    
    fs::write("contents/app/user_code_print.rs", format!("{:#?}",app.user_code))?;

    fs::write("contents/app/hardware_tasks_print.rs", format!("{:#?}",app.hardware_tasks))?;

    fs::write("contents/app/software_task_print.rs", format!("{:#?}",app.software_tasks))?;

    fs::write("contents/app/user_imports_print.rs", format!("{:#?}",app.user_imports))?;

    //Extra
    fs::write("contents/extra_print.rs", format!("{:#?}",extra))?;

    Ok(())
}