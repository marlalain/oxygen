use std::fs::File;
use std::env;

fn main() {
    let mut arguments: Vec<String> = std::env::args().collect();
    arguments.remove(0);

    for argument in &arguments {
        let mut _file = File::create(argument);
        println!("Created file '{}'.", argument);
    }
}
