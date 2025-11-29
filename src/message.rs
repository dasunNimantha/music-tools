use crate::model::{FileMetadata, Screen};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    NavigateTo(Screen),
    GoHome,

    // Metadata Editor
    SelectFiles,
    SelectFolder,
    FolderSelected(Option<PathBuf>),
    StartFolderScan,
    FilesSelected(Vec<PathBuf>),
    ArtistChanged(String),
    AlbumChanged(String),
    GenreChanged(String),
    YearChanged(String),
    SelectImage,
    ImageSelected(Option<PathBuf>),
    ProcessFiles,
    ProcessingComplete(Result<Vec<String>, String>),
    RemoveFile(usize),
    ClearAllFiles,
    FileSelected(usize),
    MetadataLoaded(usize, Result<FileMetadata, String>),

    // Music Downloader (placeholder)
    DownloadUrlChanged(String),
    StartDownload,

    // Audio Converter (placeholder)
    SelectConvertFiles,
    ConvertFormatChanged(String),
    StartConvert,

    // Theme
    ToggleTheme,

    // Animation
    Tick(Instant),
}
