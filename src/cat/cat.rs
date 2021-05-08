use std::env;
use std::fs;

fn main() {
    let mut arguments: Vec<String> = std::env::args().collect();
    arguments.remove(0);

    for argument in &arguments {
        let content = fs::read_to_string(argument).unwrap();
        println!("{}", content);
    }
}
