use anyhow::{Context, Result};
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time;
use url::Url;

const BASE_URL: &str = "https://songhub.lk";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
const MAX_CONCURRENT_REQUESTS: usize = 8;
const PAGE_FETCH_TIMEOUT: Duration = Duration::from_secs(10);
const RATE_LIMIT_DELAY: Duration = Duration::from_millis(100);

#[derive(Debug, Clone)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub url: String,
    pub download_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Artist {
    pub name: String,
    pub slug: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct SearchResults {
    pub songs: Vec<Song>,
    pub artists: Vec<String>,
}

#[derive(Clone)]
pub struct SongHubScraper {
    client: reqwest::Client,
}

impl SongHubScraper {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client })
    }

    pub async fn search_by_artist(&self, artist_name: &str) -> Result<SearchResults> {
        tokio::time::sleep(Duration::from_millis(300)).await;

        let search_url = format!("{}/search?q={}", BASE_URL, urlencoding::encode(artist_name));
        let url = Url::parse(&search_url).context("Invalid search URL")?;

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to fetch search page")?;

        if !response.status().is_success() {
            anyhow::bail!("Search request failed with status: {}", response.status());
        }

        let html = response
            .text()
            .await
            .context("Failed to read response body")?;
        let document = Html::parse_document(&html);

        let mut songs = Vec::new();
        let mut artists = Vec::new();

        let song_selector = Selector::parse("a[href*='/song/'], a[href*='/mp3/']")
            .unwrap_or_else(|_| Selector::parse("a").unwrap());

        for element in document.select(&song_selector) {
            if let Some(href) = element.value().attr("href") {
                let full_url = if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("{}{}", BASE_URL, href)
                };

                let title = element.text().collect::<String>().trim().to_string();

                if !title.is_empty() && (href.contains("/song/") || href.contains("/mp3/")) {
                    songs.push(Song {
                        title: title.clone(),
                        artist: artist_name.to_string(),
                        url: full_url,
                        download_url: None,
                    });
                }
            }
        }

        let artist_selector = Selector::parse("a[href*='/artist/']")
            .unwrap_or_else(|_| Selector::parse("a").unwrap());

        for element in document.select(&artist_selector) {
            if let Some(text) = element.text().next() {
                let artist = text.trim().to_string();
                if !artist.is_empty() && !artists.contains(&artist) {
                    artists.push(artist);
                }
            }
        }

        Ok(SearchResults { songs, artists })
    }

    async fn fetch_and_parse_page(
        client: &reqwest::Client,
        url: Url,
        semaphore: Arc<Semaphore>,
    ) -> Result<Vec<Artist>> {
        let _permit = semaphore.acquire().await.unwrap();
        tokio::time::sleep(RATE_LIMIT_DELAY).await;

        let response = match time::timeout(PAGE_FETCH_TIMEOUT, client.get(url.clone()).send()).await
        {
            Ok(Ok(r)) => r,
            Ok(Err(e)) => return Err(anyhow::anyhow!("Failed to fetch page: {}", e)),
            Err(_) => return Err(anyhow::anyhow!("Request timeout")),
        };

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Page request failed with status: {}",
                response.status()
            ));
        }

        let html = response
            .text()
            .await
            .context("Failed to read response body")?;
        let document = Html::parse_document(&html);
        Self::parse_artists_from_html(&document)
    }

    fn parse_artists_from_html(document: &Html) -> Result<Vec<Artist>> {
        let link_selector = Selector::parse("a")
            .map_err(|e| anyhow::anyhow!("Failed to parse link selector: {:?}", e))?;

        let mut artists = Vec::new();

        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                let normalized_href = if href.starts_with("http") {
                    if let Ok(url) = Url::parse(href) {
                        url.path().to_string()
                    } else {
                        continue;
                    }
                } else if href.starts_with("/artist/") {
                    href.to_string()
                } else if href.starts_with("artist/") {
                    format!("/{}", href)
                } else {
                    continue;
                };

                if !normalized_href.starts_with("/artist/") {
                    continue;
                }

                if normalized_href == "/artist" || normalized_href == "/artist/" {
                    continue;
                }

                let slug = normalized_href
                    .strip_prefix("/artist/")
                    .unwrap_or("")
                    .trim();

                if slug.is_empty() || slug.contains("/artist/") || slug == "artist" {
                    continue;
                }

                let mut name = element.text().collect::<String>().trim().to_string();
                name = name
                    .replace(" mp3 songs", "")
                    .replace(" mp3 song", "")
                    .replace(" mp3", "")
                    .trim()
                    .to_string();

                let parts: Vec<&str> = name.split_whitespace().collect();
                if parts.len() > 2 {
                    let mid = parts.len() / 2;
                    let first_half = parts[..mid].join(" ");
                    let second_half = parts[mid..].join(" ");

                    if first_half.to_lowercase() == second_half.to_lowercase() {
                        name = first_half;
                    }
                }

                if name.is_empty() || name.len() < 2 {
                    name = slug
                        .split('-')
                        .map(|s| {
                            let mut chars = s.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => {
                                    first.to_uppercase().collect::<String>() + chars.as_str()
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");
                }

                if !name.is_empty() && !slug.is_empty() {
                    let full_url = format!("{}/artist/{}", BASE_URL, slug);
                    artists.push(Artist {
                        name,
                        slug: slug.to_string(),
                        url: full_url,
                    });
                }
            }
        }

        Ok(artists)
    }

    pub async fn get_artists_by_letter(&self, letter: Option<char>) -> Result<Vec<Artist>> {
        let load_all_pages = letter.is_some();
        let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

        let first_page_url = if let Some(l) = letter {
            let letter_lower = l.to_ascii_lowercase();
            format!("{}/artist?letter={}", BASE_URL, letter_lower)
        } else {
            format!("{}/artist", BASE_URL)
        };

        let url = Url::parse(&first_page_url).context("Invalid artists URL")?;
        let mut first_page_artists =
            Self::fetch_and_parse_page(&self.client, url, semaphore.clone())
                .await
                .context("Failed to fetch first page")?;

        if !load_all_pages {
            first_page_artists.sort_by(|a, b| a.name.cmp(&b.name));
            return Ok(first_page_artists);
        }

        let mut all_artists = first_page_artists;
        let mut seen_slugs: HashSet<String> = all_artists.iter().map(|a| a.slug.clone()).collect();
        let mut page = 2;
        let max_pages = 50;
        let batch_size = 5;

        while page <= max_pages {
            let mut batch_tasks = Vec::new();
            let letter_lower = letter.unwrap().to_ascii_lowercase();

            for i in 0..batch_size {
                let page_num = page + i;
                if page_num > max_pages {
                    break;
                }

                let url_str = format!(
                    "{}/artist?letter={}&page={}",
                    BASE_URL, letter_lower, page_num
                );
                if let Ok(url) = Url::parse(&url_str) {
                    let client = self.client.clone();
                    let sem = semaphore.clone();
                    batch_tasks.push(tokio::spawn(async move {
                        Self::fetch_and_parse_page(&client, url, sem).await
                    }));
                }
            }

            let results = futures::future::join_all(batch_tasks).await;
            let mut found_any = false;

            for result in results {
                match result {
                    Ok(Ok(page_artists)) if !page_artists.is_empty() => {
                        for artist in page_artists {
                            if seen_slugs.insert(artist.slug.clone()) {
                                all_artists.push(artist);
                                found_any = true;
                            }
                        }
                    }
                    _ => {}
                }
            }

            if !found_any {
                break;
            }

            page += batch_size;
        }

        all_artists.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(all_artists)
    }

    pub async fn get_all_artists(&self) -> Result<Vec<Artist>> {
        self.get_artists_by_letter(None).await
    }

    pub async fn get_artist_songs(&self, artist_slug: &str) -> Result<Vec<Song>> {
        tokio::time::sleep(Duration::from_millis(300)).await;

        let artist_url = format!("{}/artist/{}", BASE_URL, artist_slug);
        let url = Url::parse(&artist_url).context("Invalid artist URL")?;

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to fetch artist page")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Artist page request failed with status: {}",
                response.status()
            );
        }

        let html = response
            .text()
            .await
            .context("Failed to read response body")?;
        let document = Html::parse_document(&html);

        let artist_name = document
            .select(&Selector::parse("h1").unwrap_or_else(|_| Selector::parse("title").unwrap()))
            .next()
            .and_then(|e| {
                let text = e.text().collect::<String>();
                text.strip_suffix(" Songs")
                    .or_else(|| text.strip_suffix(" mp3 songs"))
                    .map(|s| s.trim().to_string())
            })
            .unwrap_or_else(|| artist_slug.replace('-', " "));

        let mut songs = Vec::new();
        let song_selector = Selector::parse("a[href*='/song/'], a[href*='/mp3/']")
            .unwrap_or_else(|_| Selector::parse("a").unwrap());

        for element in document.select(&song_selector) {
            if let Some(href) = element.value().attr("href") {
                if href.contains("/artist/") {
                    continue;
                }

                let full_url = if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("{}{}", BASE_URL, href)
                };

                let title = element
                    .text()
                    .collect::<String>()
                    .trim()
                    .replace(" mp3 song", "")
                    .replace(" mp3 songs", "")
                    .trim()
                    .to_string();

                if !title.is_empty() && (href.contains("/song/") || href.contains("/mp3/")) {
                    songs.push(Song {
                        title,
                        artist: artist_name.clone(),
                        url: full_url,
                        download_url: None,
                    });
                }
            }
        }

        // Sort songs by title in ascending order (case-insensitive)
        songs.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

        Ok(songs)
    }

    pub async fn get_streaming_url(&self, song_url: &str) -> Result<Option<String>> {
        self.get_download_url(song_url).await
    }

    pub async fn get_download_url(&self, song_url: &str) -> Result<Option<String>> {
        tokio::time::sleep(Duration::from_millis(300)).await;

        let url = Url::parse(song_url).context("Invalid song URL")?;

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to fetch song page")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Song page request failed with status: {}",
                response.status()
            );
        }

        let html = response
            .text()
            .await
            .context("Failed to read response body")?;

        let mp3_regex = Regex::new(r#"https?://[^"'\s]+\.mp3"#)?;
        if let Some(captures) = mp3_regex.find(&html) {
            return Ok(Some(captures.as_str().to_string()));
        }

        let document = Html::parse_document(&html);
        let download_selector = Selector::parse(
            "a[href*='download'], a[href*='.mp3'], button[data-url], a[data-download]",
        )
        .unwrap_or_else(|_| Selector::parse("a").unwrap());

        for element in document.select(&download_selector) {
            if let Some(href) = element.value().attr("href") {
                if href.contains(".mp3") || href.contains("download") {
                    let full_url = if href.starts_with("http") {
                        href.to_string()
                    } else {
                        format!("{}{}", BASE_URL, href)
                    };
                    return Ok(Some(full_url));
                }
            }

            if let Some(data_url) = element.value().attr("data-url") {
                if data_url.contains(".mp3") {
                    let full_url = if data_url.starts_with("http") {
                        data_url.to_string()
                    } else {
                        format!("{}{}", BASE_URL, data_url)
                    };
                    return Ok(Some(full_url));
                }
            }
        }

        Ok(None)
    }

    pub async fn download_song(
        &self,
        download_url: &str,
        output_path: &std::path::Path,
    ) -> Result<()> {
        let url = Url::parse(download_url).context("Invalid download URL")?;

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to download file")?;

        if !response.status().is_success() {
            anyhow::bail!("Download failed with status: {}", response.status());
        }

        let bytes = response
            .bytes()
            .await
            .context("Failed to read download response")?;

        tokio::fs::write(output_path, bytes)
            .await
            .context("Failed to write file")?;

        Ok(())
    }
}

impl Default for SongHubScraper {
    fn default() -> Self {
        Self::new().expect("Failed to create scraper")
    }
}
