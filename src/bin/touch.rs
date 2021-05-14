use seahorse::{App, Context, Flag, FlagType};
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("touch")
        .description("Creates an empty file")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("touch file1 [files2+]")
        .action(|c| match touch(c) {
            Ok(()) => std::process::exit(0),
            Err(e) => println!("Could not create file: {:?}", e),
        })
        .flag(
            Flag::new("verbose", FlagType::Bool)
                .description("Becomes more verbose")
                .alias("v"),
        );
    app.run(args);
}

fn touch(c: &Context) -> std::io::Result<()> {
    for arg in &c.args {
        let mut _file = File::create(arg)?;
        if c.bool_flag("verbose") {
            println!("Created file {:?}", arg);
        }
    }
    Ok(())
}
