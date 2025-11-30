use crate::utils::music_downloader::MusicDownloaderState;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Home,
    MetadataEditor,
    MusicDownloader,
    AudioConverter,
}

#[derive(Debug, Clone, Default)]
pub struct FileMetadata {
    pub artist: String,
    pub album: String,
    pub title: String,
    pub year: Option<u32>,
    pub genre: String,
    pub track: Option<u32>,
    pub duration: Option<u64>,
    pub format: String,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u8>,
}

pub struct AppState {
    pub current_screen: Screen,
    pub files: Vec<PathBuf>,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub year: String,
    pub album_art_path: Option<PathBuf>,
    pub status: String,
    pub error_logs: Vec<String>,
    pub processing: bool,
    pub loading_files: bool,
    pub loading_rotation: f32,
    pub pending_folder_scan: Option<PathBuf>,
    pub scan_delay_ticks: u32,
    pub selected_file_index: Option<usize>,
    pub file_metadata: HashMap<usize, FileMetadata>,
    pub last_metadata_folder: Option<PathBuf>,
    // Music Downloader state
    pub downloader_state: MusicDownloaderState,
    // Audio Converter state
    pub convert_format: String,
    pub convert_status: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_screen: Screen::Home,
            files: Vec::new(),
            artist: String::new(),
            album: String::new(),
            genre: String::new(),
            year: String::new(),
            album_art_path: None,
            status: "Ready to edit metadata".to_string(),
            error_logs: Vec::new(),
            processing: false,
            loading_files: false,
            loading_rotation: 0.0,
            pending_folder_scan: None,
            scan_delay_ticks: 0,
            selected_file_index: None,
            file_metadata: HashMap::new(),
            last_metadata_folder: None,
            downloader_state: MusicDownloaderState::new(),
            convert_format: "MP3".to_string(),
            convert_status: "Select files to convert".to_string(),
        }
    }
}
