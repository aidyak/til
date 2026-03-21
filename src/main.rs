use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "til")]
#[command(about = "指定ディレクトリに今日の日付のmarkdownを作成して開く")]
struct Args {
    #[arg(value_name = "DIR", default_value = ".")]
    dir: PathBuf,

    #[arg(long)]
    file: bool,

    #[arg(long, value_name = "PATTERN", conflicts_with = "files")]
    grep: Option<String>,

    #[arg(long, value_name = "PATTERN", conflicts_with = "grep")]
    files: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let dir = normalize_dir(&args.dir)?;
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to make directory: {}", dir.display()))?;

    if let Some(pattern) = &args.grep {
        search_markdown_contents(&dir, pattern)?;
        return Ok(());
    }

    if let Some(pattern) = &args.files {
        search_markdown_files(&dir, pattern)?;
        return Ok(());
    }

    let file_path = build_today_til_path(&dir);
    create_til_if_not_exists(&file_path)?;

    if args.file {
        open_in_nvim(&file_path)?;
        println!("Opened file: {}", file_path.display());
    } else {
        open_in_nvim(&dir)?;
        println!("Opened directory: {}", dir.display());
        println!("TIL file: {}", file_path.display());
    }

    Ok(())
}
