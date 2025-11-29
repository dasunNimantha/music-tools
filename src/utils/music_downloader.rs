// Music Downloader Utility
// Download music from various online sources (placeholder)

/// State specific to the music downloader utility
#[derive(Debug, Clone, Default)]
pub struct MusicDownloaderState {
    pub url: String,
    pub status: String,
    pub downloading: bool,
    pub progress: f32,
}

impl MusicDownloaderState {
    pub fn new() -> Self {
        Self {
            status: "Enter a URL to download".to_string(),
            ..Default::default()
        }
    }
}

