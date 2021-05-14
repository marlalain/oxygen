use seahorse::{App, Context, Flag, FlagType};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("cp")
        .description("Copy a file")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cp file1 file2")
        .action(|c| match cp(c) {
            Ok(()) => std::process::exit(0),
            Err(e) => println!("Could not copy file: {:?}", e),
        })
        .flag(
            Flag::new("verbose", FlagType::Bool)
                .description("Becomes more verbose")
                .alias("v"),
        );
    app.run(args);
}

fn cp(c: &Context) -> std::io::Result<()> {
    fs::copy(&c.args[0], &c.args[1])?;
    if c.bool_flag("verbose") {
        println!("Copying file {:?} to {:?}", &c.args[0], &c.args[1]);
    }
    Ok(())
}
