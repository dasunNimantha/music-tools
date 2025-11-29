// Metadata Editor Utility
// Handles editing artist, album, genre, year, and cover art for music files

use std::path::PathBuf;
use std::collections::HashMap;

/// Metadata information for a single audio file
#[derive(Debug, Clone, Default)]
pub struct FileMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub year: Option<u32>,
    pub duration: Option<u64>,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub has_cover: bool,
}

/// State specific to the metadata editor utility
#[derive(Debug, Clone, Default)]
pub struct MetadataEditorState {
    pub files: Vec<PathBuf>,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub year: String,
    pub album_art_path: Option<PathBuf>,
    pub selected_file_index: Option<usize>,
    pub file_metadata: HashMap<usize, FileMetadata>,
    pub loading_files: bool,
    pub loading_rotation: f32,
    pub pending_folder_scan: Option<PathBuf>,
    pub scan_delay_ticks: u32,
    pub processing: bool,
    pub status: String,
}

impl MetadataEditorState {
    pub fn new() -> Self {
        Self {
            status: "Ready to edit metadata".to_string(),
            ..Default::default()
        }
    }
    
    pub fn clear_files(&mut self) {
        self.files.clear();
        self.file_metadata.clear();
        self.selected_file_index = None;
        self.status = "All files cleared".to_string();
    }
    
    pub fn remove_file(&mut self, index: usize) {
        if index < self.files.len() {
            if let Some(selected) = self.selected_file_index {
                if selected == index {
                    self.selected_file_index = None;
                } else if selected > index {
                    self.selected_file_index = Some(selected - 1);
                }
            }
            self.files.remove(index);
            
            // Reindex metadata
            let old_metadata: Vec<_> = (0..self.files.len() + 1)
                .filter_map(|i| {
                    if i < index {
                        self.file_metadata.get(&i).map(|m| (i, m.clone()))
                    } else if i > index {
                        self.file_metadata.get(&i).map(|m| (i - 1, m.clone()))
                    } else {
                        None
                    }
                })
                .collect();
            self.file_metadata.clear();
            for (i, meta) in old_metadata {
                self.file_metadata.insert(i, meta);
            }
            self.status = format!("{} file(s) loaded", self.files.len());
        }
    }
}

