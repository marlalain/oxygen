use seahorse::{App, Context, Flag, FlagType};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("rm")
        .description("Deletes a folder or a file")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("rm file1 [files2+]")
        .action(|c| match rm(c) {
            Ok(()) => std::process::exit(1),
            Err(e) => println!("Could not delete file: {:?}", e),
        })
        .flag(
            Flag::new("verbose", FlagType::Bool)
                .description("Becomes more verbose")
                .alias("v"),
        )
        .flag(
            Flag::new("recursive", FlagType::Bool)
                .description("Remove files and folders")
                .alias("r"),
        );
    app.run(args);
}

fn rm(c: &Context) -> std::io::Result<()> {
    for arg in &c.args {
        if c.bool_flag("recursive") {
            fs::remove_dir_all(arg)?;
            if c.bool_flag("verbose") {
                println!("Removed folder {:?}", &arg);
            }
        } else {
            fs::remove_file(arg)?;
            if c.bool_flag("verbose") {
                println!("Removed file {:?}", &arg);
            }
        }
    }
    Ok(())
}
