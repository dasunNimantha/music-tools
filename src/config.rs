pub const SUPPORTED_FORMATS: &[&str] = &[
    "mp3", "flac", "m4a", "ogg", "wma", "aac", "mp4", "opus", "wav", "aiff",
];

pub mod window {
    use iced::Size;

    pub const DEFAULT_WIDTH: f32 = 900.0;
    pub const DEFAULT_HEIGHT: f32 = 580.0;
    pub const MIN_WIDTH: f32 = 800.0;
    pub const MIN_HEIGHT: f32 = 500.0;

    pub fn default_size() -> Size {
        Size::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)
    }

    pub fn min_size() -> Size {
        Size::new(MIN_WIDTH, MIN_HEIGHT)
    }
}
