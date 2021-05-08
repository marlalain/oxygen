use std::{env, fs, io};

fn main() -> io::Result<()> {
    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);

    for argument in &arguments {
        fs::create_dir(argument).expect("Unable to create folder.");
    }

    Ok(())
}
