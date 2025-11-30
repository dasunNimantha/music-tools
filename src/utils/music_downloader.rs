use crate::utils::scraper::{Artist, Song, SongHubScraper};
use anyhow::Result;
use std::path::PathBuf;
#[derive(Debug, Clone)]
pub struct MusicDownloaderState {
    pub artist_search_query: String,
    pub all_artists: Vec<Artist>,
    pub filtered_artists: Vec<Artist>,
    pub selected_artist: Option<Artist>,
    pub status: String,
    pub loading_artists: bool,
    pub loading_songs: bool,
    pub downloading: bool,
    pub progress: f32,
    pub search_results: Vec<Song>,
    pub selected_songs: Vec<usize>,
    pub download_path: Option<PathBuf>,
}

impl Default for MusicDownloaderState {
    fn default() -> Self {
        Self {
            artist_search_query: String::new(),
            all_artists: Vec::new(),
            filtered_artists: Vec::new(),
            selected_artist: None,
            status: "Loading artists...".to_string(),
            loading_artists: false,
            loading_songs: false,
            downloading: false,
            progress: 0.0,
            search_results: Vec::new(),
            selected_songs: Vec::new(),
            download_path: None,
        }
    }
}

impl MusicDownloaderState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load artists from songhub.lk, optionally filtered by letter
    pub async fn load_artists(&mut self, letter: Option<char>) -> Result<()> {
        self.loading_artists = true;
        self.status = if let Some(l) = letter {
            format!(
                "Loading artists starting with '{}'...",
                l.to_uppercase().collect::<String>()
            )
        } else {
            "Loading artists...".to_string()
        };
        let scraper = SongHubScraper::new()?;

        let artists = scraper.get_artists_by_letter(letter).await.map_err(|e| {
            self.status = format!("Failed to load artists: {}", e);
            self.loading_artists = false;
            e
        })?;

        self.all_artists = artists.clone();
        self.filtered_artists = artists;

        self.loading_artists = false;
        self.status = format!("Loaded {} artists", self.filtered_artists.len());

        Ok(())
    }

    pub fn filter_artists(&mut self) {
        let query = self.artist_search_query.trim();

        if query.len() == 1 {
            let letter = query.chars().next().unwrap();
            if letter.is_alphabetic() {
                self.filtered_artists = self.all_artists.clone();
                self.status = format!(
                    "Type to search or press Enter to load artists for '{}'",
                    letter.to_uppercase().collect::<String>()
                );
                return;
            }
        }
        if query.is_empty() {
            self.filtered_artists = self.all_artists.clone();
        } else {
            let query_lower = query.to_lowercase();
            self.filtered_artists = self
                .all_artists
                .iter()
                .filter(|artist| artist.name.to_lowercase().contains(&query_lower))
                .cloned()
                .collect();
        }
        self.status = format!("{} artist(s) found", self.filtered_artists.len());
    }

    /// Get all songs for the selected artist
    pub async fn load_artist_songs(&mut self) -> Result<()> {
        let artist = match &self.selected_artist {
            Some(a) => a.clone(),
            None => {
                self.status = "No artist selected".to_string();
                return Ok(());
            }
        };

        self.loading_songs = true;
        self.status = format!("Loading songs for {}...", artist.name);
        let scraper = SongHubScraper::new()?;

        let songs = scraper.get_artist_songs(&artist.slug).await.map_err(|e| {
            self.loading_songs = false;
            self.status = format!("Failed to load songs: {}", e);
            e
        })?;

        self.search_results = songs;
        self.selected_songs.clear();
        self.loading_songs = false;
        self.status = format!(
            "Found {} song(s) for {}",
            self.search_results.len(),
            artist.name
        );

        Ok(())
    }

    /// Download selected songs
    pub async fn download_selected(&mut self) -> Result<Vec<String>> {
        if self.selected_songs.is_empty() {
            self.status = "No songs selected".to_string();
            return Ok(Vec::new());
        }

        if self.download_path.is_none() {
            self.status = "Please select download directory".to_string();
            return Ok(Vec::new());
        }

        let download_dir = self.download_path.as_ref().unwrap();
        let scraper = SongHubScraper::new()?;
        let mut errors = Vec::new();
        let total = self.selected_songs.len();

        self.downloading = true;
        self.progress = 0.0;

        for (idx, &song_idx) in self.selected_songs.iter().enumerate() {
            if song_idx >= self.search_results.len() {
                continue;
            }

            let song = &self.search_results[song_idx];
            self.status = format!("Downloading: {}...", song.title);
            self.progress = (idx as f32 / total as f32) * 100.0;

            let download_url = match scraper.get_download_url(&song.url).await {
                Ok(Some(url)) => url,
                Ok(None) => {
                    errors.push(format!("No download URL found for: {}", song.title));
                    continue;
                }
                Err(e) => {
                    errors.push(format!(
                        "Failed to get download URL for {}: {}",
                        song.title, e
                    ));
                    continue;
                }
            };

            let safe_title = sanitize_filename(&song.title);
            let filename = format!("{}.mp3", safe_title);
            let output_path = download_dir.join(&filename);

            match scraper.download_song(&download_url, &output_path).await {
                Ok(_) => {}
                Err(e) => {
                    errors.push(format!("Failed to download {}: {}", song.title, e));
                }
            }
        }

        self.downloading = false;
        self.progress = 100.0;

        if errors.is_empty() {
            self.status = format!("Successfully downloaded {} song(s)", total);
        } else {
            self.status = format!("Downloaded with {} error(s)", errors.len());
        }

        Ok(errors)
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}
