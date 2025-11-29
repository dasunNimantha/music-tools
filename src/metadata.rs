use crate::model::FileMetadata;
use lofty::config::WriteOptions;
use lofty::picture::Picture;
use lofty::prelude::*;
use lofty::tag::{Tag, TagType};
use std::fs;
use std::path::PathBuf;

pub fn remove_all_metadata(file_path: PathBuf) -> Result<(), String> {
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

pub fn set_metadata(
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

    for file_path in files {
        if let Err(e) = remove_all_metadata(file_path.clone()) {
            errors.push(format!("{}: {}", file_path.display(), e));
            continue;
        }

        if let Err(e) = set_metadata(
            file_path.clone(),
            artist.clone(),
            album.clone(),
            genre.clone(),
            year,
            album_art.clone(),
        ) {
            errors.push(format!("{}: {}", file_path.display(), e));
        }
    }

    if errors.is_empty() {
        Ok(vec![])
    } else {
        Err(errors.join("\n"))
    }
}
