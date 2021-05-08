use std::fs;

fn main() {
    let mut arguments: Vec<String> = std::env::args().collect();
    arguments.remove(0);

    for argument in &arguments {
        fs::create_dir(argument).expect("Unable to create folder.");
    }

}
