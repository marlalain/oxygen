use seahorse::{App, Context, Flag, FlagType};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("mv")
        .description("Move a file")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("mv file1 file2")
        .action(|c| match mv(c) {
            Ok(()) => std::process::exit(0),
            Err(e) => println!("Could not move file: {:?}", e),
        })
        .flag(
            Flag::new("verbose", FlagType::Bool)
                .description("Becomes more verbose")
                .alias("v"),
        );
    app.run(args);
}

fn mv(c: &Context) -> std::io::Result<()> {
    fs::rename(&c.args[0], &c.args[1])?;
    if c.bool_flag("verbose") {
        println!("Moving file {:?} to {:?}", &c.args[0], &c.args[1]);
    }
    Ok(())
}
