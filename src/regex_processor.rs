use regex::Regex;

pub fn process_text(text: &str, pattern: &str, replacement: &str) -> Result<String, regex::Error> {
    match Regex::new(pattern) {
        Ok(re) => Ok(re.replace_all(text, replacement).to_string()),
        Err(e) => Err(e),
    }
}