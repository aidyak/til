use anyhow::{Context, Result};
use chrono::Local;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn open_or_create_today(dir: &Path, open_file: bool) -> Result<()> {
    let dir = normalize_dir(dir)?;
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to make directory: {}", dir.display()))?;

    let file_path = build_today_til_path(&dir);
    create_til_if_not_exists(&file_path)?;

    if open_file {
        open_in_nvim(&file_path)?;
        println!("Opened file: {}", file_path.display());
    } else {
        open_in_nvim(&dir)?;
        println!("Opened directory: {}", dir.display());
        println!("TIL file: {}", file_path.display());
    }

    Ok(())
}

pub fn normalize_dir(dir: &Path) -> Result<PathBuf> {
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

pub fn search_markdown_contents(dir: &Path, pattern: &str) -> Result<()> {
    let dir = normalize_dir(dir)?;
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to make directory: {}", dir.display()))?;

    run_rg(
        &dir,
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

pub fn search_markdown_files(dir: &Path, pattern: &str) -> Result<()> {
    let dir = normalize_dir(dir)?;
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to make directory: {}", dir.display()))?;

    run_rg(
        &dir,
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
