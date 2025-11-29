//! Music Tools Library
//!
//! A multi-utility suite for music file management featuring:
//! - Metadata Editor: Edit artist, album, genre, year, and cover art
//! - Music Downloader: Download music from online sources (coming soon)
//! - Audio Converter: Convert between audio formats (coming soon)

pub mod app;
pub mod config;
pub mod file_dialog;
pub mod message;
pub mod metadata;
pub mod model;
pub mod theme;
pub mod utils;
pub mod view;

// Re-export main types for convenience
pub use app::MusicToolsApp;
pub use message::Message;
pub use model::{AppState, Screen};
pub use theme::ThemeMode;
