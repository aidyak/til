use anyhow::Result;
use std::path::PathBuf;

use crate::cli::{Args, Commands};
use crate::config;
use crate::til;

pub fn run(args: Args) -> Result<()> {
    if let Some(Commands::Setup { dir }) = args.command {
        let dir = til::normalize_dir(&dir)?;
        config::save_base_dir(&dir)?;
        println!("Saved base directory: {}", dir.display());
        return Ok(());
    }

    let dir = resolve_target_dir(args.dir)?;

    if let Some(pattern) = &args.grep {
        return til::search_markdown_contents(&dir, pattern);
    }

    if let Some(pattern) = &args.files {
        return til::search_markdown_files(&dir, pattern);
    }

    til::open_or_create_today(&dir, args.file)
}

fn resolve_target_dir(cli_dir: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(dir) = cli_dir {
        return til::normalize_dir(&dir);
    }

    if let Some(saved_dir) = config::load_base_dir()? {
        return Ok(saved_dir);
    }

    til::normalize_dir(PathBuf::from(".").as_path())
}
