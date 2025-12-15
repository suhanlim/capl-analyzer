use std::env;
use std::process;
use capl_analyzer::read_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_contents = read_file(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    println!("With text:\n{file_contents}")
}

