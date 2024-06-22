use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The file to use as reference
    #[arg(short, long)]
    pub file_name: String,

    /// The regex pattern, use '\' to use eg: quotes
    #[arg(short, long)]
    pub pattern: String,

    /// The text to place in the match
    #[arg(short, long)]
    pub new_text: String,

    /// Whether to overwrite the file or make a new one
    #[arg(short, long)]
    pub overwrite: bool,

    /// Whether to echo the new and old text (does not care about file-size)
    #[arg(short, long)]
    pub echo: bool,
}

pub fn get_matches() -> Args {
    let args = Args::parse();
    args
}
