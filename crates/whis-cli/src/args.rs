use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "whis")]
#[command(version)]
#[command(about = "Voice-to-text CLI using OpenAI Whisper or Mistral Voxtral")]
#[command(after_help = "Run 'whis' without arguments to record once (press Enter to stop).")]
pub struct Cli {
    /// Enable verbose output for debugging (audio device, FFmpeg, clipboard, etc.)
    #[arg(short, long, global = true)]
    pub verbose: bool,

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

    /// Configure settings (API keys, provider, etc.)
    Config {
        /// Set your OpenAI API key
        #[arg(long)]
        openai_api_key: Option<String>,

        /// Set your Mistral API key
        #[arg(long)]
        mistral_api_key: Option<String>,

        /// Set the transcription provider (openai or mistral)
        #[arg(long)]
        provider: Option<String>,

        /// Set the language hint (ISO-639-1 code: en, de, fr, etc.) or "auto" for auto-detect
        #[arg(long)]
        language: Option<String>,

        /// Show current configuration
        #[arg(long)]
        show: bool,
    },
}
