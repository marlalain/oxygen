use std::{env, fs, io};

fn main() -> io::Result<()> {
    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);

    for argument in &arguments {
        let content = fs::read_to_string(argument)?;
        println!("{}", content);
    }

    Ok(())
}
