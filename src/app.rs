use crate::file_dialog::{scan_folder_async, select_files, select_folder_dialog, select_image};
use crate::message::Message;
use crate::metadata::{process_files, read_file_metadata};
use crate::model::{AppState, Screen};
use crate::theme::{cosmic_theme, ThemeMode};
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
        (
            Self {
                state: AppState::default(),
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
            // Navigation
            Message::NavigateTo(screen) => {
                self.state.current_screen = screen;
                Command::none()
            }
            Message::GoHome => {
                self.state.current_screen = Screen::Home;
                Command::none()
            }

            // Metadata Editor
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
                Command::perform(select_folder_dialog(), Message::FolderSelected)
            }
            Message::FolderSelected(folder_path) => {
                if let Some(path) = folder_path {
                    self.state.loading_files = true;
                    self.state.pending_folder_scan = Some(path);
                    self.state.status = "Scanning folder for audio files...".to_string();
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
                if let Some(image_path) = select_image() {
                    Command::perform(async move { Some(image_path) }, Message::ImageSelected)
                } else {
                    Command::none()
                }
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

            // Music Downloader (placeholder)
            Message::DownloadUrlChanged(url) => {
                self.state.download_url = url;
                Command::none()
            }
            Message::StartDownload => {
                self.state.download_status = "This feature is coming soon!".to_string();
                Command::none()
            }

            // Audio Converter (placeholder)
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

            // Theme
            Message::ToggleTheme => {
                self.theme_mode = match self.theme_mode {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                };
                Command::none()
            }

            // Animation
            Message::Tick(_) => {
                if self.state.loading_files || self.state.processing {
                    self.state.loading_rotation += 0.15;
                    if self.state.loading_rotation >= std::f32::consts::TAU {
                        self.state.loading_rotation = 0.0;
                    }
                }
                // If there's a pending folder scan, wait a few ticks then trigger
                if self.state.pending_folder_scan.is_some() {
                    self.state.scan_delay_ticks += 1;
                    if self.state.scan_delay_ticks >= 5 {
                        self.state.scan_delay_ticks = 0;
                        return Command::perform(async {}, |_| Message::StartFolderScan);
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        build_view(&self.state, self.theme_mode)
    }

    fn theme(&self) -> Theme {
        cosmic_theme(self.theme_mode)
    }

    fn subscription(&self) -> Subscription<Message> {
        let active = self.state.loading_files || self.state.processing;
        if active {
            time::every(Duration::from_millis(16)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }
}
