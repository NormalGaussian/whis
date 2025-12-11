use anyhow::Result;
use std::io::Write;
use whis_core::{Settings, TranscriptionProvider};

/// Configuration for transcription, including provider, API key, and language
pub struct TranscriptionConfig {
    pub provider: TranscriptionProvider,
    pub api_key: String,
    pub language: Option<String>,
}

pub fn ensure_ffmpeg_installed() -> Result<()> {
    if std::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .is_err()
    {
        eprintln!("Error: FFmpeg is not installed or not in PATH.");
        eprintln!("\nwhis requires FFmpeg for audio compression.");
        eprintln!("Please install FFmpeg:");
        eprintln!("  - Ubuntu/Debian: sudo apt install ffmpeg");
        eprintln!("  - macOS: brew install ffmpeg");
        eprintln!("  - Windows: choco install ffmpeg or download from ffmpeg.org");
        eprintln!("  - Or visit: https://ffmpeg.org/download.html\n");
        std::process::exit(1);
    }
    Ok(())
}

pub fn load_transcription_config() -> Result<TranscriptionConfig> {
    let settings = Settings::load();
    let provider = settings.provider.clone();
    let language = settings.language.clone();

    // Load API key based on provider
    let api_key = match &provider {
        TranscriptionProvider::OpenAI => {
            // Priority: settings file > environment variable
            if let Some(key) = settings.openai_api_key {
                key
            } else {
                // Fallback to environment
                match std::env::var("OPENAI_API_KEY") {
                    Ok(key) => key,
                    Err(_) => {
                        eprintln!("Error: No OpenAI API key configured.");
                        eprintln!("\nSet your key with:");
                        eprintln!("  whis config --openai-api-key YOUR_KEY\n");
                        eprintln!("Or set the OPENAI_API_KEY environment variable.");
                        std::process::exit(1);
                    }
                }
            }
        }
        TranscriptionProvider::Mistral => {
            // Priority: settings file > environment variable
            if let Some(key) = settings.mistral_api_key {
                key
            } else {
                // Fallback to environment
                match std::env::var("MISTRAL_API_KEY") {
                    Ok(key) => key,
                    Err(_) => {
                        eprintln!("Error: No Mistral API key configured.");
                        eprintln!("\nSet your key with:");
                        eprintln!("  whis config --mistral-api-key YOUR_KEY\n");
                        eprintln!("Or set the MISTRAL_API_KEY environment variable.");
                        std::process::exit(1);
                    }
                }
            }
        }
    };

    Ok(TranscriptionConfig {
        provider,
        api_key,
        language,
    })
}

pub fn wait_for_enter() -> Result<()> {
    let mut input = String::new();
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}
