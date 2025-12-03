use anyhow::Result;
use std::io::Write;
use whis_core::{ApiConfig, Settings};

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

pub fn load_api_config() -> Result<ApiConfig> {
    // Priority: settings file > environment variable
    let settings = Settings::load();
    if let Some(key) = settings.openai_api_key {
        return Ok(ApiConfig {
            openai_api_key: key,
        });
    }

    // Fallback to environment
    match ApiConfig::from_env() {
        Ok(cfg) => Ok(cfg),
        Err(_) => {
            eprintln!("Error: No API key configured.");
            eprintln!("\nSet your key with:");
            eprintln!("  whis config --api-key YOUR_KEY\n");
            eprintln!("Or set the OPENAI_API_KEY environment variable.");
            std::process::exit(1);
        }
    }
}

pub fn wait_for_enter() -> Result<()> {
    let mut input = String::new();
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}
