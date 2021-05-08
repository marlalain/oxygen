use std::{env, fs};
use seahorse::{App, Context, Flag, FlagType, error::FlagError};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description("Prints the content of files")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cat file1 [files2+]")
        .action(cat)
        .flag(Flag::new("verbose", FlagType::Bool)
              .description("Becomes more verbose")
              .alias("v"),
        )
        ;
    app.run(args);
}

fn cat(c: &Context) {
    for arg in &c.args {
        let content = fs::read_to_string(arg);
        if c.bool_flag("verbose") {
            println!("Opening file {:?}", &arg);
        }
        match content {
            Ok(ok) => { print!("{}", ok); }
            Err(..) => { panic!(); }
        }
    }
}
