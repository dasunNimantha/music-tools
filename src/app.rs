use crate::file_dialog::{scan_folder_async, select_files, select_image_async};
use crate::message::Message;
use crate::metadata::{process_files, read_file_metadata};
use crate::model::{AppState, Screen};
use crate::settings::AppSettings;
use crate::theme::{cosmic_theme, ThemeMode};
use crate::utils::audio_player;
use crate::utils::scraper::SongHubScraper;
use crate::view::build_view;
use iced::time;
use iced::{Application, Command, Subscription, Theme};
use std::time::Duration;

pub struct MusicToolsApp {
    state: AppState,
    theme_mode: ThemeMode,
}

impl Application for MusicToolsApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let settings = AppSettings::load();
        let mut state = AppState::default();

        // Load saved paths
        if let Some(path) = settings.get_download_directory() {
            state.downloader_state.download_path = Some(path);
        }
        if let Some(path) = settings.get_metadata_folder() {
            state.last_metadata_folder = Some(path);
        }

        (
            Self {
                state,
                theme_mode: ThemeMode::Dark,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        match self.state.current_screen {
            Screen::Home => "Music Tools".to_string(),
            Screen::MetadataEditor => "Metadata Editor - Music Tools".to_string(),
            Screen::MusicDownloader => "Music Downloader - Music Tools".to_string(),
            Screen::AudioConverter => "Audio Converter - Music Tools".to_string(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NavigateTo(screen) => {
                // Stop any playing audio when navigating away
                if self.state.current_screen == Screen::MusicDownloader
                    && screen != Screen::MusicDownloader
                {
                    audio_player::stop_audio();
                    self.state.downloader_state.playing_song_index = None;
                    self.state.downloader_state.streaming_url = None;
                    // Clear download logs when leaving downloader
                    self.state.error_logs.clear();
                }
                // Clear error logs when entering metadata editor (they're for metadata processing only)
                if screen == Screen::MetadataEditor {
                    self.state.error_logs.clear();
                }
                self.state.current_screen = screen;
                if screen == Screen::MusicDownloader
                    && self.state.downloader_state.all_artists.is_empty()
                    && !self.state.downloader_state.loading_artists
                {
                    self.state.downloader_state.loading_artists = true;
                    self.state.downloader_state.status =
                        "Loading artists from songhub.lk...".to_string();
                    return Command::perform(
                        async {
                            let scraper = SongHubScraper::new()?;
                            scraper.get_artists_by_letter(None).await
                        },
                        |result| Message::ArtistsLoaded(result.map_err(|e| e.to_string())),
                    );
                }
                Command::none()
            }
            Message::GoHome => {
                // Stop any playing audio when going home
                if self.state.current_screen == Screen::MusicDownloader {
                    audio_player::stop_audio();
                    self.state.downloader_state.playing_song_index = None;
                    self.state.downloader_state.streaming_url = None;
                    // Clear download logs when leaving downloader
                    self.state.error_logs.clear();
                }
                // Clear error logs when going home
                self.state.error_logs.clear();
                self.state.current_screen = Screen::Home;
                Command::none()
            }

            Message::SelectFiles => {
                if let Some(selected_files) = select_files() {
                    self.state.loading_files = true;
                    self.state.status = "Loading files...".to_string();
                    Command::perform(async move { selected_files }, Message::FilesSelected)
                } else {
                    Command::none()
                }
            }
            Message::SelectFolder => {
                let saved_path = self.state.last_metadata_folder.clone();
                Command::perform(
                    async move {
                        let mut dialog = rfd::AsyncFileDialog::new();
                        if let Some(path) = saved_path {
                            dialog = dialog.set_directory(&path);
                        }
                        dialog
                            .pick_folder()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    Message::FolderSelected,
                )
            }
            Message::FolderSelected(folder_path) => {
                if let Some(path) = &folder_path {
                    self.state.loading_files = true;
                    self.state.pending_folder_scan = Some(path.clone());
                    self.state.status = "Scanning folder for audio files...".to_string();

                    // Save to settings
                    let mut settings = AppSettings::load();
                    settings.set_metadata_folder(Some(path));
                    let _ = settings.save();
                    self.state.last_metadata_folder = Some(path.clone());
                }
                Command::none()
            }
            Message::StartFolderScan => {
                if let Some(path) = self.state.pending_folder_scan.take() {
                    Command::perform(scan_folder_async(path), Message::FilesSelected)
                } else {
                    Command::none()
                }
            }
            Message::FilesSelected(paths) => {
                self.state.loading_files = false;
                self.state.files = paths;
                self.state.selected_file_index = None;
                self.state.file_metadata.clear();
                self.state.status = format!("{} file(s) loaded", self.state.files.len());
                Command::none()
            }
            Message::ArtistChanged(value) => {
                self.state.artist = value;
                Command::none()
            }
            Message::AlbumChanged(value) => {
                self.state.album = value;
                Command::none()
            }
            Message::GenreChanged(value) => {
                self.state.genre = value;
                Command::none()
            }
            Message::YearChanged(value) => {
                self.state.year = value;
                Command::none()
            }
            Message::SelectImage => {
                Command::perform(async { select_image_async().await }, Message::ImageSelected)
            }
            Message::ImageSelected(path) => {
                self.state.album_art_path = path;
                if self.state.album_art_path.is_some() {
                    self.state.status = "Album art selected".to_string();
                }
                Command::none()
            }
            Message::ProcessFiles => {
                if self.state.files.is_empty() {
                    self.state.status = "No files selected".to_string();
                    return Command::none();
                }
                if self.state.artist.trim().is_empty()
                    && self.state.album.trim().is_empty()
                    && self.state.genre.trim().is_empty()
                    && self.state.year.trim().is_empty()
                    && self.state.album_art_path.is_none()
                {
                    self.state.status = "Please fill in at least one metadata field".to_string();
                    return Command::none();
                }

                self.state.processing = true;
                self.state.error_logs.clear();
                self.state.status = format!("Processing {} files...", self.state.files.len());

                let files = self.state.files.clone();
                let artist = self.state.artist.clone();
                let album = self.state.album.clone();
                let genre = if self.state.genre.trim().is_empty() {
                    None
                } else {
                    Some(self.state.genre.clone())
                };
                let year = self.state.year.parse::<u32>().ok();
                let album_art = self.state.album_art_path.clone();

                Command::perform(
                    async move { process_files(files, artist, album, genre, year, album_art).await },
                    Message::ProcessingComplete,
                )
            }
            Message::ProcessingComplete(result) => {
                self.state.processing = false;
                match result {
                    Ok(errors) => {
                        if errors.is_empty() {
                            self.state.status = format!(
                                "✓ Successfully updated {} file(s)",
                                self.state.files.len()
                            );
                        } else {
                            self.state.status = format!("Completed with {} error(s)", errors.len());
                            self.state.error_logs = errors;
                        }
                    }
                    Err(e) => {
                        self.state.status = format!("Error: {}", e);
                        self.state.error_logs = vec![e];
                    }
                }
                self.state.file_metadata.clear();
                if let Some(idx) = self.state.selected_file_index {
                    let file_path = self.state.files[idx].clone();
                    return Command::perform(
                        async move { (idx, read_file_metadata(file_path)) },
                        |(idx, result)| Message::MetadataLoaded(idx, result),
                    );
                }
                Command::none()
            }
            Message::RemoveFile(index) => {
                if index < self.state.files.len() {
                    if let Some(selected) = self.state.selected_file_index {
                        if selected == index {
                            self.state.selected_file_index = None;
                        } else if selected > index {
                            self.state.selected_file_index = Some(selected - 1);
                        }
                    }
                    self.state.files.remove(index);
                    let old_metadata: Vec<_> = (0..self.state.files.len() + 1)
                        .filter_map(|i| {
                            if i < index {
                                self.state.file_metadata.get(&i).map(|m| (i, m.clone()))
                            } else if i > index {
                                self.state.file_metadata.get(&i).map(|m| (i - 1, m.clone()))
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.state.file_metadata.clear();
                    for (i, meta) in old_metadata {
                        self.state.file_metadata.insert(i, meta);
                    }
                    self.state.status = format!("{} file(s) loaded", self.state.files.len());
                }
                Command::none()
            }
            Message::ClearAllFiles => {
                self.state.files.clear();
                self.state.file_metadata.clear();
                self.state.selected_file_index = None;
                self.state.status = "All files cleared".to_string();
                Command::none()
            }
            Message::FileSelected(index) => {
                if index < self.state.files.len() {
                    self.state.selected_file_index = Some(index);
                    let file_path = self.state.files[index].clone();
                    Command::perform(
                        async move { (index, read_file_metadata(file_path)) },
                        |(idx, result)| Message::MetadataLoaded(idx, result),
                    )
                } else {
                    Command::none()
                }
            }
            Message::MetadataLoaded(index, result) => {
                match result {
                    Ok(metadata) => {
                        self.state.file_metadata.insert(index, metadata);
                    }
                    Err(e) => {
                        self.state.status = format!("Error reading metadata: {}", e);
                    }
                }
                Command::none()
            }

            Message::LoadArtists => {
                self.state.downloader_state.loading_artists = true;
                self.state.downloader_state.status =
                    "Loading artists from songhub.lk...".to_string();
                Command::perform(
                    async {
                        let scraper = SongHubScraper::new()?;
                        scraper.get_artists_by_letter(None).await
                    },
                    |result| Message::ArtistsLoaded(result.map_err(|e| e.to_string())),
                )
            }
            Message::LoadArtistsByLetter(letter) => {
                self.state.downloader_state.loading_artists = true;
                self.state.downloader_state.status = format!(
                    "Loading artists starting with '{}'...",
                    letter.to_uppercase().collect::<String>()
                );
                Command::perform(
                    async move {
                        let scraper = SongHubScraper::new()?;
                        scraper.get_artists_by_letter(Some(letter)).await
                    },
                    |result| Message::ArtistsLoaded(result.map_err(|e| e.to_string())),
                )
            }
            Message::ArtistsLoaded(result) => {
                self.state.downloader_state.loading_artists = false;
                match result {
                    Ok(artists) => {
                        if artists.is_empty() {
                            self.state.downloader_state.status = "Warning: No artists found on the page. The page structure may have changed.".to_string();
                        } else {
                            self.state.downloader_state.all_artists = artists.clone();
                            self.state.downloader_state.filtered_artists = artists;
                            self.state.downloader_state.status = format!(
                                "Loaded {} artists",
                                self.state.downloader_state.all_artists.len()
                            );
                        }
                    }
                    Err(e) => {
                        self.state.downloader_state.status = format!("Failed to load artists: {}. Please check your internet connection and try again.", e);
                    }
                }
                Command::none()
            }
            Message::DownloaderArtistSearchChanged(query) => {
                let trimmed = query.trim();
                let is_single_letter = trimmed.len() == 1;

                self.state.downloader_state.artist_search_query = query.clone();

                if is_single_letter {
                    let letter = trimmed.chars().next().unwrap();
                    if letter.is_alphabetic() {
                        self.state.downloader_state.loading_artists = true;
                        self.state.downloader_state.status = format!(
                            "Loading artists starting with '{}'...",
                            letter.to_uppercase().collect::<String>()
                        );

                        return Command::perform(
                            async move {
                                let scraper = SongHubScraper::new()?;
                                scraper.get_artists_by_letter(Some(letter)).await
                            },
                            |result| Message::ArtistsLoaded(result.map_err(|e| e.to_string())),
                        );
                    }
                }

                if !self.state.downloader_state.loading_artists {
                    self.state.downloader_state.filter_artists();
                }
                Command::none()
            }
            Message::FilterArtists => {
                self.state.downloader_state.filter_artists();
                Command::none()
            }
            Message::SelectArtist(index) => {
                // Stop any playing audio when going back to artists
                audio_player::stop_audio();
                self.state.downloader_state.playing_song_index = None;
                self.state.downloader_state.streaming_url = None;

                if index == usize::MAX {
                    self.state.downloader_state.selected_artist = None;
                    self.state.downloader_state.search_results.clear();
                    self.state.downloader_state.selected_songs.clear();
                    self.state.downloader_state.status = "Select an artist".to_string();
                } else if index < self.state.downloader_state.filtered_artists.len() {
                    let artist = self.state.downloader_state.filtered_artists[index].clone();
                    self.state.downloader_state.selected_artist = Some(artist.clone());
                    self.state.downloader_state.search_results.clear();
                    self.state.downloader_state.selected_songs.clear();
                    self.state.downloader_state.loading_songs = true;
                    self.state.downloader_state.status =
                        format!("Loading songs for {}...", artist.name);
                    return Command::perform(
                        async move {
                            let scraper = SongHubScraper::new()?;
                            scraper.get_artist_songs(&artist.slug).await
                        },
                        |result| Message::ArtistSongsLoaded(result.map_err(|e| e.to_string())),
                    );
                }
                Command::none()
            }
            Message::LoadArtistSongs => {
                if let Some(ref artist) = self.state.downloader_state.selected_artist {
                    let slug = artist.slug.clone();
                    Command::perform(
                        async move {
                            let scraper = SongHubScraper::new()?;
                            scraper.get_artist_songs(&slug).await
                        },
                        |result| Message::ArtistSongsLoaded(result.map_err(|e| e.to_string())),
                    )
                } else {
                    Command::none()
                }
            }
            Message::ArtistSongsLoaded(result) => {
                self.state.downloader_state.loading_songs = false;
                match result {
                    Ok(songs) => {
                        self.state.downloader_state.search_results = songs;
                        if let Some(ref artist) = self.state.downloader_state.selected_artist {
                            let status = format!(
                                "Found {} song(s) for {}",
                                self.state.downloader_state.search_results.len(),
                                artist.name
                            );
                            if let Some(ref path) = self.state.downloader_state.download_path {
                                self.state.downloader_state.status =
                                    format!("{}\nDownload to: {}", status, path.display());
                            } else {
                                self.state.downloader_state.status =
                                    format!("{}\nPlease select download directory", status);
                            }
                        }
                    }
                    Err(e) => {
                        self.state.downloader_state.status = format!("Failed to load songs: {}", e);
                    }
                }
                Command::none()
            }
            Message::ToggleSongSelection(index) => {
                if let Some(pos) = self
                    .state
                    .downloader_state
                    .selected_songs
                    .iter()
                    .position(|&i| i == index)
                {
                    self.state.downloader_state.selected_songs.remove(pos);
                } else {
                    self.state.downloader_state.selected_songs.push(index);
                }
                self.state.downloader_state.status = format!(
                    "{} song(s) selected",
                    self.state.downloader_state.selected_songs.len()
                );
                Command::none()
            }
            Message::SelectAllSongs => {
                self.state.downloader_state.selected_songs =
                    (0..self.state.downloader_state.search_results.len()).collect();
                self.state.downloader_state.status = format!(
                    "{} song(s) selected",
                    self.state.downloader_state.selected_songs.len()
                );
                Command::none()
            }
            Message::DeselectAllSongs => {
                self.state.downloader_state.selected_songs.clear();
                self.state.downloader_state.status = "All songs deselected".to_string();
                Command::none()
            }
            Message::PlaySong(index) => {
                if index >= self.state.downloader_state.search_results.len() {
                    return Command::none();
                }

                // Stop any currently playing song
                if self.state.downloader_state.playing_song_index.is_some() {
                    audio_player::stop_audio();
                    self.state.downloader_state.playing_song_index = None;
                    self.state.downloader_state.streaming_url = None;
                }

                let song = self.state.downloader_state.search_results[index].clone();
                let song_url = song.url.clone();

                self.state.downloader_state.playing_song_index = Some(index);
                self.state.downloader_state.status = format!("Loading: {}...", song.title);

                Command::perform(
                    async move {
                        let scraper = SongHubScraper::new()?;
                        scraper.get_streaming_url(&song_url).await
                    },
                    move |result| {
                        Message::StreamingUrlLoaded(index, result.map_err(|e| e.to_string()))
                    },
                )
            }
            Message::StreamingUrlLoaded(index, result) => {
                match result {
                    Ok(Some(url)) => {
                        // Ensure the playing index is set correctly
                        self.state.downloader_state.playing_song_index = Some(index);
                        self.state.downloader_state.streaming_url = Some(url.clone());
                        if let Some(ref song) =
                            self.state.downloader_state.search_results.get(index)
                        {
                            self.state.downloader_state.status =
                                format!("Playing: {}...", song.title);
                        }

                        // Start audio playback (non-blocking - allows app to close)
                        // Spawn directly without waiting
                        audio_player::play_streaming_url_async(url);
                        Command::none()
                    }
                    Ok(None) => {
                        self.state.downloader_state.playing_song_index = None;
                        self.state.downloader_state.status = "No streaming URL found".to_string();
                        Command::none()
                    }
                    Err(e) => {
                        self.state.downloader_state.playing_song_index = None;
                        self.state.downloader_state.status = format!("Failed to load audio: {}", e);
                        Command::none()
                    }
                }
            }
            Message::StopSong => {
                audio_player::stop_audio();
                self.state.downloader_state.playing_song_index = None;
                self.state.downloader_state.streaming_url = None;
                if let Some(ref artist) = self.state.downloader_state.selected_artist {
                    self.state.downloader_state.status = format!(
                        "Found {} song(s) for {}",
                        self.state.downloader_state.search_results.len(),
                        artist.name
                    );
                }
                Command::none()
            }
            Message::SelectDownloadDirectory => {
                let mut dialog = rfd::FileDialog::new();
                if let Some(ref saved_path) = self.state.downloader_state.download_path {
                    dialog = dialog.set_directory(saved_path);
                }
                if let Some(path) = dialog.pick_folder() {
                    Command::perform(
                        async move { Some(path) },
                        Message::DownloadDirectorySelected,
                    )
                } else {
                    Command::none()
                }
            }
            Message::DownloadDirectorySelected(path) => {
                self.state.downloader_state.download_path = path.clone();
                if let Some(ref p) = path {
                    if let Some(ref artist) = self.state.downloader_state.selected_artist {
                        self.state.downloader_state.status = format!(
                            "Found {} song(s) for {}\nDownload to: {}",
                            self.state.downloader_state.search_results.len(),
                            artist.name,
                            p.display()
                        );
                    } else {
                        self.state.downloader_state.status =
                            format!("Download to: {}", p.display());
                    }

                    // Save to settings
                    let mut settings = AppSettings::load();
                    settings.set_download_directory(Some(p));
                    let _ = settings.save();
                }
                Command::none()
            }
            Message::DownloadSelectedSongs => {
                if self.state.downloader_state.selected_songs.is_empty() {
                    self.state.downloader_state.status = "No songs selected".to_string();
                    return Command::none();
                }
                if self.state.downloader_state.download_path.is_none() {
                    self.state.downloader_state.status =
                        "Please select download directory".to_string();
                    return Command::none();
                }

                let songs = self.state.downloader_state.search_results.clone();
                let selected_indices = self.state.downloader_state.selected_songs.clone();
                let download_path = self.state.downloader_state.download_path.clone().unwrap();

                self.state.downloader_state.downloading = true;
                self.state.downloader_state.status = "Starting download...".to_string();

                Command::perform(
                    async move {
                        use futures::stream::{FuturesUnordered, StreamExt};
                        use std::sync::Arc;
                        use tokio::sync::Semaphore;

                        let scraper = SongHubScraper::new()?;
                        let mut errors = Vec::new();
                        let mut success_logs = Vec::new();

                        // Limit concurrent downloads to 5
                        let semaphore = Arc::new(Semaphore::new(5));
                        let mut download_tasks = FuturesUnordered::new();

                        for &song_idx in selected_indices.iter() {
                            if song_idx >= songs.len() {
                                continue;
                            }

                            let song = songs[song_idx].clone();
                            let scraper_clone = scraper.clone();
                            let semaphore_clone = semaphore.clone();
                            let download_path_clone = download_path.clone();

                            // Spawn download task with semaphore limit
                            let task = async move {
                                let _permit = semaphore_clone.acquire().await.unwrap();

                                let download_url = match scraper_clone
                                    .get_download_url(&song.url)
                                    .await
                                {
                                    Ok(Some(url)) => url,
                                    Ok(None) => {
                                        return Err(format!("No download URL for: {}", song.title));
                                    }
                                    Err(e) => {
                                        return Err(format!(
                                            "Failed to get URL for {}: {}",
                                            song.title, e
                                        ));
                                    }
                                };

                                let safe_title = song
                                    .title
                                    .chars()
                                    .map(|c| match c {
                                        '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
                                        _ => c,
                                    })
                                    .collect::<String>()
                                    .trim()
                                    .to_string();
                                let filename = format!("{}.mp3", safe_title);
                                let output_path = download_path_clone.join(&filename);

                                match scraper_clone
                                    .download_song(&download_url, &output_path)
                                    .await
                                {
                                    Ok(_) => Ok(format!("✓ Downloaded: {}", song.title)),
                                    Err(e) => {
                                        Err(format!("Failed to download {}: {}", song.title, e))
                                    }
                                }
                            };

                            download_tasks.push(task);
                        }

                        // Process all downloads concurrently (max 5 at a time)
                        while let Some(result) = download_tasks.next().await {
                            match result {
                                Ok(log) => success_logs.push(log),
                                Err(error) => errors.push(error),
                            }
                        }

                        Ok((success_logs, errors))
                    },
                    |result: Result<(Vec<String>, Vec<String>), anyhow::Error>| {
                        Message::DownloadComplete(result.map_err(|e| e.to_string()))
                    },
                )
            }
            Message::DownloadComplete(result) => {
                self.state.downloader_state.downloading = false;
                match result {
                    Ok((success_logs, errors)) => {
                        let total = self.state.downloader_state.selected_songs.len();
                        let success_count = success_logs.len();
                        let error_count = errors.len();

                        if errors.is_empty() {
                            self.state.downloader_state.status =
                                format!("Successfully downloaded {} song(s)", success_count);
                        } else {
                            self.state.downloader_state.status = format!(
                                "Downloaded {} of {} song(s) ({} error(s))",
                                success_count, total, error_count
                            );
                        }

                        // Clear individual logs - only show summary
                        self.state.error_logs.clear();
                    }
                    Err(e) => {
                        self.state.downloader_state.status = format!("Download failed: {}", e);
                        self.state.error_logs = vec![format!("Download failed: {}", e)];
                    }
                }
                Command::none()
            }

            Message::SelectConvertFiles => {
                self.state.convert_status = "This feature is coming soon!".to_string();
                Command::none()
            }
            Message::ConvertFormatChanged(format) => {
                self.state.convert_format = format;
                Command::none()
            }
            Message::StartConvert => {
                self.state.convert_status = "This feature is coming soon!".to_string();
                Command::none()
            }

            Message::ToggleTheme => {
                self.theme_mode = match self.theme_mode {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                };
                Command::none()
            }

            Message::Tick(_) => {
                if self.state.loading_files
                    || self.state.processing
                    || self.state.downloader_state.loading_artists
                    || self.state.downloader_state.loading_songs
                {
                    self.state.loading_rotation += 0.15;
                    if self.state.loading_rotation >= std::f32::consts::TAU {
                        self.state.loading_rotation = 0.0;
                    }
                }
                if self.state.pending_folder_scan.is_some() {
                    self.state.scan_delay_ticks += 1;
                    if self.state.scan_delay_ticks >= 5 {
                        self.state.scan_delay_ticks = 0;
                        return Command::perform(async {}, |_| Message::StartFolderScan);
                    }
                }
                Command::none()
            }
            Message::NoOp => Command::none(),
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        build_view(&self.state, self.theme_mode)
    }

    fn theme(&self) -> Theme {
        cosmic_theme(self.theme_mode)
    }

    fn subscription(&self) -> Subscription<Message> {
        let active = self.state.loading_files
            || self.state.processing
            || self.state.downloader_state.loading_artists
            || self.state.downloader_state.loading_songs;
        if active {
            time::every(Duration::from_millis(16)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }
}
