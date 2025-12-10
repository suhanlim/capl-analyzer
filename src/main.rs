use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _file_path = parse_config(&args);

    println!("File {}", _file_path);

    let contents = fs::read_to_string(_file_path).expect("read file");

    println!("With text:\n{contents}")
}

fn parse_config(args: &[String]) -> &str {
    &args[1]
}
