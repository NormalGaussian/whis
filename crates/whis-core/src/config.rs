use serde::{Deserialize, Serialize};
use std::fmt;

/// Available transcription providers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TranscriptionProvider {
    #[default]
    OpenAI,
    Mistral,
}

impl fmt::Display for TranscriptionProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranscriptionProvider::OpenAI => write!(f, "openai"),
            TranscriptionProvider::Mistral => write!(f, "mistral"),
        }
    }
}

impl std::str::FromStr for TranscriptionProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(TranscriptionProvider::OpenAI),
            "mistral" => Ok(TranscriptionProvider::Mistral),
            _ => Err(format!("Unknown provider: {}. Use 'openai' or 'mistral'", s)),
        }
    }
}
