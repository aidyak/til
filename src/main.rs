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

fn open_in_nvim(path: &Path) -> Result<()> {
    Command::new("nvim")
        .arg(path)
        .status()
        .with_context(|| format!("Failed to run Neovim: {}", path.display()))?;
    Ok(())
}

fn search_markdown_contents(dir: &Path, pattern: &str) -> Result<()> {
    run_rg(
        dir,
        &[
            "--line-number",
            "--color",
            "never",
            "--glob",
            "*.md",
            pattern,
            ".",
        ],
        "markdown contents",
    )
}

fn search_markdown_files(dir: &Path, pattern: &str) -> Result<()> {
    run_rg(
        dir,
        &[
            "--files",
            "--glob",
            "*.md",
            "--iglob",
            &format!("*{pattern}*"),
        ],
        "markdown files",
    )
}

fn run_rg(dir: &Path, args: &[&str], target: &str) -> Result<()> {
    let status = Command::new("rg")
        .args(args)
        .current_dir(dir)
        .status()
        .with_context(|| format!("Failed to run ripgrep for {target} in {}", dir.display()))?;

    match status.code() {
        Some(0) | Some(1) => Ok(()),
        Some(code) => Err(anyhow::anyhow!(
            "ripgrep exited with status code {code} while searching {target}"
        )),
        None => Err(anyhow::anyhow!(
            "ripgrep terminated unexpectedly while searching {target}"
        )),
    }
}
