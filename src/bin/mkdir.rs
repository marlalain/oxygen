use seahorse::{App, Context, Flag, FlagType};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("mkdir")
        .description("Creates a folder")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("mkdir folder1 [folders2+]")
        .action(|c| match mkdir(c) {
            Ok(()) => std::process::exit(1),
            Err(e) => println!("Could not create folder: '{}'", e),
        })
        .flag(
            Flag::new("verbose", FlagType::Bool)
                .description("Becomes more verbose")
                .alias("v"),
        )
        .flag(
            Flag::new("parents", FlagType::Bool)
                .description("Create possible parent directories")
                .alias("p"),
        );
    app.run(args);
}

fn mkdir(c: &Context) -> std::io::Result<()> {
    for arg in &c.args {
        if c.bool_flag("parents") {
            fs::create_dir_all(arg)?;
        } else {
            fs::create_dir(arg)?;
        }
        if c.bool_flag("verbose") {
            println!("Creating folder '{:?}'", &arg);
        }
    }
    Ok(())
}
