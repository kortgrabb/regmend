use regmand::{cli, file_io, regex_processor};
use std::process;

fn main() {
    let args = cli::get_matches();

    // Get the arguments
    let path = args.file_name.as_str();
    let pattern = args.pattern.as_str();
    let replacement = args.new_text.as_str();

    // The text to replace
    let to_replace = match file_io::read_file(path) {
        Ok(content) => content,
        Err(_) => {
            println!("that file does not exist!");
            process::exit(1);
        }
    };

    // The new text with regex replacement
    let new_text = match regex_processor::process_text(to_replace.as_str(), pattern, replacement) {
        Ok(res) => res,
        Err(e) => {
            println!("invalid regex!\nError: {}", e);
            process::exit(1);
        }
    };

    // Determine whether to make a new file or not
    let mut path_to_write = String::new();
    if args.overwrite {
        path_to_write = "replaced-".to_string() + path;
    } else {
        path_to_write = path.to_string();
    }
    if let Err(e) = file_io::write_file(path_to_write.as_str(), &new_text) {
        eprintln!("Error writing to file: {}", e);
        process::exit(1);
    }

    // Print the information
    println!("Regex replace successful!");
    if args.echo {
        println!("old: {}", to_replace);
        println!("new: {}", new_text);
    }
}
