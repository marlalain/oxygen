use std::{fs, io, env};

fn main() -> io::Result<()> {
    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);

    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:#?}", dir.file_name());
    }

    Ok(())
}
