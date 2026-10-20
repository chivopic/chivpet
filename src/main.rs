mod animation;
mod cli;
mod daemon;
mod engine;
mod ipc;
mod model;
mod paths;
mod storage;
mod tui;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    cli::run_cli(cli).await
}
