mod app;
mod args;
mod commands;
mod hotkey;
mod ipc;
mod service;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    match cli.command {
        Some(args::Commands::Listen { hotkey }) => commands::listen::run(hotkey),
        Some(args::Commands::Stop) => commands::stop::run(),
        Some(args::Commands::Status) => commands::status::run(),
        Some(args::Commands::Config { api_key, show }) => commands::config::run(api_key, show),
        None => commands::record_once::run(),
    }
}
