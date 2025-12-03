use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "whis")]
#[command(version)]
#[command(about = "Voice-to-text CLI using OpenAI Whisper API")]
#[command(after_help = "Run 'whis' without arguments to record once (press Enter to stop).")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the background service that listens for hotkey triggers
    Listen {
        /// Hotkey to trigger recording (e.g., "ctrl+shift+r")
        #[arg(short = 'k', long, default_value = "ctrl+shift+r")]
        hotkey: String,
    },

    /// Stop the background service
    Stop,

    /// Check service status
    Status,

    /// Configure settings (API key, etc.)
    Config {
        /// Set your OpenAI API key
        #[arg(long)]
        api_key: Option<String>,

        /// Show current configuration
        #[arg(long)]
        show: bool,
    },
}
