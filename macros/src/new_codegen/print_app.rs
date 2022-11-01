
use rtic_syntax::ast::App;
use std::fs;


// saves/prints information about app to the folder contents/ and contents/app/

pub fn print_app(app: &App) -> std::io::Result<()>{
    let contents_app = format!("{:#?}",app);
    fs::write("contents/app.rs", contents_app)?;

    let contents_app_args = format!("{:#?}",app.args);
    fs::write("contents/app/args.rs", contents_app_args)?;

    let contents_app_name = format!("{:#?}",app.name);
    fs::write("contents/app/name.rs", contents_app_name)?;

    let contents_app_init = format!("{:#?}",app.init);
    fs::write("contents/app/init.rs", contents_app_init)?;

    let contents_app_idle = format!("{:#?}",app.idle);
    fs::write("contents/app/idle.rs", contents_app_idle)?;

    let contents_app_monotonics = format!("{:#?}",app.monotonics);
    fs::write("contents/app/monotonics.rs", contents_app_monotonics)?;

    let contents_app_shared_resources = format!("{:#?}",app.shared_resources);
    fs::write("contents/app/shared_resources.rs", contents_app_shared_resources)?;

    let contents_app_local_resources = format!("{:#?}",app.local_resources);
    fs::write("contents/app/local_resources.rs", contents_app_local_resources)?;

    let contents_app_user_imports = format!("{:#?}",app.user_imports);
    fs::write("contents/app/user_imports.rs", contents_app_user_imports)?;

    let contents_app_user_code = format!("{:#?}",app.user_code);
    fs::write("contents/app/user_code.rs", contents_app_user_code)?;

    let contents_app_hardware_tasks = format!("{:#?}",app.hardware_tasks);
    fs::write("contents/app/hardware_tasks.rs", contents_app_hardware_tasks)?;

    let contents_app_software = format!("{:#?}",app.software_tasks);
    fs::write("contents/app/software.rs", contents_app_software)?;

    let contents_app_user_imports = format!("{:#?}",app.user_imports);
    fs::write("contents/app/user_imports.rs", contents_app_user_imports)?;

    Ok(())
}