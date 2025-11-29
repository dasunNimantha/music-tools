// Audio Converter Utility
// Convert audio files between different formats (placeholder)

use std::path::PathBuf;

/// Supported output formats
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum OutputFormat {
    #[default]
    MP3,
    FLAC,
    WAV,
    OGG,
    AAC,
}

impl OutputFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::MP3 => "MP3",
            OutputFormat::FLAC => "FLAC",
            OutputFormat::WAV => "WAV",
            OutputFormat::OGG => "OGG",
            OutputFormat::AAC => "AAC",
        }
    }
    
    pub fn all() -> &'static [OutputFormat] {
        &[
            OutputFormat::MP3,
            OutputFormat::FLAC,
            OutputFormat::WAV,
            OutputFormat::OGG,
            OutputFormat::AAC,
        ]
    }
}

/// State specific to the audio converter utility
#[derive(Debug, Clone, Default)]
pub struct AudioConverterState {
    pub files: Vec<PathBuf>,
    pub output_format: OutputFormat,
    pub status: String,
    pub converting: bool,
    pub progress: f32,
}

impl AudioConverterState {
    pub fn new() -> Self {
        Self {
            status: "Select files to convert".to_string(),
            ..Default::default()
        }
    }
}

