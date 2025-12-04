use crate::config::SUPPORTED_FORMATS;
use rfd::{AsyncFileDialog, FileDialog};
use std::path::PathBuf;
use walkdir::WalkDir;

pub async fn select_image_async() -> Option<PathBuf> {
    let mut dialog = AsyncFileDialog::new()
        .add_filter("Image Files", &["jpg", "jpeg", "png", "bmp", "gif", "webp"]);

    dialog = dialog.set_directory(std::env::current_dir().unwrap_or_default());

    let file = dialog.pick_file().await;
    file.map(|handle| handle.path().to_path_buf())
}

pub fn select_files() -> Option<Vec<PathBuf>> {
    FileDialog::new()
        .add_filter(
            "Audio Files",
            &["mp3", "flac", "m4a", "ogg", "wma", "aac", "mp4", "opus"],
        )
        .set_directory(std::env::current_dir().unwrap_or_default())
        .pick_files()
}

pub async fn select_folder_dialog(initial_path: Option<PathBuf>) -> Option<PathBuf> {
    let mut dialog = AsyncFileDialog::new();
    if let Some(path) = initial_path {
        dialog = dialog.set_directory(&path);
    } else {
        dialog = dialog.set_directory(std::env::current_dir().unwrap_or_default());
    }
    let folder = dialog.pick_folder().await;
    folder.map(|handle| handle.path().to_path_buf())
}

pub async fn scan_folder_async(folder_path: PathBuf) -> Vec<PathBuf> {
    scan_folder_for_audio(&folder_path)
}

pub fn scan_folder_for_audio(folder_path: &PathBuf) -> Vec<PathBuf> {
    let mut audio_files = Vec::new();
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
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
