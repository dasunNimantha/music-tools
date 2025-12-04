use crate::model::FileMetadata;
use lofty::config::WriteOptions;
use lofty::picture::Picture;
use lofty::prelude::*;
use lofty::tag::{Tag, TagType};
use std::fs;
use std::panic;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tokio::time::timeout;

fn remove_all_metadata_inner(file_path: PathBuf) -> Result<(), String> {
    match lofty::read_from_path(&file_path) {
        Ok(mut tagged_file) => {
            tagged_file.clear();
            use lofty::file::AudioFile;
            match tagged_file.save_to_path(&file_path, WriteOptions::default()) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Error saving file: {}", e)),
            }
        }
        Err(e) => Err(format!("Error reading file: {}", e)),
    }
}

pub fn remove_all_metadata(file_path: PathBuf) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();
    let path = file_path.clone();

    let handle = thread::spawn(move || {
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {})); // Suppress panic output

        let result =
            panic::catch_unwind(panic::AssertUnwindSafe(|| remove_all_metadata_inner(path)));

        panic::set_hook(old_hook);

        let _ = tx.send(result);
    });

    match rx.recv_timeout(Duration::from_secs(30)) {
        Ok(Ok(Ok(()))) => {
            let _ = handle.join();
            Ok(())
        }
        Ok(Ok(Err(e))) => {
            let _ = handle.join();
            Err(e)
        }
        Ok(Err(_panic_info)) => {
            let _ = handle.join();
            // Don't try to extract panic message as it may contain invalid UTF-8
            // Just report that the file has corrupted metadata
            Err("File has corrupted metadata (encoding issue)".to_string())
        }
        Err(_) => {
            let _ = handle.join();
            Err("Timeout or thread communication error while processing file".to_string())
        }
    }
}

fn set_metadata_inner(
    file_path: PathBuf,
    artist: String,
    album: String,
    genre: Option<String>,
    year: Option<u32>,
    album_art: Option<PathBuf>,
) -> Result<(), String> {
    match lofty::read_from_path(&file_path) {
        Ok(mut tagged_file) => {
            if tagged_file.primary_tag().is_none() {
                tagged_file.insert_tag(Tag::new(TagType::Id3v2));
            }

            if let Some(tag) = tagged_file.primary_tag_mut() {
                if !artist.is_empty() {
                    tag.set_artist(artist);
                }
                if !album.is_empty() {
                    tag.set_album(album);
                }
                if let Some(g) = genre {
                    if !g.is_empty() {
                        tag.set_genre(g);
                    }
                }
                if let Some(y) = year {
                    tag.set_year(y);
                }

                if let Some(art_path) = album_art {
                    match fs::read(&art_path) {
                        Ok(image_data) => {
                            let mime_type = art_path
                                .extension()
                                .and_then(|ext| ext.to_str())
                                .map(|ext| match ext.to_lowercase().as_str() {
                                    "jpg" | "jpeg" => "image/jpeg",
                                    "png" => "image/png",
                                    "bmp" => "image/bmp",
                                    "gif" => "image/gif",
                                    "webp" => "image/webp",
                                    _ => "image/jpeg",
                                })
                                .unwrap_or("image/jpeg");

                            use lofty::picture::{MimeType, PictureType};
                            let mime = match mime_type {
                                "image/jpeg" => MimeType::Jpeg,
                                "image/png" => MimeType::Png,
                                "image/bmp" => MimeType::Bmp,
                                "image/gif" => MimeType::Gif,
                                _ => MimeType::Jpeg,
                            };
                            let picture = Picture::new_unchecked(
                                PictureType::CoverFront,
                                Some(mime),
                                None,
                                image_data,
                            );
                            while tag.picture_count() > 0 {
                                tag.remove_picture(0);
                            }
                            tag.set_picture(0, picture);
                        }
                        Err(e) => return Err(format!("Error reading image file: {}", e)),
                    }
                }
            }

            use lofty::file::AudioFile;
            match tagged_file.save_to_path(&file_path, WriteOptions::default()) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Error saving file: {}", e)),
            }
        }
        Err(e) => Err(format!("Error reading file: {}", e)),
    }
}

