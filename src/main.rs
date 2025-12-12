use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_contents = read_file(&args);
    println!("With text:\n{file_contents}")
}

fn read_file(args: &[String]) -> String {
    let file_path = &args[1];
    
    fs::read_to_string(file_path).expect("read file fail")
}
