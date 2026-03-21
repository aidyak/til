use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "til")]
#[command(about = "指定ディレクトリに今日の日付のmarkdownを作成して開く")]
struct Args {
    #[arg(value_name = "DIR", default_value = ".")]
    dir: PathBuf,

    #[arg(long)]
    file: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let dir = normalize_dir(&args.dir)?;
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to make directory: {}", dir.display()))?;
    let file_path = build_today_til_path(&dir);
    create_til_if_not_exists(&file_path)?;

    if args.file {
        open::that(&file_path)
            .with_context(|| format!("Failed to open file: {}", file_path.display()))?;
        println!("Opened file: {}", file_path.display());
    } else {
        open::that(&dir).with_context(|| format!("Failed to open directory: {}", dir.display()))?;
        println!("Opened directory: {}", dir.display());
        println!("TIL file: {}", file_path.display());
    }

    Ok(())
}

fn normalize_dir(dir: &Path) -> Result<PathBuf> {
    if dir.is_absolute() {
        Ok(dir.to_path_buf())
    } else {
        let cwd = std::env::current_dir().context("Failed to get current directory")?;
        Ok(cwd.join(dir))
    }
}

fn build_today_til_path(dir: &Path) -> PathBuf {
    let today = Local::now().format("%Y-%m-%d").to_string();
    dir.join(format!("{today}-til.md"))
}

fn create_til_if_not_exists(file_path: &Path) -> Result<()> {
    if file_path.exists() {
        return Ok(());
    }

    let date = Local::now().format("%Y-%m-%d").to_string();
    let content = build_til_content(&date);

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_path)
        .with_context(|| format!("ファイルを作成できませんでした: {}", file_path.display()))?;

    fs::write(file_path, content)
        .with_context(|| format!("ファイルに書き込めませんでした: {}", file_path.display()))?;

    Ok(())
}

fn build_til_content(date: &str) -> String {
    format!(
        r#"# TIL - {date}

## Summary
- 

## Details
- 

## Code / Example

## Notes
- 

## Next
-
"#
    )
}
