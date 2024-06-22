use clap::{Arg, Parser};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub file_name: String,
    #[arg(short, long)]
    pub regex: String,
    #[arg(short, long)]
    pub new: String,
}

// regextool -i "input-file" -m "regex" -r "replace"
pub fn get_matches() -> Args {
    let args = Args::parse();
    args
}
