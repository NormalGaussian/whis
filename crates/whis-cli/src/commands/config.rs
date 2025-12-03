use anyhow::Result;
use whis_core::Settings;

pub fn run(api_key: Option<String>, show: bool) -> Result<()> {
    if let Some(key) = api_key {
        // Validate format
        if !key.starts_with("sk-") {
            eprintln!("Invalid key format. OpenAI keys start with 'sk-'");
            std::process::exit(1);
        }

        let mut settings = Settings::load();
        settings.openai_api_key = Some(key);
        settings.save()?;
        println!("API key saved to {}", Settings::path().display());
        return Ok(());
    }

    if show {
        let settings = Settings::load();
        println!("Config file: {}", Settings::path().display());
        println!("Shortcut: {}", settings.shortcut);
        if let Some(key) = &settings.openai_api_key {
            let masked = if key.len() > 10 {
                format!("{}...{}", &key[..6], &key[key.len() - 4..])
            } else {
                "***".to_string()
            };
            println!("API key: {masked}");
        } else {
            println!("API key: (not set, using $OPENAI_API_KEY)");
        }
        return Ok(());
    }

    // No flags - show help
    eprintln!("Usage: whis config --api-key <KEY>");
    eprintln!("       whis config --show");
    std::process::exit(1);
}
