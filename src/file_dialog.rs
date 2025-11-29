use std::path::PathBuf;
use walkdir::WalkDir;
use rfd::{FileDialog, AsyncFileDialog};
use crate::config::SUPPORTED_FORMATS;

pub fn select_image() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Image Files", &["jpg", "jpeg", "png", "bmp", "gif", "webp"])
        .set_directory(std::env::current_dir().unwrap_or_default())
        .pick_file()
}

pub fn select_files() -> Option<Vec<PathBuf>> {
    FileDialog::new()
        .add_filter("Audio Files", &["mp3", "flac", "m4a", "ogg", "wma", "aac", "mp4", "opus"])
        .set_directory(std::env::current_dir().unwrap_or_default())
        .pick_files()
}

pub async fn select_folder_dialog() -> Option<PathBuf> {
    let folder = AsyncFileDialog::new()
        .set_directory(std::env::current_dir().unwrap_or_default())
        .pick_folder()
        .await;
    
    folder.map(|handle| handle.path().to_path_buf())
}

pub async fn scan_folder_async(folder_path: PathBuf) -> Vec<PathBuf> {
    scan_folder_for_audio(&folder_path)
}

pub fn scan_folder_for_audio(folder_path: &PathBuf) -> Vec<PathBuf> {
    let mut audio_files = Vec::new();
    for entry in WalkDir::new(folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if SUPPORTED_FORMATS.contains(&ext_str.as_str()) {
                    audio_files.push(path.to_path_buf());
                }
            }
        }
    }
    audio_files
}

