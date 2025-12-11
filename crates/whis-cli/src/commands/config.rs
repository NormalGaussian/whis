use anyhow::Result;
use whis_core::{Settings, TranscriptionProvider};

pub fn run(
    openai_api_key: Option<String>,
    mistral_api_key: Option<String>,
    provider: Option<String>,
    language: Option<String>,
    show: bool,
) -> Result<()> {
    let mut settings = Settings::load();
    let mut changed = false;

    // Handle provider change
    if let Some(provider_str) = provider {
        match provider_str.parse::<TranscriptionProvider>() {
            Ok(p) => {
                settings.provider = p;
                changed = true;
                println!("Provider set to: {}", provider_str);
            }
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }

    // Handle language change
    if let Some(lang) = language {
        if lang.to_lowercase() == "auto" {
            settings.language = None;
            changed = true;
            println!("Language set to: auto-detect");
        } else {
            // Validate ISO-639-1 format: 2 lowercase alphabetic characters
            let lang_lower = lang.to_lowercase();
            if lang_lower.len() != 2 || !lang_lower.chars().all(|c| c.is_ascii_lowercase()) {
                eprintln!("Invalid language code. Use ISO-639-1 format (e.g., 'en', 'de', 'fr') or 'auto'");
                std::process::exit(1);
            }
            settings.language = Some(lang_lower.clone());
            changed = true;
            println!("Language set to: {}", lang_lower);
        }
    }

    // Handle OpenAI API key
    if let Some(key) = openai_api_key {
        // Validate format for OpenAI
        if !key.starts_with("sk-") {
            eprintln!("Invalid key format. OpenAI keys start with 'sk-'");
            std::process::exit(1);
        }

        settings.openai_api_key = Some(key);
        changed = true;
        println!("OpenAI API key saved");
    }

    // Handle Mistral API key (basic validation)
    if let Some(key) = mistral_api_key {
        let key_trimmed = key.trim();
        if key_trimmed.is_empty() {
            eprintln!("Invalid Mistral API key: cannot be empty");
            std::process::exit(1);
        }
        if key_trimmed.len() < 20 {
            eprintln!("Invalid Mistral API key: key appears too short");
            std::process::exit(1);
        }
        settings.mistral_api_key = Some(key_trimmed.to_string());
        changed = true;
        println!("Mistral API key saved");
    }

    // Save if anything changed
    if changed {
        settings.save()?;
        println!("Config saved to {}", Settings::path().display());
        return Ok(());
    }

    if show {
        println!("Config file: {}", Settings::path().display());
        println!("Provider: {}", settings.provider);
        println!(
            "Language: {}",
            settings.language.as_deref().unwrap_or("auto-detect")
        );
        println!("Shortcut: {}", settings.shortcut);

        // OpenAI API key
        if let Some(key) = &settings.openai_api_key {
            println!("OpenAI API key: {}", mask_key(key));
        } else {
            println!("OpenAI API key: (not set, using $OPENAI_API_KEY)");
        }

        // Mistral API key
        if let Some(key) = &settings.mistral_api_key {
            println!("Mistral API key: {}", mask_key(key));
        } else {
            println!("Mistral API key: (not set, using $MISTRAL_API_KEY)");
        }

        return Ok(());
    }

    // No flags - show help
    eprintln!("Usage:");
    eprintln!("  whis config --provider <openai|mistral>");
    eprintln!("  whis config --language <en|de|fr|...|auto>");
    eprintln!("  whis config --openai-api-key <KEY>");
    eprintln!("  whis config --mistral-api-key <KEY>");
    eprintln!("  whis config --show");
    std::process::exit(1);
}

fn mask_key(key: &str) -> String {
    if key.len() > 10 {
        format!("{}...{}", &key[..6], &key[key.len() - 4..])
    } else {
        "***".to_string()
    }
}
