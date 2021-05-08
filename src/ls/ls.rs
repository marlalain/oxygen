use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let mut arguments: Vec<String> = std::env::args().collect();
    arguments.remove(0);

    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:#?}", dir.file_name());
    }

    Ok(())
}