pub fn set_metadata(
    file_path: PathBuf,
    artist: String,
    album: String,
    genre: Option<String>,
    year: Option<u32>,
    album_art: Option<PathBuf>,
) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();
    let path = file_path.clone();
    let artist_clone = artist.clone();
    let album_clone = album.clone();
    let genre_clone = genre.clone();
    let art_clone = album_art.clone();

    let handle = thread::spawn(move || {
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {})); // Suppress panic output

        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            set_metadata_inner(
                path,
                artist_clone,
                album_clone,
                genre_clone,
                year,
                art_clone,
            )
        }));

        panic::set_hook(old_hook);

        let _ = tx.send(result);
    });

    match rx.recv_timeout(Duration::from_secs(30)) {
        Ok(Ok(Ok(()))) => {
            let _ = handle.join();
            Ok(())
        }
        Ok(Ok(Err(e))) => {
            let _ = handle.join();
            Err(e)
        }
        Ok(Err(_panic_info)) => {
            let _ = handle.join();
            // Don't try to extract panic message as it may contain invalid UTF-8
            // Just report that the file has corrupted metadata
            Err("File has corrupted metadata (encoding issue)".to_string())
        }
        Err(_) => {
            let _ = handle.join();
            Err("Timeout or thread communication error while processing file".to_string())
        }
    }
}

pub fn read_file_metadata(file_path: PathBuf) -> Result<FileMetadata, String> {
    match lofty::read_from_path(&file_path) {
        Ok(tagged_file) => {
            let mut metadata = FileMetadata::default();

            let props = tagged_file.properties();
            metadata.duration = Some(props.duration().as_secs());
            metadata.bitrate = props.audio_bitrate();
            metadata.sample_rate = props.sample_rate();
            metadata.channels = props.channels();

            let ext = file_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown")
                .to_uppercase();
            metadata.format = ext;

            if let Some(tag) = tagged_file.primary_tag() {
                metadata.artist = tag.artist().map(|s| s.to_string()).unwrap_or_default();
                metadata.album = tag.album().map(|s| s.to_string()).unwrap_or_default();
                metadata.title = tag.title().map(|s| s.to_string()).unwrap_or_default();
                metadata.year = tag.year();
                metadata.genre = tag.genre().map(|s| s.to_string()).unwrap_or_default();
                metadata.track = tag.track();
            }

            Ok(metadata)
        }
        Err(e) => Err(format!("Error reading file: {}", e)),
    }
}

pub async fn process_files(
    files: Vec<PathBuf>,
    artist: String,
    album: String,
    genre: Option<String>,
    year: Option<u32>,
    album_art: Option<PathBuf>,
) -> Result<Vec<String>, String> {
    let mut errors = Vec::new();
    const FILE_TIMEOUT: Duration = Duration::from_secs(30);

    for file_path in files {
        let file_display = file_path.display().to_string();

        // Process remove_all_metadata with timeout
        let remove_result = timeout(
            FILE_TIMEOUT,
            tokio::task::spawn_blocking({
                let path = file_path.clone();
                move || remove_all_metadata(path)
            }),
        )
        .await;

        match remove_result {
            Ok(Ok(Ok(()))) => {}
            Ok(Ok(Err(e))) => {
                errors.push(format!("{}: {}", file_display, e));
                continue;
            }
            Ok(Err(e)) => {
                let error_msg = if e.is_panic() {
                    format!(
                        "{}: Processing failed due to encoding/metadata corruption issue",
                        file_display
                    )
                } else {
                    format!("{}: Task error: {}", file_display, e)
                };
                errors.push(error_msg);
                continue;
            }
            Err(_) => {
                errors.push(format!(
                    "{}: Timeout while removing metadata (exceeded {}s)",
                    file_display,
                    FILE_TIMEOUT.as_secs()
                ));
                continue;
            }
        }

        // Process set_metadata with timeout
        let set_result = timeout(
            FILE_TIMEOUT,
            tokio::task::spawn_blocking({
                let path = file_path.clone();
                let artist_clone = artist.clone();
                let album_clone = album.clone();
                let genre_clone = genre.clone();
                let art_clone = album_art.clone();
                move || {
                    set_metadata(
                        path,
                        artist_clone,
                        album_clone,
                        genre_clone,
                        year,
                        art_clone,
                    )
                }
            }),
        )
        .await;

        match set_result {
            Ok(Ok(Ok(()))) => {}
            Ok(Ok(Err(e))) => {
                errors.push(format!("{}: {}", file_display, e));
            }
            Ok(Err(e)) => {
                let error_msg = if e.is_panic() {
                    format!(
                        "{}: Processing failed due to encoding/metadata corruption issue",
                        file_display
                    )
                } else {
                    format!("{}: Task error: {}", file_display, e)
                };
                errors.push(error_msg);
            }
            Err(_) => {
                errors.push(format!(
                    "{}: Timeout while setting metadata (exceeded {}s)",
                    file_display,
                    FILE_TIMEOUT.as_secs()
                ));
            }
        }
    }

    Ok(errors)
}
