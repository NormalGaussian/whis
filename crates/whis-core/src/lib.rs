pub mod audio;
pub mod clipboard;
pub mod config;
pub mod settings;
pub mod transcribe;
pub mod verbose;

pub use audio::{AudioChunk, AudioRecorder, RecordingData, RecordingOutput};
pub use clipboard::copy_to_clipboard;
pub use config::TranscriptionProvider;
pub use settings::Settings;
pub use transcribe::{parallel_transcribe, transcribe_audio, ChunkTranscription};
pub use verbose::set_verbose;
