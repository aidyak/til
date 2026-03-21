use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "til")]
pub struct Args {
    #[arg(value_name = "DIR", default_value = ".")]
    pub dir: PathBuf,

    #[arg(long)]
    pub file: bool,

    #[arg(long, conflicts_with = "files")]
    pub grep: Option<String>,

    #[arg(long, conflicts_with = "grep")]
    pub files: Option<String>,

    #[arg(long)]
    pub pretty: bool,
}
