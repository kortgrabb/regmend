use std::{fs, process};
use regmand::{cli, file_io, regex_processor};

fn main() {
    let args = cli::get_matches();

    let path = args.file_name.as_str();
    let pattern = args.regex.as_str();
    let replacement = args.new.as_str();

    let to_replace = match file_io::read_file(path) {
        Ok(content) => content,
        Err(_) => {
            println!("that file does not exist!");
            process::exit(1);
        }
    };

    let new_text = match regex_processor::process_text(to_replace.as_str(), pattern, replacement) {
        Ok(res) => res,
        Err(e) => {
            println!("invalid regex!\nError: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = file_io::write_file(path, &new_text) {
        eprintln!("Error writing to file: {}", e);
        process::exit(1);
    }

    println!("regex replace successful!")
}
