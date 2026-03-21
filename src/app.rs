use anyhow::Result;

use crate::cli::Args;
use crate::til;

pub fn run(args: Args) -> Result<()> {
    if let Some(pattern) = &args.grep {
        return til::search_markdown_contents(&args.dir, pattern);
    }

    if let Some(pattern) = &args.files {
        return til::search_markdown_files(&args.dir, pattern);
    }

    til::open_or_create_today(&args.dir, args.file)
}
