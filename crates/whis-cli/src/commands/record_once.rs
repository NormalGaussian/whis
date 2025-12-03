use anyhow::Result;
use std::io::{self, Write};
use whis_core::{
    AudioRecorder, RecordingOutput, copy_to_clipboard, parallel_transcribe,
    transcribe_audio,
};
use crate::app;

pub fn run() -> Result<()> {
    // Create Tokio runtime for async operations
    let runtime = tokio::runtime::Runtime::new()?;

    // Check if FFmpeg is available
    app::ensure_ffmpeg_installed()?;

    // Load API configuration
    let config = app::load_api_config()?;

    // Create recorder and start recording
    let mut recorder = AudioRecorder::new()?;
    recorder.start_recording()?;

    print!("Recording... (press Enter to stop)");
    io::stdout().flush()?;
    app::wait_for_enter()?;

    // Finalize recording and get output
    let audio_result = recorder.finalize_recording()?;

    // Transcribe based on output type
    let transcription = match audio_result {
        RecordingOutput::Single(audio_data) => {
            // Small file - simple transcription
            print!("\rTranscribing...                        \n");
            io::stdout().flush()?;

            match transcribe_audio(&config.openai_api_key, audio_data) {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("Transcription error: {e}");
                    std::process::exit(1);
                }
            }
        }
        RecordingOutput::Chunked(chunks) => {
            // Large file - parallel transcription
            print!("\rTranscribing...                        \n");
            io::stdout().flush()?;

            runtime.block_on(async {
                match parallel_transcribe(&config.openai_api_key, chunks, None).await {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!("Transcription error: {e}");
                        std::process::exit(1);
                    }
                }
            })
        }
    };

    // Copy to clipboard
    copy_to_clipboard(&transcription)?;

    println!("Copied to clipboard");

    Ok(())
}
