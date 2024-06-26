use std::fs;
use std::io::{self, Read, Write};

pub fn read_file(path: &str) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

pub fn write_file(path: &str, contents: &str) -> io::Result<()> {
    match fs::write(path, contents) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
