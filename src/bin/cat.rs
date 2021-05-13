use seahorse::{App, Context, Flag, FlagType};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("cat")
        .description("Prints the content of files")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cat file1 [files2+]")
        .action(|c| match cat(c) {
            Ok(()) => std::process::exit(1),
            Err(e) => println!("Could not read file: '{}'", e),
        })
        .flag(
            Flag::new("verbose", FlagType::Bool)
                .description("Becomes more verbose")
                .alias("v"),
        );
    app.run(args);
}

fn cat(c: &Context) -> std::io::Result<()> {
    for arg in &c.args {
        let content = fs::read_to_string(arg)?;
        if c.bool_flag("verbose") {
            println!("Opening file '{:?}'", &arg);
        }
        print!("{}", content);
    }
    Ok(())
}
