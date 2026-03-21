use anyhow::Result;
use clap::Parser;
mod app;
mod cli;
mod til;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    app::run(args)
}
