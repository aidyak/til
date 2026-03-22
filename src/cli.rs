use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "til")]
#[command(about = "指定ディレクトリに今日の日付のmarkdownを作成して開く")]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(value_name = "DIR")]
    pub dir: Option<PathBuf>,

    #[arg(long)]
    pub file: bool,

    #[arg(long, conflicts_with = "files")]
    pub grep: Option<String>,

    #[arg(long, conflicts_with = "grep")]
    pub files: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Setup {
        #[arg(value_name = "DIR")]
        dir: PathBuf,
    },
}
