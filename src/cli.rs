use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub file_name: String,
    #[arg(short, long)]
    pub pattern: String,
    #[arg(short, long)]
    pub new_text: String,
    #[arg(short, long)]
    pub overwrite: bool,
    #[arg(short, long)]
    pub echo: bool,
}

pub fn get_matches() -> Args {
    let args = Args::parse();
    args
}
