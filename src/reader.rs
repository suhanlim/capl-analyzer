use std::fs;

pub fn run(args: &[String]) -> Result<String, &'static str> {
    if args.len() == 1{
        return Err("not input arguments");
    }

    let file_path = &args[1];
    
    let content = fs::read_to_string(file_path).expect("read file fail");

    Ok(content)
}
