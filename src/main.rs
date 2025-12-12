use std::env;
use capl_analyzer::read_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_contents = read_file(&args);
    println!("With text:\n{file_contents}")
}

