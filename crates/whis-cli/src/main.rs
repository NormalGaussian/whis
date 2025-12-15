mod app;
mod args;
mod commands;
mod hotkey;
mod ipc;
mod service;

use anyhow::Result;
use clap::Parser;
use whis_core::set_verbose;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    // Enable verbose logging if requested
    set_verbose(cli.verbose);

    match cli.command {
        Some(args::Commands::Listen { hotkey }) => commands::listen::run(hotkey),
        Some(args::Commands::Stop) => commands::stop::run(),
        Some(args::Commands::Status) => commands::status::run(),
        Some(args::Commands::Config {
            openai_api_key,
            mistral_api_key,
            provider,
            language,
            show,
        }) => commands::config::run(openai_api_key, mistral_api_key, provider, language, show),
        None => commands::record_once::run(),
    }
}
