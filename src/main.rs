use std::env;
use std::process;
mod reader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_contents = reader::run(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    println!("With text:\n{file_contents}")
}
