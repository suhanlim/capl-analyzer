use std::fs;

pub fn read_file(args: &[String]) -> String {
    if args.len() == 1{
        panic!("not input file path");
    }

    let file_path = &args[1];
    
    fs::read_to_string(file_path).expect("read file fail")
}
